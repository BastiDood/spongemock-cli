use rand::Rng;
use std::io::{ErrorKind, Result, Write};

pub fn spongemock<Source, Rand, Writer>(
    mut src: Source,
    dest: &mut Writer,
    rng: &mut Rand,
) -> Result<()>
where
    Source: Iterator<Item = Result<u8>>,
    Rand: Rng,
    Writer: Write,
{
    let iter = src.by_ref();
    while let Some(mut byte) = iter.next().transpose()? {
        // Check if without leading one
        if byte < 0b_1000_0000 {
            if rng.gen() {
                byte.make_ascii_uppercase();
            } else {
                byte.make_ascii_lowercase();
            }

            dest.write_all(&[byte])?;
            continue;
        }

        // Otherwise, we expect full UTF-8 encoding
        let leading_ones = byte.leading_ones() as usize;
        if !(2..=4).contains(&leading_ones) {
            return Err(ErrorKind::InvalidData.into());
        }

        // Fill out the rest of the UTF-8 `char` buffer
        let mut char_be = [byte, 0, 0, 0];
        for curr in char_be[1..leading_ones].iter_mut() {
            let byte = iter.next().ok_or(ErrorKind::InvalidData)??;
            if byte.leading_ones() != 1 {
                return Err(ErrorKind::InvalidData.into());
            }

            *curr = byte;
        }

        // SAFETY: We have verified `char_be` to be valid UTF-8 up to the `leading_ones` (exclusive).
        let text = unsafe { core::str::from_utf8(&char_be[..leading_ones]).unwrap_unchecked() };
        for ch in text.chars() {
            let mut buf = [0; 4];
            if rng.gen() {
                for c in ch.to_uppercase() {
                    let bytes = c.encode_utf8(&mut buf).as_bytes();
                    dest.write_all(bytes)?;
                }
            } else {
                for c in ch.to_lowercase() {
                    let bytes = c.encode_utf8(&mut buf).as_bytes();
                    dest.write_all(bytes)?;
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::spongemock;
    use rand::rngs::mock::StepRng;

    /// Since the [`StepRng`](StepRng) mock RNG samples Booleans by
    /// using the most significant bit of the `u32` returned from the
    /// [`RngCore::next_u32`](rand::RngCore::next_u32) method, we set
    /// the increment as defined below.
    ///
    /// This specific integer allows us to toggle the most
    /// significant bit after every incrementation. We simply
    /// exploit the fact that `StepRng` performs wrapping addition.
    const INCREMENT: u64 = 0x_0000_0000_8000_0000;

    #[test]
    fn ascii_only() {
        let bytes = "Hello".bytes().map(Ok);
        let mut dest = Vec::new();
        let mut rng = StepRng::new(0, INCREMENT);
        spongemock(bytes, &mut dest, &mut rng).unwrap();
        assert_eq!(&dest, "hElLo".as_bytes());
    }

    #[test]
    fn with_greek_characters() {
        let bytes = "Ηελλο".bytes().map(Ok);
        let mut dest = Vec::new();
        let mut rng = StepRng::new(0, INCREMENT);
        spongemock(bytes, &mut dest, &mut rng).unwrap();
        assert_eq!(&dest, "ηΕλΛο".as_bytes());
    }

    #[test]
    fn with_mixed_greek_and_ascii() {
        let bytes = "Ηελλο World".bytes().map(Ok);
        let mut dest = Vec::new();
        let mut rng = StepRng::new(0, INCREMENT);
        spongemock(bytes, &mut dest, &mut rng).unwrap();
        assert_eq!(&dest, "ηΕλΛο wOrLd".as_bytes());
    }
}

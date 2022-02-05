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

        // We have verified `char_be` to be valid UTF-8 up to the `index` (inclusive).
        let text = core::str::from_utf8(&char_be[..leading_ones]).unwrap();
        for ch in text.chars() {
            let mut buf = [0; 4];
            if rng.gen_bool(0.5) {
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

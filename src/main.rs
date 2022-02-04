use rand::{rngs::SmallRng, Rng, SeedableRng};
use std::io::{stdin, stdout, ErrorKind, Read, Result, Write};

fn main() -> Result<()> {
    let mut bytes = stdin().bytes();
    let mut output = stdout();
    let mut rng = SmallRng::from_entropy();

    let iter = bytes.by_ref();
    while let Some(byte) = iter.next().transpose()? {
        // Check if without leading one
        if byte < 0b_1000_0000 {
            output.write_all(&[byte])?;
            continue;
        }

        // Otherwise, we expect full UTF-8 encoding
        let leading_ones = byte.leading_ones();
        if !(2..=4).contains(&leading_ones) {
            return Err(ErrorKind::InvalidData.into());
        }

        // Fill out the rest of the UTF-8 `char` buffer
        let mut char_be = [byte, 0, 0, 0];
        for curr in char_be[1..].iter_mut() {
            let byte = iter.next().ok_or(ErrorKind::InvalidData)??;
            if curr.leading_ones() != 1 {
                return Err(ErrorKind::InvalidData.into());
            }
            *curr = byte;
        }

        // SAFETY: We have verified `char_be` to be valid UTF-8.
        let word = u32::from_be_bytes(char_be);
        let ch = unsafe { char::from_u32_unchecked(word) };

        // Finally randomize the Spongemock
        if rng.gen_bool(0.5) {
            for ch in ch.to_uppercase() {
                let slice = ch.encode_utf8(&mut char_be).as_bytes();
                output.write_all(slice)?;
            }
        } else {
            for ch in ch.to_lowercase() {
                let slice = ch.encode_utf8(&mut char_be).as_bytes();
                output.write_all(slice)?;
            }
        }
    }

    Ok(())
}

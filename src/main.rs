use rand::{rngs::SmallRng, Rng, SeedableRng};
use std::io::{stdin, stdout, ErrorKind, Read, Result, Write};

fn main() -> Result<()> {
    let mut bytes = stdin().bytes();
    let mut output = stdout();
    let mut rng = SmallRng::from_entropy();

    let iter = bytes.by_ref();
    while let Some(mut byte) = iter.next().transpose()? {
        // Check if without leading one
        if byte < 0b_1000_0000 {
            if rng.gen_bool(0.5) {
                byte.make_ascii_uppercase();
            } else {
                byte.make_ascii_lowercase();
            }

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
        let mut index = 1;
        for curr in char_be[1..].iter_mut() {
            let byte = iter.next().ok_or(ErrorKind::InvalidData)??;
            if curr.leading_ones() != 1 {
                return Err(ErrorKind::InvalidData.into());
            }

            *curr = byte;
            index += 1;
        }

        // We have verified `char_be` to be valid UTF-8 up to the `index` (inclusive).
        let text = core::str::from_utf8(&char_be[..=index]).unwrap();
        for ch in text.chars() {
            let mut buf = [0; 4];
            if rng.gen_bool(0.5) {
                for c in ch.to_uppercase() {
                    let bytes = c.encode_utf8(&mut buf).as_bytes();
                    output.write_all(bytes)?;
                }
            } else {
                for c in ch.to_lowercase() {
                    let bytes = c.encode_utf8(&mut buf).as_bytes();
                    output.write_all(bytes)?;
                }
            }
        }
    }

    Ok(())
}

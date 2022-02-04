use rand::{rngs::SmallRng, Rng, SeedableRng};
use std::io::{stdin, stdout, BufRead, BufReader, Result, Write};

fn main() -> Result<()> {
    let mut input = BufReader::new(stdin());
    let mut output = stdout();

    let mut rng = SmallRng::from_entropy();
    let mut buffer = String::new();

    while input.read_line(&mut buffer)? != 0 {
        for mut ch in buffer.chars() {
            if rng.gen_bool(0.5) {
                ch.make_ascii_uppercase()
            } else {
                ch.make_ascii_lowercase()
            }

            let unicode: u32 = ch.into();
            let bytes = unicode.to_be_bytes();
            let text = String::from_utf8_lossy(&bytes);
            output.write_all(text.as_bytes())?;
        }

        buffer.clear();
    }

    Ok(())
}

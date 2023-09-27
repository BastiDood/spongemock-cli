use std::io::{stdin, stdout, Result, Write as _};
use utf8_chars::BufReadCharsExt as _;

fn main() -> Result<()> {
    let mut rng = nanorand::WyRand::new();
    let mut stdout = stdout().lock();
    for c in stdin().lock().chars() {
        let c = c?;
        if nanorand::RandomGen::random(&mut rng) {
            for c in c.to_uppercase() {
                write!(&mut stdout, "{c}")?;
            }
        } else {
            for c in c.to_lowercase() {
                write!(&mut stdout, "{c}")?;
            }
        };
    }
    Ok(())
}

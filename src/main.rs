use rand::{rngs::SmallRng, SeedableRng};
use spongemock::spongemock;
use std::io::{stdin, stdout, Read, Result};

fn main() -> Result<()> {
    let mut dest = stdout();
    let mut rng = SmallRng::from_entropy();
    spongemock(stdin().bytes(), &mut dest, &mut rng)
}

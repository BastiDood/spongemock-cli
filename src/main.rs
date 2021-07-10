use clap::{App, Arg};
use rand::random;
use std::io::{self, stdin, Read};

fn main() -> io::Result<()> {
    // Set up CLI
    let matches = App::new("Spongemock CLI")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::with_name("input"))
        .get_matches();

    // Parse results
    let mut input = if let Some(input) = matches.value_of("input") {
        input.to_owned()
    } else {
        let mut buffer = String::with_capacity(32);
        stdin().lock().read_to_string(&mut buffer)?;
        buffer
    };

    // Optimize the case when all characters are ASCII-based
    if input.is_ascii() {
        // SAFETY: The input has been verified to be within the ASCII range.
        let ascii_bytes = unsafe { input.as_bytes_mut() };
        for byte in ascii_bytes {
            if random() {
                byte.make_ascii_uppercase();
            } else {
                byte.make_ascii_lowercase();
            }
        }
        print!("{}", input);
        return Ok(());
    }

    // Otherwise, implement the less efficient version which requires copying
    let spongemocked: String = input
        .trim()
        .chars()
        .map(|mut c| {
            if random() {
                c.make_ascii_uppercase();
            } else {
                c.make_ascii_lowercase();
            }
            c
        })
        .collect();
    print!("{}", spongemocked);

    Ok(())
}

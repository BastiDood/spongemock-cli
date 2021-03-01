use rand::random;
use std::{
    env,
    io::{stdin, Read},
};

fn main() {
    let mut args = env::args().skip(1);
    let input = args.next().unwrap_or_else(|| {
        let mut buffer = String::with_capacity(32);
        stdin()
            .lock()
            .read_to_string(&mut buffer)
            .expect("unable to read input");
        buffer
    });

    if let "--help" | "-h" = input.as_str() {
        println!("spongemock [--help, -h] [text]");
        return;
    }

    let spongemocked: String = input
        .trim()
        .chars()
        .map(|c| {
            if random() {
                c.to_ascii_uppercase()
            } else {
                c.to_ascii_lowercase()
            }
        })
        .collect();

    print!("{}", spongemocked);
}

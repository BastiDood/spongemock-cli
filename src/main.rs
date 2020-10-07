use std::{ env, io::{ self, Read } };
use rand::random;

fn main() {
    let mut args = env::args().skip(1);
    let input = args.next();

    let text = match input {
        Some(text) => {
            if let "--help" | "-h"  = text.as_str() {
                println!("spongemock [--help, -h] [text]");
                return;
            }
            text
        },
        None => {
            let mut buffer = String::with_capacity(32);
            io::stdin().lock().read_to_string(&mut buffer).unwrap();
            buffer
        },
    };

    let spongemocked: String = text
        .trim()
        .chars()
        .map(|c| if random() { c.to_ascii_uppercase() } else { c.to_ascii_lowercase() })
        .collect();

    print!("{}", spongemocked.as_str());
}

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

    let result: String = text
        .into_bytes()
        .into_iter()
        .map(|byte| {
            let mut ch: char = byte.into();
            if random() { ch.make_ascii_uppercase(); }
            else { ch.make_ascii_lowercase(); }
            return ch;
        })
        .collect();

    print!("{}", result.trim());

}

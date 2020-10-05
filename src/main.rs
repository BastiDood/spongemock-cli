use std::env;
use rand::random;

fn main() {
    let mut args = env::args().skip(1);
    let text = args.next().unwrap_or("--help".to_owned());

    if text == "--help" {
        println!("spongemock [--help] <text>");
        return;
    }

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

    println!("{}", result.as_str());
}

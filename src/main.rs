use std::env;
use rand;

fn main() {
    let mut args = env::args().skip(1);
    let text = args.next().unwrap_or("--help".to_owned());

    if text == "--help" {
        println!("spongemock [--help] <text>");
        return;
    }

    let result: String = text.chars()
        .map(|x| if rand::random() {
            x.to_uppercase().to_string()
        } else {
            x.to_lowercase().to_string()
        })
        .collect();

    println!("{}", result.as_str());
}

use std::env;
use rand;

fn main() {
    let mut args = env::args().skip(1);
    let text = args.next().unwrap_or("--help".to_owned());

    if text == "--help" {
        println!("spongemock [--help] <text>");
        return;
    }

    println!("{}", &text);
}

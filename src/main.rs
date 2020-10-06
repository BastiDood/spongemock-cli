use std::{ env, io::{ self, Read } };
use rand::random;

fn main() {
    let mut args = env::args().skip(1);
    let input = args.next();

    let mut text = match input {
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

    // SAFETY: Since we are looking into a mutable slice
    // into an already valid `String`, the bytes have already
    // been checked beforehand for UTF-8 compliance.
    let underlying_vector = unsafe { text.as_mut_vec() };
    for byte in underlying_vector.iter_mut() {
        if random() { byte.make_ascii_uppercase(); }
        else { byte.make_ascii_lowercase(); }
    } 

    print!("{}", text.as_str());
}

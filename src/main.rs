mod core;

use crate::core::enigma::Enigma;

fn main() {
    let mut _enigma = Enigma::new();
    let encrypted_string = _enigma.encrypt("Hello world");
    println!("Encrypted string: {}", encrypted_string);
}

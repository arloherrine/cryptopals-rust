use std::io;
use bin_utils;

pub fn main() {
    let mut hex_input = String::new();

    io::stdin().read_line(&mut hex_input)
        .expect("Failed to read line");

    println!("{}", bin_utils::hex_to_base64(&hex_input));
}

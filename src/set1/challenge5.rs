use std::io::{self, Read};
use std::iter::FromIterator;
use bin_utils;


pub fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).expect("Failed to read input");

    let plain_text = buffer.as_bytes();

    let key = "ICE";
    let full_key: Vec<u8> = Vec::from_iter(key.as_bytes().iter().cycle().take(plain_text.len()).map(|x| *x));
    let cipher_text = bin_utils::xor_buffers(&plain_text, &full_key);
    println!("{}", bin_utils::bytes_to_hex(&cipher_text));
}

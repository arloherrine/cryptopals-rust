use std::io::prelude::*;
use std::fs::File;

use bin_utils;
use cipher_utils;

pub fn main() {
    let mut input_str = String::new();
    File::open("data/25.txt").expect("Failed to open file")
        .read_to_string(&mut input_str).expect("Failed to read input");

    let mut clean_str = String::new();
    for line in input_str.lines() {
        clean_str.push_str(line)
    }
    let input = bin_utils::base64_to_bytes(clean_str);
    //println!("{}", String::from_utf8(cipher_utils::cbc_decrypt(&input, "YELLOW SUBMARINE".as_bytes(), &vec![0; 16], cipher_utils::decrypt_aes_ecb)).unwrap());
}

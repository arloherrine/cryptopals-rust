use std::io;
use bin_utils;
use char_frequency;

pub fn main() {
    let mut hex_input = String::new();
    io::stdin().read_line(&mut hex_input)
        .expect("Failed to read line");
    let cipher_text = bin_utils::hex_to_bytes(&hex_input);
    let mut keys: Vec<u8> = (0..0b1111111).collect();
    keys.sort_by_key(|k| char_frequency::score_string(&bin_utils::xor_buffers(&cipher_text, &vec![*k; cipher_text.len()])).round() as u32);
    keys.reverse();
    println!("key: 0x{:x}. plain text: {}", keys[0],
        String::from_utf8(bin_utils::xor_buffers(&cipher_text, &vec![keys[0]; cipher_text.len()])).unwrap());
}

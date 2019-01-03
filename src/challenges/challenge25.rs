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
    let plain = cipher_utils::decrypt_aes_ecb(&input, &"YELLOW SUBMARINE".to_string().into_bytes());

    let key = cipher_utils::random_key(16);
    let nonce = cipher_utils::random_key(16);
    let ciphertext = cipher_utils::ctr_crypt(&plain, &key, &nonce);

    let editor = CtrEditor {
        key: key,
        nonce: nonce,
    };

    let len = ciphertext.len();
    let evil_text = vec![0 as u8; len];
    let evil_key_stream = editor.edit(&ciphertext, 0, &evil_text);
    let evil_plain = bin_utils::xor_buffers(&evil_key_stream, &ciphertext);
    println!("{}", bin_utils::bytes_to_ascii(&evil_plain));
}

struct CtrEditor {
    key: Vec<u8>,
    nonce: Vec<u8>,
}

impl CtrEditor {
    fn edit(&self, ciphertext: &[u8], offset: usize, newtext: &[u8]) -> Vec<u8> {
        cipher_utils::ctr_crypt_edit(ciphertext, &self.key, &self.nonce, offset, newtext)
    }
}


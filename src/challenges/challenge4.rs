use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::iter::FromIterator;
use bin_utils;
use char_frequency;

pub fn main() {
    let path = Path::new("data/set1chal4.txt");

    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(_) => panic!("couldn't open data file"),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(_) => panic!("couldn't read data file"),
        Ok(_) => (),
    }

    let candidates = s.lines().flat_map(|line| candidates(line));
    let best = candidates.max().unwrap();
    println!("key: 0x{:x}\ncipher text: {}\nplain text: {}",
        best.key, bin_utils::bytes_to_hex(&best.cipher_text), String::from_utf8(best.plain_text).unwrap());
}

#[derive(PartialOrd)]
#[derive(PartialEq)]
#[derive(Ord)]
#[derive(Eq)]
struct CipherCandidate {
    score: u32,
    key: u8,
    cipher_text: Vec<u8>,
    plain_text: Vec<u8>,
}

impl CipherCandidate {
    fn new(key: u8, cipher_text: Vec<u8>) -> CipherCandidate {
        let plain_text = bin_utils::xor_buffers(&cipher_text, &vec![key; cipher_text.len()]);
        let score = char_frequency::score_string(&plain_text);
        CipherCandidate{key: key, plain_text: plain_text, cipher_text: cipher_text, score: (score * 100.0) as u32}
    }
}

fn candidates(s: &str) -> Vec<CipherCandidate> {
    //let cipher_text = bin_utils::hex_to_bytes(s);
    let keys: Vec<u8> = (0..0b1111111).collect();
    Vec::from_iter(keys.iter().map(|k| CipherCandidate::new(*k, bin_utils::hex_to_bytes(s))))
}
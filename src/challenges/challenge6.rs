use std::f32;
use std::io::prelude::*;
use std::fs::File;
use std::iter::FromIterator;

use bin_utils;
use cipher_utils;

pub fn main() {
    let mut input_str = String::new();
    File::open("data/set1chal6.txt").expect("Failed to open file")
        .read_to_string(&mut input_str).expect("Failed to read input");

    let mut clean_str = String::new();
    for line in input_str.lines() {
        clean_str.push_str(line)
    }

    let input = bin_utils::base64_to_bytes(clean_str);

    let mut key_size = 0;
    let mut best_score = f32::MAX;
    for k in 2..40 {
        let divisor = (k as f32) * 6.0;
        let score = 0.0
            + bin_utils::hamming_distance(&input[k*0..k*1], &input[k*1..k*2]) as f32 / divisor
            + bin_utils::hamming_distance(&input[k*0..k*1], &input[k*2..k*3]) as f32 / divisor
            + bin_utils::hamming_distance(&input[k*0..k*1], &input[k*3..k*4]) as f32 / divisor
            + bin_utils::hamming_distance(&input[k*1..k*2], &input[k*2..k*3]) as f32 / divisor
            + bin_utils::hamming_distance(&input[k*1..k*2], &input[k*2..k*3]) as f32 / divisor
            + bin_utils::hamming_distance(&input[k*2..k*3], &input[k*3..k*4]) as f32 / divisor;
        if score < best_score {
            key_size = k;
            best_score = score;
        }
    }

    let chunked = bin_utils::chunkify(&input, key_size);
    let transposed = bin_utils::transpose(chunked);
    let key_vec: Vec<u8> = transposed.iter().map(|x| cipher_utils::solve_single_xor(&x)).collect();
    let full_key: Vec<u8> = Vec::from_iter(key_vec.iter().cycle().take(input.len()).map(|x| *x));
    let plain_text = bin_utils::xor_buffers(&input, &full_key);
    println!("key: {}", String::from_utf8(key_vec).unwrap());
    println!("Plain text:");
    println!("{}", String::from_utf8(plain_text).unwrap());
}

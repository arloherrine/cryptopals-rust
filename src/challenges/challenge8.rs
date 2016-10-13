use std::io::prelude::*;
use std::fs::File;

use bin_utils;

pub fn main() {
    let mut input_str = String::new();
    File::open("data/set1chal8.txt").expect("Failed to open file")
        .read_to_string(&mut input_str).expect("Failed to read input");

    for (line_num, line) in input_str.lines().enumerate() {
        let cipher = bin_utils::hex_to_bytes(line);
        let size = 16;
        let blocks = cipher.len() / 16;

        if (0..blocks)
                .flat_map(|i|->Vec<(usize, usize)> {(i+1..blocks).map(|j| (i, j)).collect()})
                .any(|(i, j)| cipher[i*size..(i+1)*size] == cipher[j*size..(j+1)*size]) {
            println!("{}", line_num);
        }
    }
}

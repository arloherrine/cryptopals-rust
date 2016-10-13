use bin_utils;
use char_frequency;

pub fn solve_single_xor(cipher_text: &[u8]) -> u8 {
    let mut best_score = 0.0;
    let mut best_key = 0;
    for key in 0..u8::max_value() {
        let score = char_frequency::score_string(&single_xor(key, cipher_text));
        if score > best_score {
            best_score = score;
            best_key = key;
        }
    }
    best_key
}

pub fn single_xor(key: u8, text: &[u8]) -> Vec<u8> {
    bin_utils::xor_buffers(text, &vec![key; text.len()])
}

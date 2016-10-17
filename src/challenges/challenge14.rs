use rand;

use bin_utils;
use cipher_utils;

pub fn main() {
    let black_box = BlackBox::new();

    let (block_size, secret_size) = find_block_size(&black_box).unwrap();
    println!("block size: {}", block_size);

    let (start_block, pad_length) = find_starting_points(&black_box, block_size);
    println!("start block: {}, pad length {}", start_block, pad_length);
    let start_index = start_block * block_size;

    let mut deciphered = Vec::new();
    while deciphered.len() < secret_size - (start_index - pad_length) {
        let mut payload = vec!['X' as u8; pad_length];
        payload.extend(vec!['A' as u8; block_size - (deciphered.len() % block_size) - 1]);
        let deciphered_block_bytes = (deciphered.len() / block_size) * block_size;
        let target = &black_box.encrypt(&payload)[start_index + deciphered_block_bytes..start_index + deciphered_block_bytes + block_size];

        let mut payload = vec!['A' as u8; pad_length];
        if block_size - 1 > deciphered.len() {
            payload.extend(vec!['A' as u8; block_size - 1 - deciphered.len()]);
            payload.extend_from_slice(&deciphered[..]);
        } else {
            payload.extend_from_slice(&deciphered[deciphered.len() - (block_size - 1)..]);
        }
        payload.push('X' as u8);

        let byte = (0..u8::max_value()).find(|b: &u8| {
            payload[pad_length + block_size - 1] = *b;
            let result = black_box.encrypt(&payload);
            &result[start_index..start_index + block_size] == target
        });
        deciphered.push(byte.expect("Failed to find matching byte"));
    }
    println!("{}", String::from_utf8(deciphered).unwrap());
}

fn find_starting_points(black_box: &BlackBox, block_size: usize) -> (usize, usize) {
    let mut input = vec!['A' as u8; block_size * 2];
    while input.len() < block_size * 4 {
        let size = input.len();
        let encrypted = black_box.encrypt(&input);
        for i in 0..encrypted.len() / block_size - 1 {
            let block_1 = &encrypted[i * block_size..(i + 1) * block_size];
            let block_2 = &encrypted[(i + 1) * block_size..(i + 2) * block_size];
            if block_1 == block_2 {
                for j in 0..size - 2 * block_size {
                    input[j] = 'X' as u8;
                    let encrypted = black_box.encrypt(&input);
                    if block_1 != &encrypted[i * block_size..(i + 1) * block_size] {
                        return (i, j);
                    }
                }
            }
        }
        input.push('A' as u8);
    }
    panic!("Never found two consecutive identical blocks");
}

fn find_block_size(black_box: &BlackBox) -> Option<(usize, usize)> {
    let old_size = black_box.encrypt("A".as_bytes()).len();
    for size in 2..30 {
        let new_size = black_box.encrypt(&vec!['A' as u8; size]).len();
        if new_size != old_size {
            return Some((new_size - old_size, old_size - (size - 1)))
        }
    }
    None
}

struct BlackBox {
    secret: Vec<u8>,
    prefix: Vec<u8>,
    key: Vec<u8>,
}

impl BlackBox {
    fn new() -> BlackBox {
        BlackBox{
            secret: bin_utils::base64_to_bytes("Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkgaGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBqdXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUgYnkK".to_string()),
            prefix: cipher_utils::random_key((rand::random::<usize>() % 50) + 5),
            key: cipher_utils::random_key(16),
        }
    }

    fn encrypt(&self, data: &[u8]) -> Vec<u8> {
        let mut result = self.prefix.to_vec();
        result.extend_from_slice(data);
        result.extend(&self.secret);
        cipher_utils::encrypt_aes_ecb(&result, &self.key)
    }
}

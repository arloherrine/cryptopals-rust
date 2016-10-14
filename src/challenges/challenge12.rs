use bin_utils;
use cipher_utils;

pub fn main() {
    let black_box = BlackBox::new();

    let (block_size, secret_size) = find_block_size(&black_box).unwrap();
    println!("block size: {}", block_size);
    confirm_ecb(&black_box);

    let mut deciphered = vec!['A' as u8; block_size - 1];
    while deciphered.len() - (block_size - 1) < secret_size {
        let deciphered_bytes = deciphered.len() - (block_size - 1);
        let start = (deciphered_bytes / block_size) * block_size;

        let craft_len = block_size - (deciphered_bytes % block_size) - 1;
        let target = &black_box.encrypt(&deciphered[start..start + craft_len])[start..start + block_size];

        let mut crafted = deciphered[deciphered.len() - (block_size - 1)..].to_vec();
        let byte = (0..u8::max_value()).find(|b: &u8| {
            crafted.push(*b);
            let result = black_box.encrypt(&crafted);
            crafted.pop();
            &result[0..block_size] == target
        });
        deciphered.push(byte.unwrap());
    }
    deciphered.drain(..block_size - 1);
    println!("{}", String::from_utf8(deciphered).unwrap());

}

fn confirm_ecb(black_box: &BlackBox) {
    let result = black_box.encrypt(&vec!['A' as u8; 32]);
    if result[0..16] != result[16..32] {
        panic!("Not ECB??");
    }
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
    key: Vec<u8>,
}

impl BlackBox {
    fn new() -> BlackBox {
        BlackBox{
            secret: bin_utils::base64_to_bytes("Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkgaGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBqdXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUgYnkK".to_string()),
            key: cipher_utils::random_key(16),
        }
    }

    fn encrypt(&self, data: &[u8]) -> Vec<u8> {
        let mut result = Vec::new();
        result.extend_from_slice(data);
        result.extend(&self.secret);
        cipher_utils::encrypt_aes_ecb(&result, &self.key)
    }
}

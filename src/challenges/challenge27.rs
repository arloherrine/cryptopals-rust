use bin_utils;
use cipher_utils;

pub fn main() {
    let oracle = Oracle::new();
    let cipher = oracle.encrypt("who lives in a pineapple under the sea? SPONGEBOB SQUAREPANTS".to_string().as_bytes());
    let chunks = bin_utils::chunkify(&cipher, 16);
    let mut evil_cipher = Vec::new();
    evil_cipher.extend(&chunks[0]);
    evil_cipher.extend(&[0; 16]);
    evil_cipher.extend(&chunks[0]);
    let (_, plain) = oracle.decrypt(&evil_cipher);
    let evil_chunks = bin_utils::chunkify(&plain, 16);
    let evil_key = bin_utils::xor_buffers(&evil_chunks[0], &evil_chunks[2]);

    println!("evil key: {}", bin_utils::bytes_to_hex(&evil_key));
    println!("real key: {}", bin_utils::bytes_to_hex(&oracle.key));

}

struct Oracle {
    key: Vec<u8>,
}

impl Oracle {
    fn new() -> Oracle {
        Oracle {
            key: cipher_utils::random_key(16),
        }
    }

    fn encrypt(&self, input: &[u8]) -> Vec<u8> {
        cipher_utils::cbc_encrypt(input, &self.key, &self.key, cipher_utils::encrypt_aes_ecb)
    }

    fn decrypt(&self, input: &[u8]) -> (bool, Vec<u8>) {
        let plain = cipher_utils::cbc_decrypt(input, &self.key, &self.key, cipher_utils::decrypt_aes_ecb);
        (plain.iter().any(|c| *c > (std::u8::MAX >> 1)), plain)
    }

}

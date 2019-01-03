use bin_utils;
use cipher_utils;

pub fn main() {
    let oracle = Oracle::new();
    
    let payload_0 = vec![0; 32];
    let ciphertext_0 = oracle.build_and_encrypt(&payload_0);
    let chunks_0 = bin_utils::chunkify(&ciphertext_0, 16);

    let payload_1 = vec![1; 32];
    let ciphertext_1 = oracle.build_and_encrypt(&payload_1);
    let chunks_1 = bin_utils::chunkify(&ciphertext_1, 16);

    let first_diff = chunks_0.into_iter().zip(chunks_1.into_iter()).position(|(chunk_0, chunk_1)| {
        chunk_0.into_iter().zip(chunk_1.into_iter()).any(|(byte_1, byte_2)| byte_1 != byte_2)
    }).unwrap();

    let start_index = (first_diff + 1) * 16;

    let mut evil_ciphertext = Vec::new();
    evil_ciphertext.extend_from_slice(&ciphertext_0[0..start_index]);
    evil_ciphertext.extend(bin_utils::xor_buffers(&ciphertext_0[start_index..start_index + 16], &"xx;admin=true;xx".to_string().as_bytes()));
    evil_ciphertext.extend_from_slice(&ciphertext_0[start_index + 16..ciphertext_0.len()]);
    println!("attack got admin? {}", oracle.is_admin(&evil_ciphertext));
}

struct Oracle {
    key: Vec<u8>,
    nonce: Vec<u8>,
}

const PREFIX: &'static str = "comment1=cooking%20MCs;userdata=";
const SUFFIX: &'static str = ";comment2=%20like%20a%20pound%20of%20bacon";

impl Oracle {
    fn new() -> Oracle {
        Oracle {
            key: cipher_utils::random_key(16),
            nonce: cipher_utils::random_key(16),
        }
    }

    fn build_and_encrypt(&self, input: &[u8]) -> Vec<u8> {
        let mut data = PREFIX.as_bytes().to_vec();
        data.extend(input.iter().filter(|&&b| b != (';' as u8) && b != ('=' as u8)));
        data.extend_from_slice(SUFFIX.as_bytes());
        cipher_utils::ctr_crypt(&data, &self.key, &self.nonce)
    }

    fn is_admin(&self, cipher: &[u8]) -> bool {
        let plain = bin_utils::bytes_to_ascii(&cipher_utils::ctr_crypt(&cipher, &self.key, &self.nonce));
        if let Some(_) = plain.find(";admin=true;") {
            true
        } else {
            false
        }
    }
}


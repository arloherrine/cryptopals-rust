use bin_utils;
use cipher_utils;

pub fn main() {

    let oracle = Oracle::new();
    let blank_result = oracle.build_and_encrypt(&vec![0; 32]);

    let mut mask = vec![0; 36];
    mask.extend_from_slice(";admin=true;".as_bytes());
    mask.extend(vec![0; blank_result.len() - 48]);

    let payload = bin_utils::xor_buffers(&blank_result, &mask);
    println!("attack got admin? {}", oracle.is_admin(&payload));
}

struct Oracle {
    key: Vec<u8>,
    iv: Vec<u8>,
}

const PREFIX: &'static str = "comment1=cooking%20MCs;userdata=";
const SUFFIX: &'static str = ";comment2=%20like%20a%20pound%20of%20bacon";

impl Oracle {
    fn new() -> Oracle {
        Oracle {
            key: cipher_utils::random_key(16),
            iv: cipher_utils::random_key(16),
        }
    }

    fn build_and_encrypt(&self, input: &[u8]) -> Vec<u8> {
        let mut data = PREFIX.as_bytes().to_vec();
        data.extend(input.iter().filter(|&&b| b != (';' as u8) && b != ('=' as u8)));
        data.extend_from_slice(SUFFIX.as_bytes());
        cipher_utils::cbc_encrypt(&data, &self.key, &self.iv, cipher_utils::encrypt_aes_ecb)
    }

    fn is_admin(&self, cipher: &[u8]) -> bool {
        let plain = bin_utils::bytes_to_ascii(&cipher_utils::strip_pkcs(cipher_utils::cbc_decrypt(
            &cipher, &self.key, &self.iv, cipher_utils::decrypt_aes_ecb)).expect("expected valid padding"));
        if let Some(_) = plain.find(";admin=true;") {
            true
        } else {
            false
        }
    }
}

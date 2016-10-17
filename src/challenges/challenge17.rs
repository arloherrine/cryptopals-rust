use bin_utils;
use cipher_utils;

const CIPHER_TEXTS: [&'static str; 10] = [
    "MDAwMDAwTm93IHRoYXQgdGhlIHBhcnR5IGlzIGp1bXBpbmc=",
    "MDAwMDAxV2l0aCB0aGUgYmFzcyBraWNrZWQgaW4gYW5kIHRoZSBWZWdhJ3MgYXJlIHB1bXBpbic=",
    "MDAwMDAyUXVpY2sgdG8gdGhlIHBvaW50LCB0byB0aGUgcG9pbnQsIG5vIGZha2luZw==",
    "MDAwMDAzQ29va2luZyBNQydzIGxpa2UgYSBwb3VuZCBvZiBiYWNvbg==",
    "MDAwMDA0QnVybmluZyAnZW0sIGlmIHlvdSBhaW4ndCBxdWljayBhbmQgbmltYmxl",
    "MDAwMDA1SSBnbyBjcmF6eSB3aGVuIEkgaGVhciBhIGN5bWJhbA==",
    "MDAwMDA2QW5kIGEgaGlnaCBoYXQgd2l0aCBhIHNvdXBlZCB1cCB0ZW1wbw==",
    "MDAwMDA3SSdtIG9uIGEgcm9sbCwgaXQncyB0aW1lIHRvIGdvIHNvbG8=",
    "MDAwMDA4b2xsaW4nIGluIG15IGZpdmUgcG9pbnQgb2g=",
    "MDAwMDA5aXRoIG15IHJhZy10b3AgZG93biBzbyBteSBoYWlyIGNhbiBibG93",
];

pub fn main() {

    let oracle = Oracle::new();
    for index in 0..10 {
        let (cipher_text, iv) = oracle.get_encrypted(index);

        let chunks: Vec<&[u8]> = iv.chunks(16).chain(cipher_text.chunks(16)).collect();
        let solution_blocks = chunks[0..chunks.len() - 1].iter().zip(chunks[1..chunks.len()].iter()).map(
            |(&iv, &chunk)| -> Vec<u8> {
                let mut payload = vec![0 as u8; 16];
                payload.extend(chunk);

                let mut solution = Vec::new();
                for byte_index in (0..16).rev() {
                    for (i, byte) in solution.iter().enumerate() {
                        payload[16 - i - 1] = byte ^ (16 - byte_index as u8);
                    }
                    for byte in 0..u8::max_value() {
                        payload[byte_index] = byte;
                        if oracle.valid_padding(&payload, &vec![0; 16]) {
                            let plain_byte = byte ^ (16 - byte_index as u8);
                            solution.push(plain_byte);
                            break;
                        }
                    }
                }
                solution.reverse();
                bin_utils::xor_buffers(&iv, &solution)
            });

        let mut solution = Vec::new();
        for block in solution_blocks {
            solution.extend(block);
        }
        let duplicate = solution.to_vec();
        let stripped = cipher_utils::strip_pkcs(solution);
        if let Some(clean) = stripped {
            let duplicate = clean.to_vec();
            match String::from_utf8(clean) {
                Err(_) => println!("{}", bin_utils::bytes_to_ascii(&duplicate)),
                Ok(s) => println!("{}", s),
            };
        } else {
            let triplicate = duplicate.to_vec();
            match String::from_utf8(duplicate) {
                Err(_) => println!("{}", bin_utils::bytes_to_ascii(&triplicate)),
                Ok(s) => println!("{}", s),
            };
        }
    }
}

struct Oracle {
    key: Vec<u8>,
}

impl Oracle {
    fn new() -> Oracle {
        Oracle { key: cipher_utils::random_key(16) }
    }

    fn get_encrypted(&self, index: usize) -> (Vec<u8>, Vec<u8>) {
        let iv = cipher_utils::random_key(16);
        //let iv = vec![0; 16];
        (cipher_utils::cbc_encrypt(&bin_utils::base64_to_bytes(CIPHER_TEXTS[index].to_string()),
                                   &self.key, &iv, cipher_utils::encrypt_aes_ecb) , iv)
    }

    fn valid_padding(&self, data: &[u8], iv: &[u8]) -> bool {
        let padded = cipher_utils::cbc_decrypt(data, &self.key, iv, cipher_utils::decrypt_aes_ecb);
        if let Some(_) = cipher_utils::strip_pkcs(padded) {
            true
        } else {
            false
        }
    }
}

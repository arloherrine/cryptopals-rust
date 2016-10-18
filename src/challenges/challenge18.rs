use bin_utils;
use cipher_utils;

pub fn main() {
    let cipher = bin_utils::base64_to_bytes("L77na/nrFsKvynd6HzOoG7GHTLXsTVu9qvY/2syLXzhPweyyMTJULu/6/kXX0KSvoOLSFQ==".to_string());
    let plain = cipher_utils::ctr_crypt(&cipher, "YELLOW SUBMARINE".to_string().into_bytes(), vec![0 as u8; 16]);
    println!("{}", bin_utils::bytes_to_ascii(&plain));
}

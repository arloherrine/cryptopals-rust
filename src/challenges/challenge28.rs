use bin_utils;
use cipher_utils;

pub fn main() {
    let key = cipher_utils::random_key(16);
    let message = "This is a message baby!".to_string();
    let message_1 = "This is a massage baby!".to_string();

    let hmac = cipher_utils::sha1_hmac(&key, message.as_bytes());
    let hmac_1 = cipher_utils::sha1_hmac(&key, message_1.as_bytes());
    println!("hmac_0: {}", bin_utils::bytes_to_hex(&hmac));
    println!("hmac_1: {}", bin_utils::bytes_to_hex(&hmac_1));

}

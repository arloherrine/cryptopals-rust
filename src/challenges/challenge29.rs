use crypto::digest::Digest;
use crypto::sha1::Sha1;
use rand;

use bin_utils;
use cipher_utils;

pub fn main() {
    let key_rand: usize = rand::random();
    let key = cipher_utils::random_key(key_rand % 100);
    let message = "comment1=cooking%20MCs;userdata=foo;comment2=%20like%20a%20pound%20of%20bacon".to_string();
    

    let mut payload = Vec::new();
    payload.extend_from_slice(&key);
    payload.extend_from_slice(message.as_bytes());
    let mut hasher = Sha1::new();
    hasher.input(&payload);

    let evil_payload = ";admin=true".to_string();
    
    let result: Option<(Vec<u8>, Vec<u8>)> = (1..100).find_map(|pad_len| {
        let (evil_message, evil_hmac) = forged_hmac(message.as_bytes(), evil_payload.as_bytes(), &hasher, pad_len);
        if cipher_utils::verify_sha1_hmac(&evil_message, &key, &evil_hmac) {
            Some((evil_message, evil_hmac))
        } else {
            None
        }
    });
    if let Some((evil_message, evil_hmac)) = result {
        println!("message: {}, hmac: {}", bin_utils::bytes_to_ascii(&evil_message), bin_utils::bytes_to_hex(&evil_hmac));
    } else {
        println!("Failed to find one");
    }

}

fn forged_hmac(message: &[u8], evil_payload: &[u8], hasher: &Sha1, pad_len: usize) -> (Vec<u8>, Vec<u8>) {
    let mut evil_message = Vec::new();
    evil_message.extend(message);

    // glue padding
    evil_message.push(0x80);
    evil_message.extend(vec![0; pad_len - 1]);

    evil_message.extend(evil_payload);
    //println!("{}", bin_utils::bytes_to_ascii(&evil_message));
    let mut cloned_hasher = hasher.clone();
    let mut evil_hmac = vec![0; hasher.output_bytes()];
    cloned_hasher.result(&mut evil_hmac);
    (evil_message, evil_hmac)
}

fn pad_sha1(data: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    result.extend_from_slice(data);
    result.push(0x80);
    let zeros = 64 - result.len() % 64;
    for _ in 0..zeros {
        result.push(0);
    }
    result
}

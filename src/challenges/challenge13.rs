use cipher_utils;

use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;
use std::iter::FromIterator;

pub fn main() {
    let oracle = Oracle::new();

    let email_cipher = oracle.encrypted_profile_for("me@aaa.com");
    let email_block = &email_cipher[0..16];

    let admin_cipher = oracle.encrypted_profile_for("aaaa@aaa.admin");
    let admin_block = &admin_cipher[16..32];

    let role_cipher = oracle.encrypted_profile_for("mmm@mmm.mm");
    let role_block = &role_cipher[16..32];

    let mut evil_cipher = Vec::new();
    evil_cipher.extend_from_slice(email_block);
    evil_cipher.extend_from_slice(role_block);
    evil_cipher.extend_from_slice(admin_block);
    let profile_map = oracle.decrypt(&evil_cipher);
    print_map(&profile_map);
}

struct Oracle {
    key: Vec<u8>,
}

impl Oracle {
    fn new() -> Oracle {
        Oracle {key: cipher_utils::random_key(16)}
    }

    fn encrypted_profile_for(&self, email: &str) -> Vec<u8> {
        let encoded = profile_for(email);
        self.encrypt(&encoded)
    }

    fn encrypt(&self, profile: &str) -> Vec<u8> {
        cipher_utils::encrypt_aes_ecb(profile.as_bytes(), &self.key)
    }

    fn decrypt(&self, cipher: &[u8]) -> HashMap<String, String> {
        parse_cookie_struct(&String::from_utf8(cipher_utils::decrypt_aes_ecb(cipher, &self.key)).unwrap())
    }
}

fn profile_for(email: &str) -> String {
    let mut encoded = String::from("email='");
    encoded.push_str(&String::from_iter(email.chars().filter(|&c| c != '&' && c != '=')));
    encoded.push_str("'&uid=10&role='user'");
    encoded
}

fn parse_cookie_struct(input: &str) -> HashMap<String, String> {
    let mut result = HashMap::new();
    for entry in input.split('&') {
        let mut parts = entry.split('=');
        let key_opt = parts.next();
        let value_opt = parts.next();
        if let Some(key) = key_opt {
            if let Some(value) = value_opt {
                result.insert(String::from(key.trim()), String::from(value.trim()));
            }
        }
    }
    result
}

fn print_map<K, V>(map: &HashMap<K, V>) where K: Display + Eq + Hash, V: Display + Eq + Hash {
    println!("{{");
    for (k, v) in map.iter() {
        println!("    {}: {},", k, v);
    }
    println!("}}");
}

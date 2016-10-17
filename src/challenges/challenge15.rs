use bin_utils;
use cipher_utils;

pub fn main() {
    let mut string = "ICE ICE BABY".as_bytes().to_vec();
    string.extend(vec![4; 4]);
    print!("{}", bin_utils::bytes_to_ascii(&string));
    if let Some(stripped) = cipher_utils::strip_pkcs(string) {
        println!(": has valid pkcs padding -> {}", String::from_utf8(stripped).unwrap());
    } else {
        println!(": does not have valid pkcs padding...");
    }

    let mut string = "ICE ICE BABY".as_bytes().to_vec();
    string.extend(vec![5; 4]);
    print!("{}", bin_utils::bytes_to_ascii(&string));
    if let Some(stripped) = cipher_utils::strip_pkcs(string) {
        println!(": has valid pkcs padding -> {}", String::from_utf8(stripped).unwrap());
    } else {
        println!(": does not have valid pkcs padding...");
    }

    let mut string = "ICE ICE BABY".as_bytes().to_vec();
    string.extend(vec![1, 2, 3, 4]);
    print!("{}", bin_utils::bytes_to_ascii(&string));
    if let Some(stripped) = cipher_utils::strip_pkcs(string) {
        println!(": has valid pkcs padding -> {}", String::from_utf8(stripped).unwrap());
    } else {
        println!(": does not have valid pkcs padding...");
    }
}

use bin_utils;

pub fn main() {
    let input = "YELLOW SUBMARINE".to_string().into_bytes();
    println!("{}", bin_utils::bytes_to_ascii(&bin_utils::pkcs_pad(&input, 20)));
}

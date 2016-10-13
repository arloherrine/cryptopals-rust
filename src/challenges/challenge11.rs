use cipher_utils;

pub fn main() {
    let plain = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA".as_bytes();
    for _ in 0..20 {
        let result = cipher_utils::ecb_cbc_rand_encrypt(&plain);
        let mut ecb = false;
        for i in 15..39 {
            if result[i..i+16] == result[i+16..i+32] {
                ecb = true;
            }
        }
        if ecb {
            println!("Guess ecb\n");
        } else {
            println!("Guess cbc\n");
        }
    }
}

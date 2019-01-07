use dh;
use bin_utils;

pub fn main() {
    let (priv_a, pub_a) = dh::generate_standard_keys();
    let (priv_b, pub_b) = dh::generate_standard_keys();

    println!("s_a_B: 0x{}", bin_utils::bytes_to_hex(&dh::session_key(&priv_a, &pub_b, &dh::P).to_bytes_be()));
    println!("s_b_A: 0x{}", bin_utils::bytes_to_hex(&dh::session_key(&priv_b, &pub_a, &dh::P).to_bytes_be()));
}

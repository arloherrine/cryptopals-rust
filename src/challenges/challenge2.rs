use std::io;
use bin_utils;

pub fn main() {
    let mut hex_input1 = String::new();
    let mut hex_input2 = String::new();

    io::stdin().read_line(&mut hex_input1)
        .expect("Failed to read line 1");
    io::stdin().read_line(&mut hex_input2)
        .expect("Failed to read line 2");
    let buf1 = bin_utils::hex_to_bytes(&hex_input1);
    let buf2 = bin_utils::hex_to_bytes(&hex_input2);
    let xord = bin_utils::xor_buffers(&buf1, &buf2);
    println!("{}", bin_utils::bytes_to_hex(&xord));
}

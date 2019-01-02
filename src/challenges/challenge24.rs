use mt;
use rand;
use bin_utils;
use cipher_utils;

pub fn main() {
    //let plain_text: Vec<u8> = cipher_utils::random_key(rand::random::<usize>() % 50).iter()
    let plain_text: Vec<u8> = cipher_utils::random_key(rand::random::<usize>() % 5).iter()
        .chain(vec!['A' as u8; 14].iter()).cloned().collect();
    let key: u16 = rand::random();
    println!("real key: {:04x}", key);
    let cipher_text = mt_stream_crypt(&plain_text, key);

    let key_stream = bin_utils::xor_buffers(&plain_text, &cipher_text);
    let block_start = ((plain_text.len() - 14) / 4 + 1) * 4;
    let target =
        (key_stream[block_start] as u32) << 24
        ^ (key_stream[block_start + 1] as u32) << 16
        ^ (key_stream[block_start + 2] as u32) << 8
        ^ (key_stream[block_start + 3] as u32);
    let cracked_key = (0..u16::max_value()).find(|&seed| mt::WordStream::with_seed(seed).nth(block_start / 4).unwrap() == target);
    if let Some(solved) = cracked_key {
        println!("cracked key: {:04x}", solved);
    } else {
        println!("failed to solve");
    }
}

fn mt_stream_crypt(data: &[u8], key: u16) -> Vec<u8> {
    mt::ByteStream::with_seed(key).zip(data.iter()).map(|(a, &b)| a ^ b).collect()
}

pub fn hex_to_base64(hex_input: &str) -> String {
    let bin_bytes = hex_to_bytes(hex_input);
    bytes_to_base64(&bin_bytes[..])
}

pub fn hex_to_bytes(hex_input: &str) -> Vec<u8> {
    let hex_bytes = hex_input.as_bytes();
    let mut bin_bytes = Vec::new();
    for i in 0..(hex_bytes.len() / 2) {
        let a = hex_to_byte(hex_bytes[i * 2]);
        let b = hex_to_byte(hex_bytes[i * 2 + 1]);
        bin_bytes.push((a << 4) | b);
    }
    bin_bytes
}

pub fn hex_to_byte(byte: u8) -> u8 {
    return match byte {
        48 => 0,
        49 => 1,
        50 => 2,
        51 => 3,
        52 => 4,
        53 => 5,
        54 => 6,
        55 => 7,
        56 => 8,
        57 => 9,
        97 => 10,
        98 => 11,
        99 => 12,
        100 => 13,
        101 => 14,
        102 => 15,
        _ => unimplemented!(),
    };
}

const B64_STRINGS: [&'static str; 64] = [
    "A","B","C","D","E","F","G","H","I","J","K","L","M","N","O","P","Q","R","S","T","U","V","W","X","Y","Z",
    "a","b","c","d","e","f","g","h","i","j","k","l","m","n","o","p","q","r","s","t","u","v","w","x","y","z",
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "+", "/",
];

pub fn bytes_to_base64(bin_bytes: &[u8]) -> String {
    let mut b64_str = String::new();
    for i in 0..(bin_bytes.len() / 3) {
        let pad_one = bin_bytes.len() <= (i*3 + 1);
        let pad_two = bin_bytes.len() <= (i*3 + 2);
        let x = bin_bytes[i * 3] as usize;
        let y = if pad_one {0} else {bin_bytes[i*3 + 1] as usize};
        let z = if pad_two {0} else {bin_bytes[i*3 + 2] as usize};

        let a = B64_STRINGS[(x >> 2) as usize];
        let b = B64_STRINGS[((x << 4) & 63) | (y >> 4)];
        let c = if pad_one {"="} else {B64_STRINGS[((y << 2) & 63) | (z >> 6)]};
        let d = if pad_two {"="} else {B64_STRINGS[z & 63]};
        b64_str.push_str(a);
        b64_str.push_str(b);
        b64_str.push_str(c);
        b64_str.push_str(d);
    }
    b64_str
}

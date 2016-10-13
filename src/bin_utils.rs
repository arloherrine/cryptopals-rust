use std::iter::FromIterator;

pub fn hex_to_base64(hex_input: &str) -> String {
    let bin_bytes = hex_to_bytes(hex_input);
    bytes_to_base64(&bin_bytes[..])
}

fn base64_to_byte(byte: u8) -> Option<u8> {
    B64_STRINGS.iter().position(|c| *c as u8 == byte).map(|x| x as u8)
}

pub fn base64_to_bytes(b64_input: String) -> Vec<u8> {
    let mut bin_bytes = Vec::new();
    for chunk in b64_input.into_bytes().chunks(4) {
        let a = base64_to_byte(chunk[0]).unwrap();
        let b = base64_to_byte(chunk[1]).unwrap();
        let c = base64_to_byte(chunk[2]);
        let d = base64_to_byte(chunk[3]);
        bin_bytes.push((a << 2) | (b >> 4));
        if let Some(x) = c {
            bin_bytes.push((b << 4) | (x >> 2));
            if let Some(y) = d {
                bin_bytes.push((x << 6) | y);
            }
        }
    }
    bin_bytes
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

const B64_STRINGS: [u8; 64] = [
    'A' as u8,'B' as u8,'C' as u8,'D' as u8,'E' as u8,'F' as u8,'G' as u8,'H' as u8,'I' as u8,'J' as u8,'K' as u8,'L' as u8,'M' as u8,'N' as u8,'O' as u8,'P' as u8,'Q' as u8,'R' as u8,'S' as u8,'T' as u8,'U' as u8,'V' as u8,'W' as u8,'X' as u8,'Y' as u8,'Z' as u8,
    'a' as u8,'b' as u8,'c' as u8,'d' as u8,'e' as u8,'f' as u8,'g' as u8,'h' as u8,'i' as u8,'j' as u8,'k' as u8,'l' as u8,'m' as u8,'n' as u8,'o' as u8,'p' as u8,'q' as u8,'r' as u8,'s' as u8,'t' as u8,'u' as u8,'v' as u8,'w' as u8,'x' as u8,'y' as u8,'z' as u8,
    '0' as u8, '1' as u8, '2' as u8, '3' as u8, '4' as u8, '5' as u8, '6' as u8, '7' as u8, '8' as u8, '9' as u8, '+' as u8, '/' as u8,
];

const HEX_STRINGS: [&'static str; 16] = [
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "a", "b", "c", "d", "e", "f",
];

pub fn bytes_to_base64(bin_bytes: &[u8]) -> String {
    let mut b64_str = String::new();
    for i in 0..(bin_bytes.len() / 3) {
        let pad_one = bin_bytes.len() <= (i*3 + 1);
        let pad_two = bin_bytes.len() <= (i*3 + 2);
        let x = bin_bytes[i * 3] as usize;
        let y = if pad_one {0} else {bin_bytes[i*3 + 1] as usize};
        let z = if pad_two {0} else {bin_bytes[i*3 + 2] as usize};

        let a = B64_STRINGS[(x >> 2)];
        let b = B64_STRINGS[((x << 4) & 63) | (y >> 4)];
        let c = if pad_one {'=' as u8} else {B64_STRINGS[((y << 2) & 63) | (z >> 6)]};
        let d = if pad_two {'=' as u8} else {B64_STRINGS[z & 63]};
        b64_str.push(a as char);
        b64_str.push(b as char);
        b64_str.push(c as char);
        b64_str.push(d as char);
    }
    b64_str
}

pub fn bytes_to_hex(bin_bytes: &[u8]) -> String {
    let mut hex_str = String::new();
    for byte in bin_bytes {
        hex_str.push_str(HEX_STRINGS[(byte >> 4) as usize]);
        hex_str.push_str(HEX_STRINGS[(byte & 0b1111) as usize]);
    }
    hex_str
}

pub fn xor_buffers(buf1: &[u8], buf2: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    for (a, b) in buf1.into_iter().zip(buf2.into_iter()) {
        result.push(a ^ b);
    }
    result
}

pub fn hamming_distance(buf1: &[u8], buf2: &[u8]) -> u8 {
    let xord = xor_buffers(buf1, buf2);
    let mut distance: u8 = 0;
    for i in xord {
        for shift in 0..7 {
            distance += (i >> shift) & 1;
        }
    }
    distance
}

pub fn chunkify<T>(source: &Vec<T>, size: usize) -> Vec<Vec<T>> where T: Copy {
    let mut chunks = Vec::new();
    for chunk in source.chunks(size) {
        chunks.push(Vec::from_iter(chunk.iter().map(|x| *x)));
    }
    chunks
}

pub fn transpose<T>(source: Vec<Vec<T>>) -> Vec<Vec<T>> where T: Copy {
    let mut transposed = Vec::new();
    for i in 0..source[0].len() {
        let mut trans_row = Vec::new();
        for row in source.iter() {
            if row.len() > i {
                trans_row.push(row[i]);
            }
        }
        transposed.push(trans_row);
    }
    transposed
}

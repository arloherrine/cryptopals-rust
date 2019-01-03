use bin_utils;
use char_frequency;
use crypto::{ symmetriccipher, buffer, aes, blockmodes };
use crypto::buffer::{ RefReadBuffer, RefWriteBuffer, ReadBuffer, WriteBuffer, BufferResult };
use rand;

pub fn solve_single_xor(cipher_text: &[u8]) -> u8 {
    let mut best_score = 0.0;
    let mut best_key = 0;
    for key in 0..u8::max_value() {
        let score = char_frequency::score_string(&single_xor(key, cipher_text));
        if score > best_score {
            best_score = score;
            best_key = key;
        }
    }
    best_key
}

pub fn single_xor(key: u8, text: &[u8]) -> Vec<u8> {
    bin_utils::xor_buffers(text, &vec![key; text.len()])
}

/*
pub struct CryptoKey {
    pub data: Vec<u8>,
}

impl CryptoKey {
    pub fn new(size: usize) -> CryptoKey {
        CryptoKey { data: vec![0; size] }
    }

    pub fn increment(&mut self) -> bool {
        self.increment_rec(0)
    }

    fn increment_rec(&mut self, index: usize) -> bool {
        let (new_value, overflow) = self.data[index].overflowing_add(1);
        self.data[index] = new_value;
        overflow && (index + 1 >= self.data.len() || self.increment_rec(index + 1))
    }
}
*/

pub fn decrypt_aes_ecb(data: &[u8], key: &[u8]) -> Vec<u8> {
    let mut decryptor = aes::ecb_decryptor(
            aes::KeySize::KeySize128,
            key,
            blockmodes::NoPadding);
    crypt(data, |read_buffer, write_buffer| decryptor.decrypt(read_buffer, write_buffer, true))
}

pub fn encrypt_aes_ecb(data: &[u8], key: &[u8]) -> Vec<u8> {
    let mut encryptor = aes::ecb_encryptor(
        aes::KeySize::KeySize128,
        key,
        blockmodes::NoPadding);
    crypt(&bin_utils::pkcs_pad(data, 16), |read_buffer, write_buffer| encryptor.encrypt(read_buffer, write_buffer, true))
}

pub fn crypt<F>(data: &[u8], mut crypt_op: F) -> Vec<u8>
        where F : FnMut(&mut RefReadBuffer, &mut RefWriteBuffer) -> Result<BufferResult, symmetriccipher::SymmetricCipherError> {
    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = crypt_op(&mut read_buffer, &mut write_buffer).expect("Crypt operation failed");
        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => { }
        }
    }

    final_result
}

pub fn cbc_decrypt<F>(data: &[u8], key: &[u8], iv: &[u8], cipher: F) -> Vec<u8>
        where F : Fn(&[u8], &[u8]) -> Vec<u8> {
    let mut last_block = iv;
    let mut result = Vec::new();
    for block in data.chunks(16) {
        result.extend(bin_utils::xor_buffers(&cipher(&block, key), last_block));
        last_block = block;
    }
    result
}

pub fn cbc_encrypt<F>(data: &[u8], key: &[u8], iv: &[u8], cipher: F) -> Vec<u8>
    where F : Fn(&[u8], &[u8]) -> Vec<u8> {
    let mut last_block = iv.to_vec();
    let mut result = Vec::new();
    for block in bin_utils::pkcs_pad(data, 16).chunks(16) {
        last_block = cipher(&bin_utils::xor_buffers(&last_block, block), key);
        result.extend(&last_block);
    }
    result
}

pub fn random_key(size: usize) -> Vec<u8> {
    (0..size).map(|_| rand::random()).collect()
}

pub fn ecb_cbc_rand_encrypt(data: &[u8]) -> Vec<u8> {
    let key = random_key(16);
    let mut fuzzed = Vec::new();
    fuzzed.extend(random_key((rand::random::<usize>() % 5) + 5));
    fuzzed.extend_from_slice(data);
    fuzzed.extend(random_key((rand::random::<usize>() % 5) + 5));
    if rand::random() {
        println!("Did   ecb");
        encrypt_aes_ecb(&fuzzed, &key)
    } else {
        println!("Did   cbc");
        cbc_encrypt(&fuzzed, &key, &random_key(16), encrypt_aes_ecb)
    }
}

pub fn strip_pkcs(mut data: Vec<u8>) -> Option<Vec<u8>> {
    let pad_byte = data[data.len() - 1];
    let len = data.len();
    if pad_byte != 0 && (pad_byte as usize) < len {
        let result = {
            let mut padding = data.drain((len - pad_byte as usize)..);
            padding.all(|b| b == pad_byte)
        };
        if result { Some(data) } else { None }
    } else {
        None
    }
}

pub fn ctr_crypt(data: &[u8], key: &[u8], nonce: &[u8]) -> Vec<u8> {
    let mut key_stream = CtrKeyStream::new(key, nonce);
    data.iter().map(|&byte| key_stream.next() ^ byte).collect()
}

pub fn ctr_crypt_edit(data: &[u8], key: &[u8], nonce: &[u8], offset: usize, newtext: &[u8]) -> Vec<u8> {
    let mut key_stream = CtrKeyStream::new(key, nonce);
    key_stream.seek(offset);
    let mut result = Vec::new();
    result.extend_from_slice(&data[0..offset]);
    result.extend(newtext.iter().map(|&byte| key_stream.next() ^ byte));
    result.extend_from_slice(&data[offset + newtext.len()..]);
    result
}

struct CtrKeyStream {
    key: Vec<u8>,
    counter: Vec<u8>,
    stream_buffer: Vec<u8>,
}

impl CtrKeyStream {
    fn new(key: &[u8], nonce: &[u8]) -> CtrKeyStream {
        let mut key_vec = Vec::new();
        key_vec.extend_from_slice(key);
        let mut nonce_vec = Vec::new();
        nonce_vec.extend_from_slice(nonce);
        CtrKeyStream {
            key: key_vec,
            counter: nonce_vec,
            stream_buffer: Vec::new(),
        }
    }

    fn next(&mut self) -> u8 {
        if self.stream_buffer.is_empty() {
            self.stream_buffer = encrypt_aes_ecb(&self.counter, &self.key);
            self.stream_buffer.reverse();
            for i in 0..16 {
                let index = (i + 8) % 16;
                let (result, overflow) = self.counter[index].overflowing_add(1);
                self.counter[index] = result;
                if !overflow {
                    break;
                }
            }
        }
        self.stream_buffer.pop().unwrap()
    }

    fn seek(&mut self, size: usize) {
        for _ in 0..size {
            self.next();
        }
    }
}

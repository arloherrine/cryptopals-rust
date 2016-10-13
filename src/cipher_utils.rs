use bin_utils;
use char_frequency;
use crypto::{ symmetriccipher, buffer, aes, blockmodes };
use crypto::buffer::{ RefReadBuffer, RefWriteBuffer, ReadBuffer, WriteBuffer, BufferResult };

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

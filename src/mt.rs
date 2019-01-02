use std::time::{SystemTime, UNIX_EPOCH};

const W: u32 = 32;
const N: usize = 624;
const M: usize = 397;
//const R: u32 = 31;
const A: u32 = 0x9908B0DF;
const U: u32 = 11;
const D: u32 = 0xFFFFFFFF;
const S: u32 = 7;
const B: u32 = 0x9D2C5680;
const T: u32 = 15;
const C: u32 = 0xEFC60000;
const L: u32 = 18;
const F: u32 = 1812433253;

const LOWER_MASK: u32 = 0x7FFFFFFF; // (1 << R) - 1 -> That is, the binary number of R 1's
const UPPER_MASK: u32 = 0x80000000; // lowest W bits of not LOWER_MASK

pub struct Gen {
    state: [u32; N],
    index: usize,
}

pub fn current_time_millis() -> u32 {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_secs() as u32 * 1000 + 
            since_the_epoch.subsec_millis() as u32
        /*
    let t = time::get_time();
    let i_millis = (t.sec * 1000) + ((t.nsec as i64) / 1000);
    (i_millis & (((1 as i64) << 32) - 1)) as u32
    */
}

pub fn untemper(&y: &u32) -> u32 {
    let mut y = y;
    let mut xor_mask = y >> L;
    while xor_mask != 0 {
        y = y ^ xor_mask;
        xor_mask = xor_mask >> L;
    }

    xor_mask = (y << T) & C;
    while xor_mask != 0 {
        y = y ^ xor_mask;
        xor_mask = (xor_mask << T) & C;
    }

    xor_mask = (y << S) & B;
    while xor_mask != 0 {
        y = y ^ xor_mask;
        xor_mask = (xor_mask << S) & B;
    }

    let mut xor_mask = (y >> U) & D;
    while xor_mask != 0 {
        y = y ^ xor_mask;
        xor_mask = (xor_mask >> U) & D;
    }
    y
}

impl Gen {
    pub fn new() -> Gen {
        Gen::with_seed(current_time_millis())
    }

    pub fn with_seed(seed: u32) -> Gen {
        let mut state = [seed; N];
        for i in 1..N {
            state[i] = F.wrapping_mul(state[i.wrapping_sub(1)] ^ (state[i.wrapping_sub(1)] >> (W-2)).wrapping_add(i as u32));
        }
        Gen { state:state, index:N }
    }

    pub fn clone(state: &[u32]) -> Gen {
        if state.len() != N {
            panic!("Can't clone with invalid length state slice");
        }
        let mut new_state = [0 as u32; N];
        for (i, element) in state.iter().enumerate() {
            new_state[i] = *element;
        }
        Gen { state:new_state, index:N }
    }

    fn twist(&mut self) {
        for i in 0..N {
            let x = (self.state[i] & UPPER_MASK).wrapping_add(self.state[(i.wrapping_add(1)) % N] & LOWER_MASK);
            let x_a = if x & 1 == 1 { (x >> 1) ^ A} else { x >> 1 };
            self.state[i] = self.state[(i.wrapping_add(M)) % N] ^ x_a;
        }
        self.index = 0;
    }

    pub fn next_u32(&mut self) -> u32 {
        if self.index > N {
            panic!("Not seeded properly");
        }
        if self.index == N {
            self.twist();
        }

        let mut y = self.state[self.index];
        y = y ^ ((y >> U) & D);
        y = y ^ ((y << S) & B);
        y = y ^ ((y << T) & C);
        y = y ^ (y >> L);

        self.index += 1;
        y
    }
}

pub struct WordStream {
    generator: Gen,
}

impl WordStream {
    pub fn with_seed(seed: u16) -> WordStream {
        WordStream { generator: Gen::with_seed(seed as u32) }
    }
}

impl Iterator for WordStream {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        Some(self.generator.next_u32())
    }
}

pub struct ByteStream {
    generator: Gen,
    index: u8,
    buffer: u32,
}

impl ByteStream {
    pub fn with_seed(seed: u16) -> ByteStream {
        ByteStream { generator: Gen::with_seed(seed as u32), index: 3, buffer: 0 }
    }
}

impl Iterator for ByteStream {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        self.index += 1;
        if self.index > 3 {
            self.index = 0;
            self.buffer = self.generator.next_u32();
        }
        Some(((self.buffer >> (8 * (3 - self.index))) & 0xFF) as u8)
    }
}

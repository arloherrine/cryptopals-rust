use time;

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
    let t = time::get_time();
    let i_millis = (t.sec * 1000) + ((t.nsec as i64) / 1000);
    (i_millis & (((1 as i64) << 32) - 1)) as u32
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

use mt;

pub fn main() {
    let mut gen1 = mt::Gen::with_seed(12352);
    println!("From seed 12352: {}, {}, {}, {}, {}", gen1.next_u32(), gen1.next_u32(), gen1.next_u32(), gen1.next_u32(), gen1.next_u32());
    let mut gen2 = mt::Gen::with_seed(12352);
    println!("From seed 12352: {}, {}, {}, {}, {}", gen2.next_u32(), gen2.next_u32(), gen2.next_u32(), gen2.next_u32(), gen2.next_u32());
    let mut gen3 = mt::Gen::with_seed(453734);
    println!("From seed 453734: {}, {}, {}, {}, {}", gen3.next_u32(), gen3.next_u32(), gen3.next_u32(), gen3.next_u32(), gen3.next_u32());
}

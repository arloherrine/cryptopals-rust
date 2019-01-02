use mt;

pub fn main() {
    let mut generator = mt::Gen::new();
    let mut outputs = Vec::new();
    for _ in 0..624 {
        outputs.push(generator.next_u32());
    }

    let state: Vec<u32> = outputs.iter().map(mt::untemper).collect();
    let mut spliced = mt::Gen::clone(&state);

    let real_outputs: Vec<u32> = (0..50).map(|_| generator.next_u32()).collect();
    let fake_outputs: Vec<u32> = (0..50).map(|_| spliced.next_u32()).collect();

    for n in real_outputs.iter() {
        print!("{:016x} ", n);
    }
    println!("");

    for n in fake_outputs.iter() {
        print!("{:016x} ", n);
    }
    println!("");

    assert_eq!(real_outputs, fake_outputs);
}

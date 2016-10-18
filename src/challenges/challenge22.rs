use mt;

use std::thread;
use std::time::Duration;
use rand;

pub fn main() {
    let start_time = mt::current_time_millis() + 40000;

    let target = get_target();

    let maybe_seed = (start_time..start_time + 1000000).find(|&seed| target == mt::Gen::with_seed(seed).next_u32());

    if let Some(seed) = maybe_seed {
        println!("found seed: {}", seed);
    } else {
        println!("failed to find seed");
    }
}

fn get_target() -> u32 {
    thread::sleep(Duration::from_secs((rand::random::<u64>() % 1000) + 40));
    let seed = mt::current_time_millis();
    let target = mt::Gen::with_seed(seed).next_u32();
    println!("true seed: {}", seed);
    thread::sleep(Duration::from_secs((rand::random::<u64>() % 100) + 40));
    target
}

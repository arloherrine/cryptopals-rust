use std::env;

mod bin_utils;
mod char_frequency;
mod cipher_utils;
mod challenges;

extern crate crypto;

fn main() {
    let challenges = [
        challenges::challenge1::main,
        challenges::challenge2::main,
        challenges::challenge3::main,
        challenges::challenge4::main,
        challenges::challenge5::main,
        challenges::challenge6::main,
        challenges::challenge7::main,
        challenges::challenge8::main,
    ];
    challenges[get_int_arg(1) - 1]();
}

fn get_int_arg(n: usize) -> usize {
    match env::args().nth(n) {
        Some(x) => match x.parse() {
            Ok(y) => y,
            Err(_) => unimplemented!(),
        },
        _ => unimplemented!(),
    }

}

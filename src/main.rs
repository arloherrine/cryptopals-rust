use std::env;

mod bin_utils;
mod char_frequency;
mod cipher_utils;
mod mt;
mod challenges;

extern crate crypto;
extern crate rand;

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
        challenges::challenge9::main,
        challenges::challenge10::main,
        challenges::challenge11::main,
        challenges::challenge12::main,
        challenges::challenge13::main,
        challenges::challenge14::main,
        challenges::challenge15::main,
        challenges::challenge16::main,
        challenges::challenge17::main,
        challenges::challenge18::main,
        challenges::challenge19::main,
        challenges::challenge20::main,
        challenges::challenge21::main,
        challenges::challenge22::main,
        challenges::challenge23::main,
        challenges::challenge24::main,
        challenges::challenge25::main,
        challenges::challenge26::main,
        challenges::challenge27::main,
        challenges::challenge28::main,
        challenges::challenge29::main,
        challenges::challenge30::main,
        challenges::challenge31::main,
        challenges::challenge32::main,
        challenges::challenge33::main,
        challenges::challenge34::main,
        challenges::challenge35::main,
        challenges::challenge36::main,
        challenges::challenge37::main,
        challenges::challenge38::main,
        challenges::challenge39::main,
        challenges::challenge40::main,
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

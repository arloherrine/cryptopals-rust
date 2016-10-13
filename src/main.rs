use std::env;

mod bin_utils;
mod char_frequency;
mod cipher_utils;
mod set1;

fn main() {
    let challenges = [
        [
            set1::challenge1::main,
            set1::challenge2::main,
            set1::challenge3::main,
            set1::challenge4::main,
            set1::challenge5::main,
            set1::challenge6::main,
        ],
    ];
    challenges[get_int_arg(1) - 1][get_int_arg(2) - 1]();
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

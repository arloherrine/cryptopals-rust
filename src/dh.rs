use rand;
use num_bigint::{BigUint, RandBigInt};

use cipher_utils;

lazy_static! {
    pub static ref P: BigUint = BigUint::from_bytes_be(b"\
            ffffffffffffffffc90fdaa22168c234c4c6628b80dc1cd129024\
            e088a67cc74020bbea63b139b22514a08798e3404ddef9519b3cd\
            3a431b302b0a6df25f14374fe1356d6d51c245e485b576625e7ec\
            6f44c42e9a637ed6b0bff5cb6f406b7edee386bfb5a899fa5ae9f\
            24117c4b1fe649286651ece45b3dc2007cb8a163bf0598da48361\
            c55d39a69163fa8fd24cf5f83655d23dca3ad961c62f356208552\
            bb9ed529077096966d670c354e4abc9804f1746c08ca237327fff\
            fffffffffffff");
    pub static ref G: BigUint = BigUint::from_bytes_be(b"2");
}

pub fn generate_standard_keys() -> (BigUint, BigUint) {
    generate_keys(&P, &G)
}

pub fn generate_keys(p: &BigUint, g: &BigUint) -> (BigUint, BigUint) {
    let mut rng = rand::thread_rng();
    let private = rng.gen_biguint_below(p);
    let public = g.modpow(&private, p);
    (private, public)
}

pub fn session_key(private: &BigUint, public: &BigUint, p: &BigUint) -> BigUint {
    public.modpow(private, p)
}

pub fn sha1_key(private: &BigUint, public: &BigUint, p: &BigUint) -> [u8; 20] {
    let session_key = session_key(private, public, p);
    cipher_utils::sha1_bytes(&session_key.to_bytes_be())
}

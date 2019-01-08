use num_bigint::BigUint;

use bin_utils;
use cipher_utils;
use dh;

pub fn main() {
    let mut a = MessageParty::new();
    let mut b = MessageParty::new();
    let (p, g, a_public) = a.init(dh::P.clone(), dh::G.clone());
    
    let b_public = b.init_ack(p, g, a_public);

    a.receive_ack(b_public);
    let (iv, data) = a.send("This be my secret message yo!!".to_string().as_bytes());

    let (iv, echoed) = b.reply(iv, &data);

    let plain = a.receive(&iv, &echoed);
    println!("A got: {}", bin_utils::bytes_to_ascii(&plain));

    let mut a = MessageParty::new();
    let mut b = MessageParty::new();
    let mut m = MimParty::new();

    let (p, g, a_public) = a.init(dh::P.clone(), dh::G.clone());
    let (p, g, fake_a_public) = m.init(p, g, a_public);

    let b_public = b.init_ack(p, g, fake_a_public);
    let fake_b_public = m.init_ack(b_public);

    a.receive_ack(fake_b_public);
    let (iv, data) = a.send("This is the secret message that's going to get intercepted :-(".to_string().as_bytes());
    let (iv, data, intercepted_a) = m.relay(iv, data);
    println!("Intercepted a->b: {}", bin_utils::bytes_to_ascii(&intercepted_a));

    let (iv, echoed) = b.reply(iv, &data);
    let (iv, echoed, intercepted_b) = m.relay(iv, echoed);
    println!("Intercepted b->a: {}", bin_utils::bytes_to_ascii(&intercepted_b));

    let plain = a.receive(&iv, &echoed);
    println!("A got: {}", bin_utils::bytes_to_ascii(&plain));
}

struct MessageParty {
    private: Option<BigUint>,
    other_public: Option<BigUint>,
    p: Option<BigUint>,
}

impl MessageParty {
    fn new() -> MessageParty {
        MessageParty {
            private: None,
            other_public: None,
            p: None,
        }
    }

    fn init(&mut self, p: BigUint, g: BigUint) -> (BigUint, BigUint, BigUint) {
        let (private, public) = dh::generate_keys(&p, &g);
        self.private = Some(private);
        self.p = Some(p.clone());
        (p, g, public)
    }

    fn init_ack(&mut self, p: BigUint, g: BigUint, other_public: BigUint) -> BigUint {
        let (private, public) = dh::generate_keys(&p, &g);
        self.private = Some(private);
        self.other_public = Some(other_public);
        self.p = Some(p.clone());
        public
    }

    fn receive_ack(&mut self, other_public: BigUint) {
        self.other_public = Some(other_public);
    }

    fn send(&self, message: &[u8]) -> (Vec<u8>, Vec<u8>) {
        let iv = cipher_utils::random_key(16);
        let private = self.private.as_ref().expect("Can't send with unset private key");
        let other_public = self.other_public.as_ref().expect("Can't send with unset other public key");
        let p = self.p.as_ref().expect("Can't send with unset p");
        let data = cipher_utils::cbc_encrypt(&message, &dh::sha1_key(private, other_public, p), &iv, cipher_utils::encrypt_aes_ecb);
        (iv, data)
    }

    fn reply(&self, iv: Vec<u8>, data: &[u8]) -> (Vec<u8>, Vec<u8>) {
        let plain = self.receive(&iv, data);
        self.send(&plain)
    }

    fn receive(&self, iv: &[u8], data: &[u8]) -> Vec<u8> {
        let private = self.private.as_ref().expect("Can't recieve with unset private key");
        let other_public = self.other_public.as_ref().expect("Can't recieve with unset other public key");
        let p = self.p.as_ref().expect("Can't recieve with unset p");
        cipher_utils::cbc_decrypt(data, &dh::sha1_key(&private, &other_public, p), iv, cipher_utils::decrypt_aes_ecb)
    }

}

struct MimParty {
    p: Option<BigUint>
}

impl MimParty {
    fn new() -> MimParty {
        MimParty {
            p: None,
        }
    }

    fn init(&mut self, p: BigUint, g: BigUint, _other_public: BigUint) -> (BigUint, BigUint, BigUint) {
        self.p = Some(p.clone());
        (p.clone(), g, p)
    }

    fn init_ack(&self, _other_public: BigUint) -> BigUint {
        self.p.as_ref().expect("Can't init ack without p").clone()
    }

    fn relay(&self, iv: Vec<u8>, data: Vec<u8>) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
        let intercepted = cipher_utils::cbc_decrypt(&data, &cipher_utils::sha1_bytes(&[0; 1]), &iv, cipher_utils::decrypt_aes_ecb);
        (iv, data, intercepted)
    }
}

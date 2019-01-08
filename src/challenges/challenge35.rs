use num_bigint::{BigUint, ToBigUint};

use bin_utils;
use cipher_utils;
use dh;

pub fn main() {
    let mut b = MessageParty::new();
    let mut a = MessageParty::start(dh::P.clone(), dh::G.clone());

    b = a.send_init(b);
    a = b.send_ack(a);
    b = a.send_public(b);
    a = b.send_public(a);
    let (b, b_received, iv) = a.send_message(b, "gimme some sweet sweet message, dude".to_string().as_bytes());
    let (_a, a_received, _) = b.send_message_iv(a, &b_received, iv);
    println!("B got: {}", bin_utils::bytes_to_ascii(&b_received));
    println!("A got: {}", bin_utils::bytes_to_ascii(&a_received));

    // g = 1
    let mut b = MessageParty::new();
    let mut a = MessageParty::start(dh::P.clone(), dh::G.clone());
    let mut m = MessageParty::UnInitializedAttacker(AttackType::G1);

    m = a.send_init(m);
    b = m.send_init(b);

    m = b.send_ack(m);
    a = m.send_ack(a);

    m = a.send_public(m);
    b = m.send_public(b);
    
    m = b.send_public(m);
    a = m.send_public(a);

    let (m, m_received, iv) = a.send_message(m, "First intercepted message".to_string().as_bytes());
    println!("M got from A: {}", bin_utils::bytes_to_ascii(&m_received));
    let (b, b_received, iv) = m.send_message_iv(b, &m_received, iv);
    println!("B got from M: {}", bin_utils::bytes_to_ascii(&b_received));

    let (m, m_received, iv) = b.send_message_iv(m, &b_received, iv);
    println!("M got from B: {}", bin_utils::bytes_to_ascii(&m_received));
    let (_a, a_received, _iv) = m.send_message_iv(a, &m_received, iv);
    println!("A got from M: {}", bin_utils::bytes_to_ascii(&a_received));

    // G = P
    let mut b = MessageParty::new();
    let mut a = MessageParty::start(dh::P.clone(), dh::G.clone());
    let mut m = MessageParty::UnInitializedAttacker(AttackType::GP);

    m = a.send_init(m);
    b = m.send_init(b);

    m = b.send_ack(m);
    a = m.send_ack(a);

    m = a.send_public(m);
    b = m.send_public(b);
    
    m = b.send_public(m);
    a = m.send_public(a);

    let (m, m_received, iv) = a.send_message(m, "Second intercepted message".to_string().as_bytes());
    println!("M got from A: {}", bin_utils::bytes_to_ascii(&m_received));
    let (b, b_received, iv) = m.send_message_iv(b, &m_received, iv);
    println!("B got from M: {}", bin_utils::bytes_to_ascii(&b_received));

    let (m, m_received, iv) = b.send_message_iv(m, &b_received, iv);
    println!("M got from B: {}", bin_utils::bytes_to_ascii(&m_received));
    let (_a, a_received, _iv) = m.send_message_iv(a, &m_received, iv);
    println!("A got from M: {}", bin_utils::bytes_to_ascii(&a_received));
}

enum MessageParty {
    UnInitialized,
    HasParams(BigUint, BigUint), // p, g
    GeneratedKeys(BigUint, BigUint, BigUint), // p, private key, my public key
    Ready(BigUint, BigUint, BigUint, BigUint), // p, private, my public key, other public key

    UnInitializedAttacker(AttackType),
    HasParamsAttacker(AttackType, BigUint, BigUint), // p, g
    HasKeyAttacker(AttackType, BigUint, BigUint), // p, public key
    HasMessageAttacker(AttackType, BigUint, Vec<u8>, Vec<u8>) // p, message, iv
}

enum AttackType {
    G1,
    GP,
    //GP1,
}

impl MessageParty {
    fn new() -> MessageParty {
        MessageParty::UnInitialized
    }

    fn start(p: BigUint, g: BigUint) -> MessageParty {
        MessageParty::HasParams(p, g)
    }

    fn send_init(&self, other: MessageParty) -> MessageParty {
        if let MessageParty::HasParams(p, g) = self {
            other.receive_init(&p, &g)
        } else if let MessageParty::HasParamsAttacker(_attack_type, p, g) = self {
            other.receive_init(&p, &g)
        } else {
            panic!("Bad state in send_init")
        }
    }

    fn receive_init(self, p: &BigUint, g: &BigUint) -> MessageParty {
        if let MessageParty::UnInitialized = self {
            MessageParty::HasParams(p.clone(), g.clone())
        } else if let MessageParty::UnInitializedAttacker(attack_type) = self {
            let attack_g = match attack_type {
                AttackType::G1 => 1.to_biguint().unwrap(),
                AttackType::GP => p.clone(),
                //AttackType::GP1 => p.clone() - 1 as u32,
            };
            MessageParty::HasParamsAttacker(attack_type, p.clone(), attack_g)
        } else {
            panic!("Bad state in receive_init")
        }
    }

    fn send_ack(&self, other: MessageParty) -> MessageParty {
        other.receive_ack()
    }

    fn receive_ack(self) -> MessageParty {
        if let MessageParty::HasParams(p, g) = self {
            let (private, public) = dh::generate_keys(&p, &g);
            MessageParty::GeneratedKeys(p, private, public)
        } else if let MessageParty::HasParamsAttacker(_, _, _) = self {
            self
        } else {
            panic!("Bad state in receive_ack")
        }
    }

    fn send_public(&self, other: MessageParty) -> MessageParty {
        if let MessageParty::GeneratedKeys(_, _, public) = self {
            other.receive_public(&public)
        } else if let MessageParty::Ready(_, _, public, _) = self {
            other.receive_public(&public)
        } else if let MessageParty::HasKeyAttacker(_attack_type, _, public) = self {
            other.receive_public(&public)
        } else {
            panic!("Bad state in send_public")
        }
    }

    fn receive_public(self, other_public: &BigUint) -> MessageParty {
        if let MessageParty::HasParams(p, g) = self {
            let (private, public) = dh::generate_keys(&p, &g);
            MessageParty::Ready(p, private, public, other_public.clone())
        } else if let MessageParty::GeneratedKeys(p, private, public) = self {
            MessageParty::Ready(p, private, public, other_public.clone())
        } else if let MessageParty::HasParamsAttacker(attack_type, p, _) = self {
            MessageParty::HasKeyAttacker(attack_type, p, other_public.clone())
        } else if let MessageParty::HasKeyAttacker(attack_type, p, _) = self {
            MessageParty::HasKeyAttacker(attack_type, p, other_public.clone())
        } else {
            panic!("Bad state in receive_public")
        }
    }

    fn send_message(&self, other: MessageParty, message: &[u8]) -> (MessageParty, Vec<u8>, Vec<u8>) {
        self.send_message_iv(other, message, cipher_utils::random_key(16))
    }

    fn send_message_iv(&self, other: MessageParty, message: &[u8], iv: Vec<u8>) -> (MessageParty, Vec<u8>, Vec<u8>) {
        if let MessageParty::Ready(p, private, _public, other_public) = self {
            let data = cipher_utils::cbc_encrypt(message, &dh::sha1_key(private, other_public, p), &iv, cipher_utils::encrypt_aes_ecb);
            other.receive_message(&data, iv)
        } else if let MessageParty::HasMessageAttacker(_attack_type, _, data, iv) = self {
            other.receive_message(&data, iv.to_vec())
        } else {
            panic!("Bad state in send_message")
        }
    }

    fn receive_message(self, data: &[u8], iv: Vec<u8>) -> (MessageParty, Vec<u8>, Vec<u8>) {
        if let MessageParty::Ready(p, private, public, other_public) = self {
            let decrypted = cipher_utils::cbc_decrypt(&data, &dh::sha1_key(&private, &other_public, &p), &iv, cipher_utils::decrypt_aes_ecb);
            (MessageParty::Ready(p, private, public, other_public), decrypted, iv)
        } else if let MessageParty::HasKeyAttacker(attack_type, p, _public) = self {
            let fake_key = match attack_type {
                AttackType::G1 => vec![1; 1],
                AttackType::GP => vec![0; 1],
                //AttackType::GP1 => p.to_bytes_be(), // TODO
            };
            let plain = cipher_utils::cbc_decrypt(&data, &cipher_utils::sha1_bytes(&fake_key), &iv, cipher_utils::decrypt_aes_ecb);
            (MessageParty::HasMessageAttacker(attack_type, p, data.to_vec(), iv.clone()), plain, iv)
        } else if let MessageParty::HasMessageAttacker(attack_type, p, _data, _iv) = self {
            let fake_key = match attack_type {
                AttackType::G1 => vec![1; 1],
                AttackType::GP => vec![0; 1],
                //AttackType::GP1 => p.to_bytes_be(), // TODO
            };
            let plain = cipher_utils::cbc_decrypt(&data, &cipher_utils::sha1_bytes(&fake_key), &iv, cipher_utils::decrypt_aes_ecb);
            (MessageParty::HasMessageAttacker(attack_type, p, data.to_vec(), iv.clone()), plain, iv)
        } else {
            panic!("Bad state in send_message")
        }
    }
}

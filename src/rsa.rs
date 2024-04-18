use crate::prime::Prime;
use crate::utils::{gcd, generate_random_biguint, mod_inverse};
use num_bigint::BigUint;
use num_traits::cast::ToPrimitive;

pub struct PublicKey {
    e: BigUint,
    n: BigUint,
}

impl PublicKey {
    pub fn e(&self) -> &BigUint {
        &self.e
    }

    pub fn n(&self) -> &BigUint {
        &self.n
    }
}

pub struct PrivateKey {
    d: BigUint,
}

impl PrivateKey {
    pub fn d(&self) -> &BigUint {
        &self.d
    }
}

pub struct RSA {
    pub_key: PublicKey,
    priv_key: PrivateKey,
}

impl RSA {
    pub fn new() -> RSA {
        let one = BigUint::from(1 as usize);
        let p = Prime::new(256, 4);
        let q = Prime::new(256, 4);
        let p_value = p.get().clone();
        let q_value = q.get().clone();

        let n = p_value.clone() * q_value.clone();
        let phi = (p_value - one.clone()) * (q_value - one.clone());

        let mut e;
        loop {
            e = generate_random_biguint(256);
            if gcd(&e, &phi) == one {
                break;
            }
        }

        let d = match mod_inverse(&e, &phi) {
            Some(d) => d,
            None => BigUint::from(0 as usize),
        };

        RSA {
            priv_key: PrivateKey { d },
            pub_key: PublicKey { e, n },
        }
    }

    pub fn get(&self) -> (BigUint, BigUint, BigUint) {
        (
            self.pub_key.e().clone(),
            self.pub_key.n().clone(),
            self.priv_key.d().clone(),
        )
    }

    pub fn encrypt(&self, msg: String) -> Vec<BigUint> {
        let msg_bytes = msg.as_bytes();

        let msg_num: Vec<BigUint> = msg_bytes.iter().map(|&b| BigUint::from(b)).collect();

        msg_num.iter().map(|num| self.encrypt_num(num)).collect()
    }

    pub fn decrypt(&self, encrypted_msg: Vec<BigUint>) -> String {
        let decrypted_num: Vec<BigUint> = encrypted_msg
            .iter()
            .map(|num| self.decrypt_num(num))
            .collect();

        let decrypted_bytes: Vec<u8> = decrypted_num
            .iter()
            .map(|num| num.to_u8().unwrap())
            .collect();

        String::from_utf8(decrypted_bytes).unwrap()
    }

    fn encrypt_num(&self, num: &BigUint) -> BigUint {
        num.modpow(self.pub_key.e(), self.pub_key.n())
    }

    fn decrypt_num(&self, num: &BigUint) -> BigUint {
        num.modpow(self.priv_key.d(), self.pub_key.n())
    }
}

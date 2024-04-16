use crate::utils::{generate_random_biguint, is_prime};
use num_bigint::BigUint;

pub struct Prime {
    number: BigUint,
}

impl Prime {
    pub fn new(bits: usize, rounds: usize) -> Prime {
        let mut num = generate_random_biguint(bits);
        while !is_prime(&num, rounds) {
            num = generate_random_biguint(bits);
        }
        Prime { number: num }
    }

    pub fn get(&self) -> &BigUint {
        &self.number
    }
}

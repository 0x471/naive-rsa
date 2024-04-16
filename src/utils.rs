use num_bigint::{BigUint, RandBigInt};
use rand::thread_rng;

pub fn miller_rabin(d: &BigUint, n: &BigUint) -> bool {
    let mut rng = thread_rng();
    let one = BigUint::from(1 as usize);
    let two = BigUint::from(2 as usize);

    let a = rng.gen_biguint_range(&two, &(n - &two));
    let mut x = a.modpow(d, n);
    if x == BigUint::from(1 as usize) || x == n - BigUint::from(1 as usize) {
        return true;
    }

    let mut d_iterator = d.clone();
    while d_iterator != n - &BigUint::from(1 as usize) {
        x = x.modpow(&two, n);
        d_iterator *= &two;

        if x == BigUint::from(1 as usize) {
            return false;
        }
        if x == n - BigUint::from(1 as usize) {
            return true;
        }
    }
    false
}

pub fn is_prime(n: &BigUint, k: usize) -> bool {
    let zero = BigUint::from(0 as usize) ;
    let one = BigUint::from(1 as usize);
    let two = BigUint::from(2 as usize);
    if n <= &one || n == &BigUint::from(4 as usize) {
        return false;
    }
    if n <= &BigUint::from(3 as usize)  {
        return true;
    }

    let mut d = n.clone() - one;
    while &d % &two == zero {
        d /= &two;
    }

    for _ in 0..k {
        if !miller_rabin(&d, n) {
            return false;
        }
    }
    true
}

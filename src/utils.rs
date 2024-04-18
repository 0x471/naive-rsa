use num_bigint::{BigInt, BigUint, RandBigInt, Sign};
use rand::thread_rng;

pub fn miller_rabin(d: &BigUint, n: &BigUint) -> bool {
    let mut rng = thread_rng();
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
    let zero = BigUint::from(0 as usize);
    let one = BigUint::from(1 as usize);
    let two = BigUint::from(2 as usize);
    if n <= &one || n == &BigUint::from(4 as usize) {
        return false;
    }
    if n <= &BigUint::from(3 as usize) {
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

pub fn generate_random_biguint(bits: usize) -> BigUint {
    let mut rng = thread_rng();
    rng.gen_biguint(bits)
}

pub fn gcd(a: &BigUint, b: &BigUint) -> BigUint {
    let zero = BigUint::from(0 as usize);
    let mut temp;
    let mut a = a.clone();
    let mut b = b.clone();
    loop {
        temp = a.clone() % b.clone();
        if temp == zero {
            return b.clone();
        }
        a = b.clone();
        b = temp;
    }
}

pub fn mod_inverse(a: &BigUint, modulus: &BigUint) -> Option<BigUint> {
    let zero = BigInt::from(0 as usize);
    let one = BigInt::from(1 as usize);

    let mut mn = (
        BigInt::from_biguint(Sign::Plus, modulus.clone()),
        BigInt::from_biguint(Sign::Plus, a.clone()),
    );
    let mut xy = (zero.clone(), one.clone());

    while mn.1 != zero {
        xy = (xy.1.clone(), xy.0.clone() - (&mn.0 / &mn.1) * xy.1.clone());
        mn = (mn.1.clone(), mn.0 % mn.1.clone());
    }

    if mn.0 > one {
        return None;
    }

    let resp = (xy.0 + BigInt::from(modulus.clone())) % BigInt::from(modulus.clone());
    Some(resp.to_biguint().unwrap())
}

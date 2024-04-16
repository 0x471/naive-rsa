use num_bigint::BigUint;
use naive_rsa::prime::Prime;
use naive_rsa::utils::is_prime;


fn main() {
    let prime_instance = Prime::new(256, 4);
    let value = prime_instance.get();
    println!(":{}", value);

    for n in 1..100 {
        if is_prime(&BigUint::from(n as usize), 4) {
            println!("{}", n);
        }
    }
}

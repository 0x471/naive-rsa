use num_bigint::BigUint;
use prime::Prime;
use utils::is_prime;

mod prime;
mod utils;

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

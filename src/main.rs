use num_bigint::BigUint;
use utils::is_prime;

mod utils;

fn main() {
    for  n in 1..100 {
        if is_prime(&BigUint::from(n as usize), 4) {
            println!("{}", n);
        }
    }
}

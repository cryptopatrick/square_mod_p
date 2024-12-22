use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::time::Instant;

fn main() {
    // Size matters so we're using a prime in the 256-bit space that is close
    // to the upper bound: 2^256.
    let modulus = BigUint::parse_bytes(
        b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF43",
        16,
    )
    .expect("Invalid modulus");

    let mut value = BigUint::one();
    let mut count = 0;

    // Perform modular squaring for 1 second
    let start = Instant::now();
    while start.elapsed().as_secs() < 1 {
        value = (&value * &value) % &modulus;
        count += 1;
    }

    println!("Modular Squarings Per Second: {}", count);
}

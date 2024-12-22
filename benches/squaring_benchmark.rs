use criterion::{black_box, criterion_group, criterion_main, Criterion};
use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::time::Instant;

fn count_number_of_squarings_in_one_second() -> u64 {
    let modulus = BigUint::parse_bytes(
        b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF43",
        16,
    )
    .expect("Invalid modulus");

    let mut value = BigUint::one();
    let mut count = 0;

    // Perform modular squaring until 1 second has elapsed
    let start = Instant::now();
    while start.elapsed().as_secs() < 1 {
        value = (&value * &value) % &modulus;
        count += 1;
    }

    count
}

fn benchmark_256bit_squarings_per_second(c: &mut Criterion) {
    c.bench_function("Squaring Benchmark", |b| {
        b.iter(|| {
            let count = count_number_of_squarings_in_one_second();
            criterion::black_box(count);
        });
    });
}

fn criterion_config() -> Criterion {
    Criterion::default()
        // Since our squaring program runs for 1 second we need to extend
        // the time that Criterion has to make measurements
        .measurement_time(std::time::Duration::from_secs(100))
        .sample_size(10) // Reduce sample count
}

criterion_group! {
    name = benches;
    config = criterion_config();
    targets = benchmark_256bit_squarings_per_second,
}
criterion_main!(benches);

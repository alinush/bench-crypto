use criterion::{black_box, criterion_group, criterion_main, Criterion};
use blstrs::Scalar;
use ff::Field;
use rand::thread_rng;

fn bench_blake3_64_bytes(c: &mut Criterion) {
    let input: [u8; 64] = [0xABu8; 64];
    
    c.bench_function("blake3_hash_64_bytes", |b| {
        b.iter(|| {
            blake3::hash(black_box(&input))
        })
    });
}

fn bench_field_multiplication(c: &mut Criterion) {
    let mut rng = thread_rng();
    let x = Scalar::random(&mut rng);
    let y = Scalar::random(&mut rng);
    
    c.bench_function("bls12381_scalar_field_multiplication", |b| {
        b.iter(|| {
            black_box(black_box(x) * black_box(y))
        })
    });
}

criterion_group!(benches, bench_blake3_64_bytes, bench_field_multiplication);
criterion_main!(benches);

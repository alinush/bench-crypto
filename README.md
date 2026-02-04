# bench-crypto

Benchmarks for common cryptographic operations using [Criterion](https://github.com/bheisler/criterion.rs).

## Operations Benchmarked

- **BLAKE3 hashing** - Hashing 64 bytes using the BLAKE3 algorithm
- **BLS12-381 scalar multiplication** - Field multiplication in the BLS12-381 scalar field

## Dependencies

- [blstrs](https://crates.io/crates/blstrs) - BLS12-381 implementation
- [blake3](https://crates.io/crates/blake3) - BLAKE3 hashing
- [criterion](https://crates.io/crates/criterion) - Benchmarking framework

## Running Benchmarks

```bash
cargo bench
```

This will run all benchmarks and output timing statistics. Results are also saved to `target/criterion/` for historical comparison.

To run a specific benchmark:

```bash
cargo bench -- blake3
cargo bench -- scalar_mul
```

To run benchmarks in single-threaded mode (useful for consistent, reproducible results):

```bash
./scripts/run-single-threaded-benches.sh
```

## Example Output

```
blake3_hash_64_bytes    time:   [42.xxx ns 42.xxx ns 42.xxx ns]
bls12381_scalar_mul     time:   [15.xxx ns 15.xxx ns 15.xxx ns]
```

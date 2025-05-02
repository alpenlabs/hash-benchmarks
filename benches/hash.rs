use bitcoin::hashes::{sha256, Hash};
use blake2::Blake2s256;
use blake3::hash;
use criterion::{criterion_main, criterion_group, Criterion, BenchmarkId};
use rand_chacha::ChaCha12Rng;
use rand::{RngCore, SeedableRng};
use sha2::{Sha256, Digest};
use sha3::Sha3_256;

// This defines the input sizes (in bytes) to benchmark
const BYTES: &[usize] = &[64, 128, 1024];

// The seed used to quickly generate input data; it is arbitrary
// This is consistent across all tested hash functions
const SEED: [u8; 32] = [0u8; 32];

// BLAKE2s with 256-bit output
fn blake2s(c: &mut Criterion) {
    let mut rng = ChaCha12Rng::from_seed(SEED);
    
    let mut group = c.benchmark_group("BLAKE2s");
    for bytes in BYTES {
        group.bench_with_input(BenchmarkId::from_parameter(bytes), bytes, |b, bytes| {
            // Set up input data
            let mut input = vec![0u8; *bytes];
            rng.fill_bytes(&mut input);
            
            // Hash
            b.iter(|| {
                Blake2s256::digest(&input);
            });
        });
    }
}

// BLAKE3 with 256-bit output
fn blake3(c: &mut Criterion) {
    let mut rng = ChaCha12Rng::from_seed(SEED);
    
    let mut group = c.benchmark_group("BLAKE3");
    for bytes in BYTES {
        group.bench_with_input(BenchmarkId::from_parameter(bytes), bytes, |b, bytes| {
            // Set up input data
            let mut input = vec![0u8; *bytes];
            rng.fill_bytes(&mut input);
            
            // Hash
            b.iter(|| {
                hash(&input);
            });
        });
    }
}

// SHA256, using the RustCrypto implementation
fn sha256_rustcrypto(c: &mut Criterion) {
    let mut rng = ChaCha12Rng::from_seed(SEED);
    
    let mut group = c.benchmark_group("SHA256/RustCrypto");
    for bytes in BYTES {
        group.bench_with_input(BenchmarkId::from_parameter(bytes), bytes, |b, bytes| {
            // Set up input data
            let mut input = vec![0u8; *bytes];
            rng.fill_bytes(&mut input);
            
            // Hash
            b.iter(|| {
                Sha256::digest(&input);
            });
        });
    }
}

// SHA256, using the Bitcoin implementation
fn sha256_bitcoin(c: &mut Criterion) {
    let mut rng = ChaCha12Rng::from_seed(SEED);
    
    let mut group = c.benchmark_group("SHA256/bitcoin");
    for bytes in BYTES {
        group.bench_with_input(BenchmarkId::from_parameter(bytes), bytes, |b, bytes| {
            // Set up input data
            let mut input = vec![0u8; *bytes];
            rng.fill_bytes(&mut input);
            
            // Hash
            b.iter(|| {
                sha256::Hash::hash(&input);
            });
        });
    }
}

// SHA3 with 256-bit output
fn sha3(c: &mut Criterion) {
    let mut rng = ChaCha12Rng::from_seed(SEED);
    
    let mut group = c.benchmark_group("SHA3");
    for bytes in BYTES {
        group.bench_with_input(BenchmarkId::from_parameter(bytes), bytes, |b, bytes| {
            // Set up input data
            let mut input = vec![0u8; *bytes];
            rng.fill_bytes(&mut input);
            
            // Hash
            b.iter(|| {
                Sha3_256::digest(&input);
            });
        });
    }
}

criterion_group!(benches, blake2s, blake3, sha256_rustcrypto, sha256_bitcoin, sha3);
criterion_main!(benches);

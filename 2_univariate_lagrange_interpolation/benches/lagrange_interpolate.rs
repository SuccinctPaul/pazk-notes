#[macro_use]
extern crate criterion;

use bls12_381::Scalar;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use ff::{Field, PrimeField};
use rand_core::OsRng;
use univariate_lagrange_interpolation::polynomial::Polynomial;

fn bench_lagrange_interpolate(c: &mut Criterion) {
    let MIN_K: u32 = std::env::var("DEGREE")
        .unwrap_or_else(|_| "16".to_string())
        .parse()
        .expect("Cannot parse DEGREE env var as u32");

    const MAX_K: u32 = 19;

    // values
    let max_n = 1 << MAX_K;
    let domain: Vec<Scalar> = (0..max_n).map(|i| Scalar::from_u128(i)).collect::<Vec<_>>();
    let values: Vec<Scalar> = (0..max_n)
        .map(|_| Scalar::random(OsRng))
        .collect::<Vec<_>>();

    let mut group = c.benchmark_group("lagrange_interpolate");

    for k in MIN_K..=MAX_K {
        let n: u128 = 1 << k;

        let x = &domain[..n];
        let y = &values[..n];

        group.bench_function(BenchmarkId::new("k", k), |b| {
            b.iter(|| Polynomial::lagrange_interpolate(x.clone(), y.clone()));
        });
    }

    group.finish();
}

criterion_group!(benches, bench_lagrange_interpolate);
criterion_main!(benches);

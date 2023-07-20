#[macro_use]
extern crate criterion;

use bls12_381::Scalar;
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use ff::{Field, PrimeField};
use rand_core::OsRng;
use univariate_lagrange_interpolation::polynomial::Polynomial;


fn bench_lagrange_interpolate(c: &mut Criterion) {
    let k: usize = std::env::var("DEGREE")
        .unwrap_or_else(|_| "4".to_string())
        .parse()
        .expect("Cannot parse DEGREE env var as u32");

    let n: u128 = 1 << k;

    // values
    let x = (0..n).map(|i| Scalar::from_u128(i)).collect::<Vec<_>>();
    let y = (0..n).map(|_| Scalar::random(OsRng)).collect::<Vec<_>>();

    let mut group = c.benchmark_group("lagrange_interpolate");

    group.bench_function(BenchmarkId::new("k", k), |b| {
        b.iter(||
            Polynomial::lagrange_interpolate(x.clone(),y.clone())
        );
    });



    group.finish();
}

criterion_group!(benches, bench_lagrange_interpolate);
criterion_main!(benches);
use criterion::BatchSize::SmallInput;
use criterion::{criterion_group, criterion_main, Criterion};
use rand::Rng;
use rand_pcg::Pcg64;
use rand_pcg::rand_core::SeedableRng;

fn sort_rev(c: &mut Criterion) {
    const SIZE: usize = 1 << 20;

    let mut group = c.benchmark_group("sort_rev");
    let vec: Vec<i32> = (0..SIZE as i32).rev().collect();

    group.bench_function("sort_rev_stable", |bencher| {
        bencher.iter_batched(
            || vec.clone(),
            |mut v| v.sort(),
            SmallInput
        )
    });

    group.bench_function("sort_rev_unstable", |bencher| {
        bencher.iter_batched(
            || vec.clone(),
            |mut v| v.sort_unstable(),
            SmallInput
        )
    });

    group.finish();
}

fn sort_sorted(c: &mut Criterion) {
    const SIZE: usize = 1 << 20;

    let mut group = c.benchmark_group("sort_sorted");
    let vec: Vec<i32> = (0..SIZE as i32).collect();

    group.bench_function("sort_sorted_stable", |bencher| {
        bencher.iter_batched(
            || vec.clone(),
            |mut v| v.sort(),
            SmallInput
        )
    });

    group.bench_function("sort_sorted_unstable", |bencher| {
        bencher.iter_batched(
            || vec.clone(),
            |mut v| v.sort_unstable(),
            SmallInput
        )
    });

    group.finish();
}

fn sort_random(c: &mut Criterion) {
    const SEED: u64 = 42;
    const SIZE: usize = 1 << 20;
    let mut rng = Pcg64::seed_from_u64(SEED);

    let mut group = c.benchmark_group("sort_random");

    let vec: Vec<u32> = (0..SIZE)
        .map(|_| rng.random::<u32>())
        .collect();

    group.bench_function("sort_random_stable", |bencher| {
        bencher.iter_batched(
            || vec.clone(),
            |mut v| v.sort(),
            SmallInput
        )
    });

    group.bench_function("sort_random_unstable", |bencher| {
        bencher.iter_batched(
            || vec.clone(),
            |mut v| v.sort_unstable(),
            SmallInput
        )
    });

    group.finish();
}

criterion_group!(benches, sort_rev, sort_sorted, sort_random);
criterion_main!(benches);
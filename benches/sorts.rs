use criterion::BatchSize::SmallInput;
use criterion::{criterion_group, criterion_main, Criterion};
use rand::Rng;
use rand_pcg::Pcg64;
use rand_pcg::rand_core::SeedableRng;
use software_testing_project::sorts::sorts::{heapsort, mergesort, quicksort, timsort};

fn sort_rev(c: &mut Criterion) {
    const SIZE: usize = 1 << 20;

    let vec: Vec<i32> = (0..SIZE as i32).rev().collect();

    let mut group = c.benchmark_group("sort_rev");

    group.bench_function("stable", |bencher| {
        bencher.iter_batched_ref(
            || vec.clone(),
            |v| v.sort(),
            SmallInput
        )
    });

    group.bench_function("unstable", |bencher| {
        bencher.iter_batched_ref(
            || vec.clone(),
            |v| v.sort_unstable(),
            SmallInput
        )
    });

    // commented out because it is O(n^2), slowest by far
    // group.bench_function("quicksort", |bencher| {
    //     bencher.iter_batched_ref(
    //         || vec.clone(),
    //         |mut v| quicksort(&mut v),
    //         SmallInput
    //     )
    // });

    group.bench_function("heapsort", |bencher| {
        bencher.iter_batched_ref(
            || vec.clone(),
            |v| heapsort(v),
            SmallInput
        )
    });

    group.bench_function("mergesort", |bencher| {
        bencher.iter_batched_ref(
            || vec.clone(),
            |v| mergesort(v),
            SmallInput
        )
    });

    group.bench_function("timsort", |bencher| {
        bencher.iter_batched_ref(
            || vec.clone(),
            |v| timsort(v),
            SmallInput
        )
    });

    group.finish();
}

fn sort_sorted(c: &mut Criterion) {
    const SIZE: usize = 1 << 20;

    let vec: Vec<i32> = (0..SIZE as i32).collect();

    let mut group = c.benchmark_group("sort_sorted");

    group.bench_function("stable", |bencher| {
        bencher.iter_batched_ref(
            || vec.clone(),
            |v| v.sort(),
            SmallInput
        )
    });

    group.bench_function("unstable", |bencher| {
        bencher.iter_batched_ref(
            || vec.clone(),
            |v| v.sort_unstable(),
            SmallInput
        )
    });

    // commented out because it is O(n^2), slowest by far
    // group.bench_function("quicksort", |bencher| {
    //     bencher.iter_batched_ref(
    //         || vec.clone(),
    //         |mut v| quicksort(&mut v),
    //         SmallInput
    //     )
    // });

    group.bench_function("heapsort", |bencher| {
        bencher.iter_batched_ref(
            || vec.clone(),
            |v| heapsort(v),
            SmallInput
        )
    });

    group.bench_function("mergesort", |bencher| {
        bencher.iter_batched_ref(
            || vec.clone(),
            |v| mergesort(v),
            SmallInput
        )
    });

    group.bench_function("timsort", |bencher| {
        bencher.iter_batched_ref(
            || vec.clone(),
            |v| timsort(v),
            SmallInput
        )
    });

    group.finish();
}

fn sort_random(c: &mut Criterion) {
    const SEED: u64 = 42;
    const SIZE: usize = 1 << 20;

    let mut rng = Pcg64::seed_from_u64(SEED);

    let vec: Vec<i32> = (0..SIZE)
        .map(|_| rng.random::<i32>())
        .collect();

    let mut group = c.benchmark_group("sort_random");

    group.bench_function("stable", |bencher| {
        bencher.iter_batched_ref(
            || vec.clone(),
            |v| v.sort(),
            SmallInput
        )
    });

    group.bench_function("unstable", |bencher| {
        bencher.iter_batched_ref(
            || vec.clone(),
            |v| v.sort_unstable(),
            SmallInput
        )
    });

    group.bench_function("quicksort", |bencher| {
        bencher.iter_batched_ref(
            || vec.clone(),
            |v| quicksort(v),
            SmallInput
        )
    });

    group.bench_function("heapsort", |bencher| {
        bencher.iter_batched_ref(
            || vec.clone(),
            |v| heapsort(v),
            SmallInput
        )
    });

    group.bench_function("mergesort", |bencher| {
        bencher.iter_batched_ref(
            || vec.clone(),
            |v| mergesort(v),
            SmallInput
        )
    });

    group.bench_function("timsort", |bencher| {
        bencher.iter_batched_ref(
            || vec.clone(),
            |v| timsort(v),
            SmallInput
        )
    });

    group.finish();
}

criterion_group!(benches, sort_rev, sort_sorted, sort_random);
criterion_main!(benches);

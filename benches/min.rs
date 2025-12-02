use std::sync::Arc;
use criterion::{criterion_group, criterion_main, Criterion};
use software_testing_project::min::{multi_threaded_min_fast, multi_threaded_min_slow};
use std::hint::black_box;

fn bench_multi_threaded_min(c: &mut Criterion) {
    let mut group = c.benchmark_group("multi_threaded_min");
    let vec: Arc<Vec<i32>> = Arc::new((0..1000000).collect());
    const NUM_THREADS: usize = 8;

    group.bench_function("multi_threaded_min_slow", |bencher| {
        bencher.iter(|| {
            multi_threaded_min_slow(black_box(&vec), NUM_THREADS).unwrap();
        })
    });

    group.bench_function("multi_threaded_min_fast", |bencher| {
        bencher.iter(|| {
            multi_threaded_min_fast(black_box(&vec), NUM_THREADS).unwrap();
        })
    });

    group.finish();
}

criterion_group!(benches, bench_multi_threaded_min);
criterion_main!(benches);

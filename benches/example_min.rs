use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use software_testing_project::example_min::{multi_threaded_min_fast, multi_threaded_min_slow};
use std::hint::black_box;

const MAX_THREADS: usize = 8;

fn bench_min_max_threads(c: &mut Criterion) {
    const VEC_SIZE: i32 = 1 << 21;
    
    let mut group = c.benchmark_group("multi_threaded_min");
    let vec: Vec<i32> = (0..VEC_SIZE).collect();

    group.bench_function("multi_threaded_min_slow", |bencher| {
        bencher.iter(|| {
            multi_threaded_min_slow(black_box(&vec), MAX_THREADS).unwrap();
        })
    });

    group.bench_function("multi_threaded_min_fast", |bencher| {
        bencher.iter(|| {
            multi_threaded_min_fast(black_box(&vec), MAX_THREADS).unwrap();
        })
    });

    group.finish();
}

fn multi_threaded_min_throughput(c: &mut Criterion) {
    const MIN_SIZE: i32 = 1 << 18;
    const MAX_SIZE: i32 = 1 << 22;
    const SIZE_STEP: usize = 1 << 18;

    let mut group = c.benchmark_group("multi_threaded_min_throughput");
    group.sample_size(10);

    for num_threads in 1..=MAX_THREADS {
        for size in (MIN_SIZE..=MAX_SIZE).step_by(SIZE_STEP) {
            let vec: Vec<i32> = (0..size).collect();

            let bytes = size << 2;
            group.throughput(Throughput::Bytes(bytes as u64));

            let bench_id_common = format!("({} size, {} threads)", size, num_threads);

            group.bench_with_input(
                BenchmarkId::new("multi_threaded_min_slow", &bench_id_common),
                &vec,
                |b, vec| {
                    b.iter(|| multi_threaded_min_slow(vec, num_threads));
                }
            );

            group.bench_with_input(
                BenchmarkId::new("multi_threaded_min_fast", &bench_id_common),
                &vec,
                |b, vec| {
                    b.iter(|| multi_threaded_min_fast(vec, num_threads));
                }
            );
        }
    }
    group.finish();
}

criterion_group!(benches, multi_threaded_min_throughput);
criterion_group!(benches1, bench_min_max_threads);

criterion_main!(benches, benches1);

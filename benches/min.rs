use criterion::{criterion_group, criterion_main, AxisScale, BenchmarkId, Criterion, PlotConfiguration};
use std::hint::black_box;
use software_testing_project::min::min::{multi_threaded_min_fast, multi_threaded_min_slow};

const MAX_THREADS: usize = 8;

fn min_with_max_threads(c: &mut Criterion) {
    const VEC_SIZE: i32 = 1 << 21;
    
    let mut group = c.benchmark_group("min_with_max_threads");
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

fn min_different_threads(c: &mut Criterion) {
    const MIN_SIZE: i32 = 1 << 19;
    const MAX_SIZE: i32 = 1 << 22;
    const SIZE_STEP: usize = 1 << 19;

    let mut group = c.benchmark_group("min_different_threads");
    group.sample_size(10);

    group.plot_config(
        PlotConfiguration::default()
            .summary_scale(AxisScale::Linear)
    );

    for size in (MIN_SIZE..=MAX_SIZE).step_by(SIZE_STEP) {
        let vec: Vec<i32> = (0..size).collect();

        for num_threads in 1..=MAX_THREADS {

            group.bench_function(
                BenchmarkId::new(format!("multi_threaded_min_slow_{size}"), num_threads),
                |bencher| {
                    bencher.iter(|| multi_threaded_min_slow(&vec, num_threads));
                }
            );

            group.bench_function(
                BenchmarkId::new(format!("multi_threaded_min_fast_{size}"), num_threads),
                |bencher| {
                    bencher.iter(|| multi_threaded_min_fast(&vec, num_threads));
                }
            );
        }
    }
    
    group.finish();
}

criterion_group!(group0, min_with_max_threads, min_different_threads);
criterion_main!(group0);

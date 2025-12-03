use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use criterion::BatchSize::SmallInput;
use software_testing_project;
use software_testing_project::state::State;

fn bench_total_time(c: &mut Criterion) {
    let board = vec![
        "   O   ",
        "   X   ",
        "   O X ",
        "   X O ",
        "  XO O ",
        "XXOXOX ",
    ];

    let evaluate_position = software_testing_project::cache_strategy::evaluate_position;

    let mut group = c.benchmark_group("total_time_group");
    group.sample_size(10);

    group.bench_function("evaluate_position", |bencher| {
        bencher.iter_batched(
            || State::encode(&board),
            |state| evaluate_position(black_box(state)),
            SmallInput,
        )
    });

    group.finish();
}

criterion_group!(benches, bench_total_time);
criterion_main!(benches);

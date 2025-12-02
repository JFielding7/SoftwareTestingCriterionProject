use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use criterion::BatchSize::SmallInput;
use software_testing_project::engine::evaluate_position;
use software_testing_project::state::State;

fn bench_connect4_simple(c: &mut Criterion) {
    let board = vec![
        "   O   ",
        "   X   ",
        "   O X ",
        "   X O ",
        "  XO O ",
        "XXOXOX ",
    ];

    let mut group = c.benchmark_group("connect4_simple");
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

criterion_group!(benches, bench_connect4_simple);
criterion_main!(benches);

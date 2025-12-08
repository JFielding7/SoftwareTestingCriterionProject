use criterion::{criterion_group, criterion_main, AxisScale, BenchmarkId, Criterion, PlotConfiguration};
use std::hint::black_box;
use criterion::BatchSize::SmallInput;
use software_testing_project::connect_four;
use software_testing_project::connect_four::state_array::StateArray;
use software_testing_project::connect_four::state::State;
use software_testing_project::connect_four::state_bitboard::StateBitboard;
use software_testing_project::connect_four::state_file::read_state_file;

const DEFAULT_DEPTH: usize = 15;


fn single_state_time(c: &mut Criterion) {
    let board = [
        "   O   ",
        "   X   ",
        "   O X ",
        "   X O ",
        "  XO O ",
        "XXOXOX ",
    ].map(|row| row.to_string()).into_iter().collect();

    type StateType = StateBitboard;

    let evaluate_position = connect_four::naive::evaluate_position;

    let mut group = c.benchmark_group("single_state_time");
    group.sample_size(10);

    group.bench_function("evaluate_position", |bencher| {
        bencher.iter_batched(
            || StateType::encode(&board),
            |state| {
                evaluate_position(black_box(state));
            },
            SmallInput,
        )
    });

    group.finish();
}

fn array_vs_bitboard_time(c: &mut Criterion) {

    let mut group = c.benchmark_group("array_vs_bitboard_time");
    group.sample_size(10);

    let array_states: Vec<StateArray> = read_state_file(DEFAULT_DEPTH).unwrap();

    group.bench_function("array", |bencher| {
        bencher.iter_batched(
            || array_states.clone(),
            |states| {
                for state in states {
                    connect_four::naive::evaluate_position(state);
                }
            },
            SmallInput
        )
    });

    let bitboard_states: Vec<StateBitboard> = read_state_file(DEFAULT_DEPTH).unwrap();

    group.bench_function("bitboard", |bencher| {
        bencher.iter_batched(
            || bitboard_states.clone(),
            |states| {
                for state in states {
                    connect_four::naive::evaluate_position(state);
                }
            },
            SmallInput
        )
    });

    group.finish();
}

fn single_depth_time(c: &mut Criterion) {
    const DEPTH: usize = DEFAULT_DEPTH;
    type StateType = StateBitboard;

    let states: Vec<StateType> = read_state_file(DEPTH).unwrap();

    let mut group = c.benchmark_group("single_depth_time");
    group.sample_size(10);

    group.bench_function("naive", |bencher| {
        bencher.iter_batched(
            || states.clone(),
            |cloned_states| {
                for state in black_box(cloned_states) {
                    connect_four::naive::evaluate_position(state);
                }
            },
            SmallInput
        )
    });

    group.bench_function("caching", |bencher| {
        bencher.iter_batched(
            || states.clone(),
            |cloned_states| {
                for state in black_box(cloned_states) {
                    connect_four::cache_strategy::evaluate_position(state);
                }
            },
            SmallInput
        )
    });

    group.bench_function("threads", |bencher| {
        bencher.iter_batched(
            || states.clone(),
            |cloned_states| {
                for state in black_box(cloned_states) {
                    connect_four::threads::evaluate_position(state);
                }
            },
            SmallInput
        )
    });

    group.finish();
}

fn multiple_depths_time(c: &mut Criterion) {
    const MIN_DEPTH: usize = DEFAULT_DEPTH;
    const MAX_DEPTH: usize = 30;

    type StateType = StateBitboard;

    let mut group = c.benchmark_group("multiple_depths_time");

    group.sample_size(10);

    group.plot_config(
        PlotConfiguration::default()
        .summary_scale(AxisScale::Logarithmic)
    );

    for depth in MIN_DEPTH..=MAX_DEPTH {
        let states: Vec<StateType> = read_state_file(depth).unwrap();

        group.bench_function(BenchmarkId::new("naive", depth), |bencher| {
            bencher.iter_batched(
                || states.clone(),
                |curr_states| {
                    for state in curr_states {
                        connect_four::naive::evaluate_position(black_box(state));
                    }
                },
                SmallInput
            )
        });

        group.bench_function(BenchmarkId::new("caching", depth), |bencher| {
            bencher.iter_batched(
                || states.clone(),
                |curr_states| {
                    for state in curr_states {
                        connect_four::cache_strategy::evaluate_position(black_box(state));
                    }
                },
                SmallInput
            )
        });

        group.bench_function(BenchmarkId::new("threads", depth), |bencher| {
            bencher.iter_batched(
                || states.clone(),
                |curr_states| {
                    for state in curr_states {
                        connect_four::threads::evaluate_position(black_box(state));
                    }
                },
                SmallInput
            )
        });
    }

    group.finish();
}

criterion_group!(benches, single_state_time, array_vs_bitboard_time, single_depth_time, multiple_depths_time);
criterion_main!(benches);

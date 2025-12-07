use criterion::{criterion_group, criterion_main, AxisScale, BenchmarkId, Criterion, PlotConfiguration};
use std::hint::black_box;
use criterion::BatchSize::SmallInput;
use software_testing_project::connect_four;
use software_testing_project::connect_four::state_array::StateArray;
use software_testing_project::connect_four::state::State;
use software_testing_project::connect_four::state_bitboard::StateBitboard;
use software_testing_project::connect_four::state_file::read_state_file;

fn example_state(c: &mut Criterion) {
    let board = [
        "   O   ",
        "   X   ",
        "   O X ",
        "   X O ",
        "  XO O ",
        "XXOXOX ",
    ].map(|row| row.to_string()).into_iter().collect();;

    type StateType = StateBitboard;

    let evaluate_position = connect_four::threads::evaluate_position;

    let mut group = c.benchmark_group("example_state");
    group.sample_size(10);

    group.bench_function("evaluate_position", |bencher| {
        bencher.iter_batched(
            || StateType::encode(&board),
            |state| {
                let ret = evaluate_position(black_box(state));
                println!("Eval: {}", ret.eval);
            },
            SmallInput,
        )
    });

    group.finish();
}

fn multiple_states_same_depth(c: &mut Criterion) {
    const DEPTH: usize = 12;
    type StateType = StateArray;

    let states: Vec<StateType> = read_state_file(DEPTH).unwrap();

    let evaluate_position = connect_four::threads::evaluate_position;

    let mut group = c.benchmark_group("multiple_states_same_depth");
    group.sample_size(10);

    group.bench_function("evaluate_position", |bencher| {
        bencher.iter_batched(
            || states.clone(),
            |cloned_states| {
                for state in black_box(cloned_states) {
                    evaluate_position(state);
                }
            },
            SmallInput
        )
    });

    group.finish();
}

fn multiple_depths(c: &mut Criterion) {
    const MIN_DEPTH: usize = 20;
    const MAX_DEPTH: usize = 30;

    type StateType = StateBitboard;
    let evaluate_position = connect_four::cache_strategy::evaluate_position;

    let mut group = c.benchmark_group("multiple_depths");

    group.sample_size(10);

    group.plot_config(
        PlotConfiguration::default()
        .summary_scale(AxisScale::Logarithmic)
    );

    for depth in MIN_DEPTH..=MAX_DEPTH {
        let states: Vec<StateType> = read_state_file(depth).unwrap();

        group.bench_function(BenchmarkId::new("evaluate_position", depth), |bencher| {
            bencher.iter_batched(
                || states.iter().map(|s| s.clone()).collect::<Vec<StateType>>(),
                |curr_states| {
                    for state in curr_states {
                        evaluate_position(black_box(state));
                    }
                },
                SmallInput
            )
        });
    }

    group.finish();
}

criterion_group!(benches, example_state, multiple_states_same_depth, multiple_depths);
criterion_main!(benches);

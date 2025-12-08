use gungraun::{main, library_benchmark_group, library_benchmark, LibraryBenchmarkConfig, Callgrind, CallgrindMetrics};
use std::hint::black_box;
use software_testing_project::connect_four;
use software_testing_project::connect_four::solver_util::EvaluatePositionReturn;
use software_testing_project::connect_four::state::State;
use software_testing_project::connect_four::state_array::StateArray;
use software_testing_project::connect_four::state_bitboard::StateBitboard;


type StateType = StateArray;

fn setup_input<S: State>() -> (S, fn(S) -> EvaluatePositionReturn) {
    let board = [
        "   O   ",
        "   X   ",
        "   O X ",
        "   X O ",
        "  XO O ",
        "XXOXOX ",
    ].map(|row| row.to_string()).into_iter().collect();

    let evaluate_position = connect_four::threads::evaluate_position;

    (S::encode(&board), evaluate_position)
}

#[library_benchmark]
#[bench::first(
    setup = setup_input::<StateType>,
)]
fn bench_single_state<S: State>(
    (state, evaluate_position): (S, fn(S) -> EvaluatePositionReturn)
) -> EvaluatePositionReturn {

    black_box(evaluate_position(state))
}

library_benchmark_group!(
    name = bench_single_state_group;
    benchmarks = bench_single_state
);

main!(
    config = LibraryBenchmarkConfig::default()
        .tool(Callgrind::default()
            .format([CallgrindMetrics::All])
        );
    library_benchmark_groups = bench_single_state_group
);

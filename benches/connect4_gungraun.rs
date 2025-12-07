use gungraun::{main, library_benchmark_group, library_benchmark, LibraryBenchmarkConfig, Callgrind, CallgrindMetrics};
use std::hint::black_box;
use software_testing_project::connect_four;
use software_testing_project::connect_four::solver_util::EvaluatePositionReturn;
use software_testing_project::connect_four::state::State;
use software_testing_project::connect_four::state_array::StateArray;
use software_testing_project::connect_four::state_bitboard::StateBitboard;


type StateType = StateArray;

fn create_state<S: State>() -> (S, fn(S) -> EvaluatePositionReturn) {
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

fn display_statistics(ret: EvaluatePositionReturn) {
    println!("Eval: {}", ret.eval);
    println!("States Evaluated: {}", ret.states_evaluated);
}

#[library_benchmark]
#[bench::first(
    setup = create_state::<StateType>,
    teardown = display_statistics
)]
fn bench_example_state<S: State>(
    (state, evaluate_position): (S, fn(S) -> EvaluatePositionReturn)
) -> EvaluatePositionReturn {

    black_box(evaluate_position(state))
}

library_benchmark_group!(
    name = bench_example_state_group;
    benchmarks = bench_example_state
);

main!(
    config = LibraryBenchmarkConfig::default()
        .tool(Callgrind::default()
            .format([CallgrindMetrics::All])
        );
    library_benchmark_groups = bench_example_state_group
);

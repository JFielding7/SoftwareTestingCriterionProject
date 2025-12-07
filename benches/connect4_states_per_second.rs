use std::cell::Cell;
use std::hint::black_box;
use std::ops::Add;
use std::time::Instant;
use criterion::measurement::{Measurement, ValueFormatter};
use criterion::{criterion_group, criterion_main, AxisScale, BenchmarkId, Criterion, PlotConfiguration, Throughput};
use criterion::BatchSize::SmallInput;
use software_testing_project::connect_four;
use software_testing_project::connect_four::state_array::StateArray;
use software_testing_project::connect_four::state::State;
use software_testing_project::connect_four::state_bitboard::StateBitboard;
use software_testing_project::connect_four::state_file::read_state_file;

thread_local! {
    static STATES_EVALUATED: Cell<usize> = Cell::new(0);
}

pub fn reset_states_evaluated() {
    STATES_EVALUATED.set(0);
}

pub fn get_states_evaluated() -> usize {
    STATES_EVALUATED.get()
}

pub fn add_states_evaluated(state_count: usize) {
    STATES_EVALUATED.set(STATES_EVALUATED.get() + state_count);
}

struct SecondsPerState {
    seconds: f64,
    total_states: usize,
    iterations: usize,
}

impl SecondsPerState {
    fn new(
        seconds: f64,
        total_states: usize,
        iterations: usize,
    ) -> Self {
        Self {
            seconds,
            total_states,
            iterations
        }
    }

    fn zero() -> Self {
        Self {
            seconds: 0.0,
            total_states: 0,
            iterations: 0
        }
    }

    fn to_f64(&self) -> f64 {
        self.seconds * self.iterations as f64 / self.total_states as f64
    }
}

impl Add<&SecondsPerState> for &SecondsPerState {
    type Output = SecondsPerState;

    fn add(self, other: &SecondsPerState) -> SecondsPerState {
        SecondsPerState::new(
            self.seconds + other.seconds,
            self.total_states + other.total_states,
            self.iterations + other.iterations
        )
    }
}

struct SecondsPerStateMeasurement;

impl Measurement for SecondsPerStateMeasurement {
    type Intermediate = Instant;
    type Value = SecondsPerState;

    fn start(&self) -> Self::Intermediate {
        reset_states_evaluated();
        Instant::now()
    }

    fn end(&self, start: Self::Intermediate) -> Self::Value {
        let states = get_states_evaluated();
        SecondsPerState::new(start.elapsed().as_secs_f64(), states, 1)
    }

    fn add(&self, v0: &Self::Value, v1: &Self::Value) -> Self::Value {
        v0 + v1
    }

    fn zero(&self) -> Self::Value {
        SecondsPerState::zero()
    }

    fn to_f64(&self, val: &Self::Value) -> f64 {
        val.to_f64()
    }

    fn formatter(&self) -> &dyn ValueFormatter {
        &StatesPerSecondFormatter
    }
}

struct StatesPerSecondFormatter;

impl ValueFormatter for StatesPerSecondFormatter {
    fn scale_values(&self, typical_value: f64, values: &mut [f64]) -> &'static str {
        for v in values.iter_mut() {
            if *v != 0.0 {
                *v = 1.0 / *v;
            }
        }

        let positions_per_second = 1.0 / typical_value;

        if positions_per_second >= 1e6 {
            for v in values {
                *v /= 1e6;
            }

            "M states per second"
        } else if positions_per_second >= 1e3 {
            for v in values {
                *v /= 1e3;
            }

            "K states per second"
        } else {
            "states per second"
        }
    }

    fn scale_throughputs(&self, _: f64, _: &Throughput, _: &mut [f64]) -> &'static str {
        ""
    }

    fn scale_for_machines(&self, _: &mut [f64]) -> &'static str {
        ""
    }
}

fn example_sps(c: &mut Criterion<SecondsPerStateMeasurement>) {
    let board = [
        "   O   ",
        "   X   ",
        "   O X ",
        "   X O ",
        "  XO O ",
        "XXOXOX ",
    ].map(|row| row.to_string()).into_iter().collect();

    type StateType = StateBitboard;
    let evaluate_position = connect_four::cache_strategy::evaluate_position;

    let mut group = c.benchmark_group("example_sps");
    group.sample_size(10);

    group.bench_function("evaluate_position", |bencher| {
        bencher.iter_batched(
            || StateType::encode(&board),
            |state| {
                let ret = evaluate_position(black_box(state));
                add_states_evaluated(ret.states_evaluated);

                println!("States Evaluated: {}", ret.states_evaluated);
                println!("Total Eval: {}", ret.eval);
            },
            SmallInput,
        )
    });

    group.finish();
}

fn array_vs_bitboard_sps(c: &mut Criterion<SecondsPerStateMeasurement>) {
    let board = [
        "   O   ",
        "   X   ",
        "   O X ",
        "   X O ",
        "  XO O ",
        "XXOXOX ",
    ].map(|row| row.to_string()).into_iter().collect();

    let mut group = c.benchmark_group("array_vs_bitboard_pps");
    group.sample_size(10);

    let evaluate_position_array = connect_four::naive::evaluate_position;

    group.bench_function("array", |bencher| {
        bencher.iter_batched(
            || StateArray::encode(&board),
            |state| {
                let ret = evaluate_position_array(black_box(state));
                add_states_evaluated(ret.states_evaluated);

                println!("States Evaluated: {}", ret.states_evaluated);
                println!("Total Eval: {}", ret.eval);
            },
            SmallInput,
        )
    });

    let evaluate_position_bitboard = connect_four::naive::evaluate_position;

    group.bench_function("bitboard", |bencher| {
        bencher.iter_batched(
            || StateBitboard::encode(&board),
            |state| {
                let ret = evaluate_position_bitboard(black_box(state));
                add_states_evaluated(ret.states_evaluated);

                println!("States Evaluated: {}", ret.states_evaluated);
                println!("Total Eval: {}", ret.eval);
            },
            SmallInput,
        )
    });

    group.finish();
}

fn multiple_depths_sps(c: &mut Criterion<SecondsPerStateMeasurement>) {
    const MIN_DEPTH: usize = 20;
    const MAX_DEPTH: usize = 30;

    type StateType = StateBitboard;
    let evaluate_position = connect_four::cache_strategy::evaluate_position;

    let mut group = c.benchmark_group("multiple_depths_sps");

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

criterion_group! {
    name = benches;
    config = Criterion::default().with_measurement(SecondsPerStateMeasurement);
    targets = example_sps, array_vs_bitboard_sps, multiple_depths_sps
}

criterion_main!(benches);

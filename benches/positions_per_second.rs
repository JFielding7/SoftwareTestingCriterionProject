use std::hint::black_box;
use std::time::Instant;
use criterion::measurement::{Measurement, ValueFormatter};
use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use criterion::BatchSize::SmallInput;
use software_testing_project::state::State;

static mut STATES_EVALUATED: usize = 0;

struct SecondsPerState;

impl Measurement for SecondsPerState {
    type Intermediate = Instant;
    type Value = f64;

    fn start(&self) -> Self::Intermediate {
        unsafe { STATES_EVALUATED = 0 }
        Instant::now()
    }

    fn end(&self, start: Self::Intermediate) -> Self::Value {
        let states = unsafe { STATES_EVALUATED };
        start.elapsed().as_secs_f64() / states as f64
    }

    fn add(&self, v0: &Self::Value, v1: &Self::Value) -> Self::Value {
        *v0 + *v1
    }

    fn zero(&self) -> Self::Value {
        0.0
    }

    fn to_f64(&self, val: &Self::Value) -> f64 {
        *val
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

    fn scale_throughputs(&self, typical_value: f64, throughput: &Throughput, values: &mut [f64]) -> &'static str {
        ""
    }

    fn scale_for_machines(&self, values: &mut [f64]) -> &'static str {
        ""
    }
}

fn bench_positions_per_second(c: &mut Criterion<SecondsPerState>) {
    let board = vec![
        "   O   ",
        "   X   ",
        "   O X ",
        "   X O ",
        "  XO O ",
        "XXOXOX ",
    ];

    let evaluate_position = software_testing_project::cache_strategy::evaluate_position;

    let mut group = c.benchmark_group("positions_per_second_group");
    group.sample_size(10);

    group.bench_function("evaluate_position", |bencher| {
        bencher.iter_batched(
            || State::encode(&board),
            |state| {
                let ret = evaluate_position(black_box(state));
                unsafe { STATES_EVALUATED += ret.positions_evaluated }
                println!("{}", ret.positions_evaluated);
                println!("{}", ret.eval);
            },
            SmallInput,
        )
    });

    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default().with_measurement(SecondsPerState);
    targets = bench_positions_per_second
}

criterion_main!(benches);

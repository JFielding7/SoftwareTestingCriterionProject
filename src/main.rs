use std::io;
use software_testing_project::sorts::sorts::{heapsort, mergesort, quicksort, timsort};
use rand::{rng, Rng};
use rand_pcg::Pcg64;

mod connect_four;

fn main() -> io::Result<()> {
    let mut vec: Vec<i32> = (0..10000000)
        .map(|_| rng().random::<i32>())
        .collect();

    timsort(&mut vec);

    println!("{:?}", vec.is_sorted());

    Ok(())
}

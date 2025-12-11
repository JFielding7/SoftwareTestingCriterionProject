use std::io;
use software_testing_project::sorts::sorts::{heapsort, mergesort, quicksort, timsort};
use rand::{rng, Rng};
use rand_pcg::Pcg64;
use crate::connect_four::state_file::generate_state_file;

mod connect_four;

fn main() -> io::Result<()> {
    // let mut vec: Vec<i32> = (0..10000000)
    //     .map(|_| rng().random::<i32>())
    //     .collect();
    //
    // mergesort(&mut vec);
    //
    // println!("{:?}", vec.is_sorted());

    for i in 16..31 {
        generate_state_file(i)?;
    }

    Ok(())
}

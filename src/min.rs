use std::cmp::min;
use std::sync::{Arc, Mutex};
use std::thread;

fn single_threaded_min(vec: &[i32]) -> i32 {
    let mut curr_min = i32::MAX;

    for &num in vec.iter() {
        curr_min = min(curr_min, num);
    }

    curr_min
}

pub fn multi_threaded_min_slow(vec: &Vec<i32>, num_threads: usize) -> thread::Result<i32> {

    let mut segment_size = vec.len() / num_threads;
    if vec.len() % num_threads != 0 {
        segment_size += 1;
    }
    let segments: Vec<&[i32]> = vec.chunks(segment_size).collect();

    let overall_min = Arc::new(Mutex::new(i32::MAX));

    thread::scope(|scope| {
        for segment in segments {
            let overall_min_clone = overall_min.clone();

            scope.spawn(move || {
                for num in segment.iter() {
                    let mut overall_min_lock = overall_min_clone.lock().unwrap();
                    *overall_min_lock = min(*overall_min_lock, *num);
                }
            });
        }
    });

    Ok(*overall_min.lock().unwrap())
}

pub fn multi_threaded_min_fast(vec: &Vec<i32>, num_threads: usize) -> thread::Result<i32> {

    let mut segment_size = vec.len() / num_threads;
    if vec.len() % num_threads != 0 {
        segment_size += 1;
    }
    let segments: Vec<&[i32]> = vec.chunks(segment_size).collect();

    let overall_min = Arc::new(Mutex::new(i32::MAX));

    thread::scope(|scope| {
        for segment in segments {
            let overall_min_ref= overall_min.clone();

            scope.spawn(move || {
                let curr_min = single_threaded_min(segment);

                let mut overall_min_lock = overall_min_ref.lock().unwrap();
                *overall_min_lock = min(*overall_min_lock, curr_min);
            });
        }
    });

    Ok(*overall_min.lock().unwrap())
}

use halgorithms::{shuffle, sort};
use std::time::{Duration, Instant};

fn stopwatch<T: Ord + Clone>(f: fn(&mut [T]), slice: &[T], iterations: u32, name: &str) {
    let mut total_duration = Duration::new(0, 0);
    for _ in 0..iterations {
        let mut clone = slice.to_vec();
        shuffle::knuth_shuffle(&mut clone);
        
        let start = Instant::now();
        f(&mut clone);
        let duration = start.elapsed();
        total_duration += duration;
    }
    println!("{name} on {} items, total duration = {:?}, average duration = {:?}", slice.len(), total_duration, total_duration / iterations);
}

fn run_sorts<T: Ord + Clone>(slice: &[T], iterations: u32) {
    stopwatch(<[T]>::sort, slice, iterations, "slice sort");
    stopwatch(sort::selection_sort, slice, iterations, "selection sort");
    stopwatch(sort::insertion_sort, slice, iterations, "insertion sort");
    stopwatch(sort::shell_sort, slice, iterations, "shell sort");
}

#[test]
#[ignore]
fn sort_seven_i32_benchmark() {
    let vec = vec![0, 1, 2, 3, 4, 5, 6];
    let iterations = 10_000;
    run_sorts(&vec, iterations);
}

#[test]
#[ignore]
fn sort_twenty_i32_benchmark() {
    let vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18 ,19];
    let iterations = 10_000;
    run_sorts(&vec, iterations);
}

#[test]
#[ignore]
fn sort_one_hundred_i32_benchmark() {
    let mut vec = Vec::with_capacity(100);
    for i in 0..100 {
        vec.push(i);
    }
    let iterations = 10_000;
    run_sorts(&vec, iterations);
}
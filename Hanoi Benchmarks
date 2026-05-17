use std::time::Instant;

use tower_of_hanoi::{solve_iterative, solve_recursive};

fn time_run<F: Fn(u32) -> Vec<(u32, u32)>>(f: F, n: u32) -> (usize, u128) {
    let start = Instant::now();
    let moves = f(n);
    let dur = start.elapsed().as_micros();
    (moves.len(), dur)
}

fn main() {
    let n = std::env::args()
        .nth(1)
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(16);

    println!("Benchmarking Tower of Hanoi for {} disks", n);

    let (count_rec, dur_rec) = time_run(solve_recursive, n);
    println!("Recursive: {} moves, {} µs", count_rec, dur_rec);

    let (count_it, dur_it) = time_run(solve_iterative, n);
    println!("Iterative: {} moves, {} µs", count_it, dur_it);
}

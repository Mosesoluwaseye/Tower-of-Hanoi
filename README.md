Tower of Hanoi (Rust)

Simple, well-tested Rust implementations of the Tower of Hanoi puzzle.

Description:

This repository contains two implementations of the Tower of Hanoi problem in Rust: a classical recursive solver and an iterative solver. Both implementations return the sequence of moves as a Vec<(u32, u32)> (source peg, destination peg) which makes them easy to test and benchmark.

Implementations

Recursive: straightforward recursive algorithm that follows the mathematical definition.
Iterative: loop-based implementation that simulates legal moves without recursion (useful for benchmarking and avoiding call-stack overhead).
Complexity

Time: O(2^n) — both implementations produce the same number of moves (2^n - 1).
Space: recursive uses O(n) stack depth; iterative stores moves O(2^n) if you materialize them.
Project Layout

Cargo.toml
README.md
src/
  ├─ lib.rs          # core algorithms (solve_recursive, solve_iterative)
  ├─ main.rs         # CLI binary (select mode: rec/iter)
  └─ bin/
	  └─ hanoi_benchmark.rs
tests/
  └─ hanoi_test.rs
See the implementation files:

src/lib.rs
src/main.rs
src/bin/hanoi_benchmark.rs
tests/hanoi_test.rs
Usage

Build and run the CLI binary (default uses recursive solver):

cargo run --bin tower_of_hanoi -- 4 rec
Run iterative mode:

cargo run --bin tower_of_hanoi -- 4 iter
The first argument is the number of disks; the second optional argument selects rec (recursive) or iter (iterative).

Benchmark

Compare implementations using the provided benchmark binary:

cargo run --bin hanoi_benchmark -- 16
Testing

Run unit/integration tests:

cargo test
Example output

For n = 3 the program prints 7 moves. Example (recursive):

1: Move disk from 1 to 3
2: Move disk from 1 to 2
3: Move disk from 3 to 2
4: Move disk from 1 to 3
5: Move disk from 2 to 1
6: Move disk from 2 to 3
7: Move disk from 1 to 3
Notes

Moves are stored as (from, to) using peg indices 1 (A), 2 (B), 3 (C).
Materializing all moves is convenient for testing and benchmarking; printing directly is more memory-efficient for very large n.
For typical constraints (n ≤ 16) both implementations run quickly and recursion depth is safe.
If you want, I can also:

add a short example program that prints moves directly without storing them,
commit these files and create a git tag/release,
or run cargo test and the benchmark here.

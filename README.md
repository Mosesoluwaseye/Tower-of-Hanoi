# Tower of Hanoi — Rust Solver

## Overview

This repository contains a Rust implementation of the Tower of Hanoi puzzle.

The goal is to move `n` disks from peg `1` to peg `3` using peg `2` as auxiliary storage, while obeying these rules:

- Move only one disk at a time.
- Never place a larger disk on top of a smaller disk.

## What is included

- `Cses.rs`
  - A standalone Rust program that reads a single integer from standard input,
    generates the complete move sequence, prints the move count, and lists each move.
- `Hanoi Benchmarks`
  - A benchmark harness that compares recursive and iterative solver runtimes.
- `Hanoi_test.rs`
  - A test file that validates solver output and compares recursive vs iterative solutions.
- `Cargo.toml`
  - Project metadata and crate configuration.

## Solver Behavior

The implementation uses a vector of moves of type `Vec<(u32, u32)>`.
Each tuple represents a disk move from one peg to another.

The expected number of moves for `n` disks is `2^n - 1`.

## How to run

The simplest way to run the solver is to compile `Cses.rs` directly:

```bash
rustc Cses.rs -o hanoi_solver
printf "4\n" | ./hanoi_solver
```

This prints the minimum number of moves followed by each move in `from to` format.

## Benchmarking

The benchmark harness in `Hanoi Benchmarks` is designed to compare recursive and iterative implementations.
It assumes a `tower_of_hanoi` crate with `solve_recursive` and `solve_iterative` functions.

If the crate is available, run the benchmark with:

```bash
cargo run --bin hanoi_benchmark -- 16
```

## Testing

`Hanoi_test.rs` contains a validation test that ensures both recursive and iterative solutions:

- produce the same number of moves,
- generate legal Tower of Hanoi moves,
- solve the puzzle correctly for small values of `n`.

Run tests with:

```bash
cargo test
```

## Algorithm summary

- Recursive solver
  - Solve `n - 1` disks from source to auxiliary.
  - Move the largest disk from source to destination.
  - Solve `n - 1` disks from auxiliary to destination.
- Iterative solver
  - Generate moves using an explicit algorithm rather than deep recursion.

## Complexity

- Time complexity: O(2^n)
- Number of moves: `2^n - 1`
- Recursive memory: O(n) stack depth

## Notes

- `Cses.rs` is suitable for direct execution and CSES-style input/output.
- The benchmark/test files show how the solver can be extended into a crate-based Rust project.

## Author

Oluwaseye Moses

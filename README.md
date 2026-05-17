# Tower of Hanoi (Rust Implementation)

## Overview

This repository implements the **Tower of Hanoi** puzzle in Rust using two different approaches:

- a **recursive solution** that follows the classical mathematical definition,
- an **iterative solution** that avoids recursion and can be used for performance comparison.

The objective is to move `n` disks from a source peg to a destination peg using a helper peg, subject to these rules:

- Only one disk may be moved at a time.
- A larger disk cannot be placed on top of a smaller disk.

---

## Implementations

### 1. Recursive Approach

The recursive solution follows the classical definition:

1. Move `n-1` disks from the source peg to the helper peg.
2. Move the largest disk from the source peg to the destination peg.
3. Move the `n-1` disks from the helper peg to the destination peg.

This implementation is clean and easy to reason about, but it relies on the function call stack.

### 2. Iterative Approach

The iterative solution uses a loop-based strategy to generate the legal moves without recursion.

- Uses a loop-based method instead of recursive function calls.
- Avoids recursion overhead.
- Produces the same move sequence using an explicit simulation of peg movement.

This version is useful for benchmarking and understanding how implementation style affects runtime performance.

---

## Implementation Details

- Language: Rust
- Core logic: `src/lib.rs`
- Program execution: `src/main.rs`
- Benchmarking: `src/bin/hanoi_benchmark.rs`
- Testing: `tests/hanoi_test.rs`

Moves are stored in a `Vec<(u32, u32)>` so the output is easy to verify and benchmark.

---

## Complexity Analysis

Both implementations have the same theoretical complexity:

- **Time Complexity:** O(2^n)
- **Space Complexity:**
  - Recursive: O(n) due to recursion stack usage.
  - Iterative: O(2^n) if all moves are materialized in memory.

The number of required moves is fixed at `2^n - 1`.

---

## Benchmark Analysis

Benchmarks compare both implementations using the same move count.

Although the theoretical complexity is the same, runtime can differ:

- Recursive implementation pays function call overhead.
- Iterative implementation avoids recursive calls, which can be faster in practice.

This shows how **real-world performance** can vary even when asymptotic complexity is identical.

---

## Notes

- Storing moves in a `Vec` improves testability and modularity.
- In competitive programming environments (e.g., CSES), printing moves directly is more memory-efficient.
- For typical constraints (`n ≤ 16`), recursion depth is safe and stack overflow is unlikely.

---

## Usage

Build and run the CLI binary (default uses the recursive solver):

```bash
cargo run --bin tower_of_hanoi -- 4 rec
```

Run iterative mode:

```bash
cargo run --bin tower_of_hanoi -- 4 iter
```

The first argument is the number of disks. The second optional argument selects `rec` (recursive) or `iter` (iterative).

---

## Run Benchmark

```bash
cargo run --bin hanoi_benchmark -- 16
```

---

## Testing

```bash
cargo test
```

---

## Project Structure

```
Cargo.toml
README.md
src/
 ├── lib.rs
 ├── main.rs
 └── bin/
     └── hanoi_benchmark.rs

tests/
 └── hanoi_test.rs
```

---

## Author

Oluwaseye Moses

# aoc-2024
My advent of code solutions for 2024

# Building
A nightly version of rust is required, for debug builds:
```
cargo run -- <DAY>
```
If you want to run benchmarks:
```
RUSTFLAGS="-C target-cpu=native" cargo run --release -- <DAY> --bench
```
Multiple days can be run. Additionally, the `--all` flag will run all days.

# Input
Inputs are stored in the `input` folder in the working directory when running the binary.
The filenames must have the following format: `<DAY>.txt`.

# Benchmarks
Benchmark results on an M3 Macbook pro.
Times include "parsing", i.e. the benchmark measures from the moment the input file is in RAM.

| *Day* | *Part 1* | *Part 2* |
|-------|----------|----------|
|   1   |  5.41 µs |  2.75 µs |
|   2   |  4.12 µs |  10.5 µs |
|   3   |  3.10 µs |  3.04 µs |
|   4   |  31.3 µs |  21.6 µs |
|   5   |   156 µs |   231 µs |

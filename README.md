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
|   1   |  5.41 µs |  5.38 µs |
|   2   |   109 µs |   190 µs |
|   3   |  8.10 µs |  12.2 µs |

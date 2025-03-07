# TFHE/ORE benchmark

A small benchmark suite comparing similar operations in the homomorphic encryption library [tfhe](https://crates.io/crates/tfhe) to the order revealing encryption library [ore-rs](https://crates.io/crates/ore-rs).

## Running

Run the benchmarks using the following command:

```rs
cargo bench
```

Performance characteristics may change if you target your native CPU.
It might be useful to try running the following as well:

```rs
RUSTFLAGS="-Ctarget-cpu=native" cargo bench
```

## Results

```txt
Results from running on an Apple M2:

tfhe/encrypt            time:   [10.972 ms 11.316 ms 11.855 ms]
tfhe/a == b             time:   [148.64 ms 151.43 ms 154.80 ms]
tfhe/a > b              time:   [159.80 ms 161.58 ms 163.65 ms]
tfhe/a < b              time:   [162.39 ms 166.15 ms 170.87 ms]
tfhe/a >= a             time:   [24.159 ms 24.767 ms 25.546 ms]
tfhe/a <= a             time:   [28.279 ms 29.630 ms 31.094 ms]

ore/encrypt             time:   [372.42 µs 373.68 µs 375.16 µs]
ore/a == b              time:   [737.36 ns 755.64 ns 783.50 ns]
ore/a > b               time:   [740.85 ns 750.47 ns 769.78 ns]
ore/a < b               time:   [747.51 ns 755.29 ns 763.02 ns]
ore/a >= a              time:   [126.00 ns 130.67 ns 136.81 ns]
ore/a <= a              time:   [126.22 ns 129.91 ns 135.20 ns]
```

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
Results from running on an AMD Ryzen 9 3900x:

tfhe/encrypt            time:   [1.9684 ms 1.9721 ms 1.9755 ms]
tfhe/a == b             time:   [110.00 ms 111.21 ms 112.40 ms]
tfhe/a > b              time:   [188.50 ms 191.92 ms 195.34 ms]
tfhe/a < b              time:   [186.07 ms 189.58 ms 193.16 ms]
tfhe/a >= a             time:   [43.571 ms 43.980 ms 44.389 ms]
tfhe/a <= a             time:   [44.045 ms 44.498 ms 44.945 ms]

ore/encrypt             time:   [48.028 µs 48.154 µs 48.280 µs]
ore/a == b              time:   [238.17 ns 238.41 ns 238.68 ns]
ore/a > b               time:   [237.78 ns 238.03 ns 238.31 ns]
ore/a < b               time:   [239.17 ns 240.28 ns 241.57 ns]
ore/a >= a              time:   [221.17 ns 221.31 ns 221.46 ns]
ore/a <= a              time:   [225.81 ns 226.83 ns 227.72 ns]
```

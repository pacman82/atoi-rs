# Contributing

Contributions are welcome! Just create a pull request.

## Benchmarks

Calling benchmarks and forward parameters to criterion (our benchmark suite), requries to explicitly specify `--bench benches`. Otherwise `cargo bench` works out of the box.

We also can use `taskset -c core_num` to run the benchmarks on specific CPU cores. Assigning core affinity can help reduce the noise, between runs.

### Save baseline

To save a baseline called "main"

```shell
taskset -c 8 cargo bench --bench benches -- --save-baseline main
```

or to benchmark with native CPU instructions enabled

```shell
RUSTFLAGS="-C target-cpu=native" taskset -c 8 cargo bench --bench benches -- --save-baseline main-native
```

### Compare with baseline

```shell
taskset -c 8 cargo bench --bench benches -- --baseline main
```

or to compare with the native CPU instructions enabled baseline

```shell
cargo bench --bench benches -- --baseline main-native
```

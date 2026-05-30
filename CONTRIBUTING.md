# Contributing

Contributions are welcome! Just create a pull request.

## Benchmarks

Calling benchmarks and forward parameters to criterion (our benchmark suite), requries to explicitly specify `--bench benches`. Otherwise `cargo bench` works out of the box.

### Save baseline

To save a baseline called "main"

```shell
cargo bench --bench benches -- --save-baseline main
```

or to benchmark with native CPU instructions enabled

```shell
RUSTFLAGS="-C target-cpu=native" cargo bench --bench benches -- --save-baseline main-native
```

### Compare with baseline

```shell
cargo bench --bench benches -- --baseline main
```

or to compare with the native CPU instructions enabled baseline

```shell
cargo bench --bench benches -- --baseline main-native
```

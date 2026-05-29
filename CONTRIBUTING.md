Contributions are welcome! Just create a pull request.

## Benchmarks

Calling benchmarks and forward parameters to criterion (our benchmark suite), requries to explicitly specify `--bench benches`. Otherwise `cargo bench` works out of the box.

### Save baseline

To save a baseline called "main"

```shell
cargo bench --bench benches -- --save-baseline main
```

### Compare with baseline

```shell
cargo bench --bench benches -- --baseline main
```

# atoi-rs
Parse integers directly from `[u8]` slices in safe code

# Reasons to use this crate
Starting from a binary or ascii format you can parse an integer around five times as fast as with
the more idiomatic detour over utf8. The crate comes with benchmarks so you can see for yourself.

# Example

Parsing to digits from a slice
```rust
use atoi::atoi;
assert_eq!(Some(42), atoi::<u32>(b"42"));
```

This [crate](https://www.crates.io/crates/atoi) as more to offer! Check out the full documentation
at [docs.rs](http://https://docs.rs/atoi).
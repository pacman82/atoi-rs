# atoi-rs
Parse integers directly from `[u8]` slices in safe code

# Examples

Parsing to digits from a slice
```rust
use atoi::atoi;
assert_eq!((42,2), atoi::<u32>(b"42"));
```
Additional bytes after the number are ignored
```rust
assert_eq!((42,2), atoi::<u32>(b"42 is the answer to life, the universe and everything"));
```
The second number indicates how many bytes were 'used'
```rust
assert_eq!((12345,5), atoi::<u32>(b"12345 and now to something completly different...));
```
`(0,0)` is returned if the slice does not start with a digit
```rust
assert_eq!((0,0), atoi::<u32>(b"Sadly we do not know the question"));
```
While signed integer types are supported...
```rust
assert_eq!((42,2), atoi::<i32>(b"42"));
```
... signs currently are not (subject to change in future versions)
``` rust
assert_eq!((0,0), atoi::<i32>(b"-42"));
```

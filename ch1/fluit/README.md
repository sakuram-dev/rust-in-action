# Buffer Overflow Example

The original `main.rs` code contains a deliberate mistake to demonstrate a buffer overflow. The code attempts to access an out-of-bounds index in a vector, which will cause a runtime panic in Rust.

### Original Code
```rust
fn main() {
    let fluit = vec![1, 2, 3];

    let buffer_overflow = fluit[4];

    assert_eq!(buffer_overflow, 4);
}
```

### Corrected Code
The corrected code ensures that the index accessed is within bounds:

```rust
fn main() {
    let fluit = vec![1, 2, 3];

    // Accessing the correct index within bounds
    let correct_value = fluit[2];

    assert_eq!(correct_value, 3);
}
```

This change prevents the runtime panic and ensures the program runs correctly.
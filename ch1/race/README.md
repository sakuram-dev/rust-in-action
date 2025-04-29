# README

## Problematic Code

```rust
use std::thread;
fn main() {
    let mut data = 100;
    thread::spawn(|| { data = 500; });
    thread::spawn(|| { data = 1000; });
    println!("{}", data);
}
```

### Issues in the Code
1. **Ownership Violation**:
   - The variable `data` is owned by the main thread, but the spawned threads attempt to access and modify it without transferring ownership or using a shared reference.
   - Rust enforces strict ownership rules to ensure memory safety, and this code violates those rules.

2. **Data Race**:
   - Multiple threads attempt to modify `data` simultaneously, leading to undefined behavior.
   - Rust prevents data races by requiring explicit synchronization mechanisms when sharing data between threads.

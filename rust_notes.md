# Rust Notes

Small syntax reminders for interview practice. This is intentionally generic and
does not include problem-specific algorithms.

## Common Shapes

```rust
let mut items = Vec::new();
items.push(value);
```

```rust
if things.is_empty() {
    return Vec::new();
}
```

```rust
for item in items {
    // use item
}
```

```rust
for (i, item) in items.iter().enumerate() {
    // use i and item
}
```

## Borrowing And References

```rust
fn work(values: &[i32]) {
    // read values without taking ownership
}
```

```rust
let n = values.len();
let first = &values[0];
let part = &values[..n];
```

## Strings

```rust
let s = "hello".to_string();
let t = other_string.clone();
```

```rust
if a == b {
    // strings can be compared with ==
}
```

## Struct Literals

```rust
let point = Point { x: 1, y: 2 };
```

## Helpful Commands

```bash
cargo run
```

```bash
cargo check
```

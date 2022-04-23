# Unique-Token

Create unique tokens.

## Example

```rust
use unique_token::Unique;

let x = Unique::new();
let y = Unique::new();

// different tokens are unequal
assert_ne!(x, y);

// clones are equal
assert_eq!(x, x.clone());
assert_eq!(y, y.clone());
```

## Documentation

[Documentation](https://docs.rs/crates/unique-token)

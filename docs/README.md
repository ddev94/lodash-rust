# Lodash for rust


Lodash-rust is a utility library delivering modularity similar to javaScript lodash.

## Install

Depend on `lodashrust` in `Cargo.toml`:

```toml
[dependencies]
lodashrust = "1.0.0"
```

# Functions

## Array

### find_index

```rust
let arr = vec![1, 2, 3, 4];
lodashrust::find_index(&arr, |&x| x == 2); // output: 1
lodashrust::find_index(&arr, |&x| x == 5); // output: -1
```

### chunk

```rust
let arr = vec![1, 2, 3, 4, 5];
lodashrust::chunk(&arr, 2); // output: [[1, 2], [3, 4], [5]]
```
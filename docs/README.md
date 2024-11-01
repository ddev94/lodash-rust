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

It returns the index of the first element predicate returns truthy for instead of the element itself.

```rust
let arr = vec![1, 2, 3, 4];
lodashrust::find_index(&arr, |&x| x == 2); // output: 1
lodashrust::find_index(&arr, |&x| x == 5); // output: -1
```

### chunk

Creates an array of elements split into groups the length of size. If array can't be split evenly, the final chunk will be the remaining elements.

```rust
let arr = vec![1, 2, 3, 4, 5];
lodashrust::chunk(&arr, 2); // output: [[1, 2], [3, 4], [5]]
```

### compact

Creates an array with all falsey values removed. The values false, null, 0, "", undefined, and NaN are falsey.

```rust
let arr = vec![0, 2, 3, 4, 5];
lodashrust::compact(&arr); // output: [2, 3, 4, 5]
```

### concat

Creates a new array concatenating array with any additional arrays and/or values.

```rust
let arr1 = vec![1, 2, 3];
let arr2 = vec![4, 5];
lodashrust::concat(&arr1, &arr2); // output: [1, 2, 3, 4, 5]
```

### difference

Creates an array of array values not included in the other given arrays using [SameValueZero](https://262.ecma-international.org/7.0/#sec-samevaluezero) for equality comparisons. The order and references of result values are determined by the first array.

```rust
let arr1 = vec![1, 2, 3];
let arr2 = vec![2, 3];
lodashrust::difference(&arr1, &arr2); // output: [1]
```


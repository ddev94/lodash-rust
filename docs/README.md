# Lodash for rust


Lodash-rust is a utility library delivering modularity similar to [javaScript lodash](https://lodash.com/).

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

### drop

Creates a slice of array with n elements dropped from the beginning.

```rust
lodashrust::drop(&vec![1, 2, 3], 1); // output: [2, 3]
lodashrust::drop(&vec![1, 2, 3], 2); // output: [3]
```

### fill

Fills elements of array with value from start up to, but not including, end.

```rust
lodashrust::fill(&vec![1, 2, 3], 2); // output: [2, 2, 2]
lodashrust::fill(&vec![1, 2, 3], 0); // output: [0, 0, 0]
```

### head

Gets the first element of array.

```rust
lodashrust::head(&vec![1, 2, 3]); // output: 1
lodashrust::head(&vec!["Hello", "Rust"]); // output: Hello
lodashrust::head(&vec![vec![1], vec![2]]); // output: vec![1]
```

### join

Converts all elements in array into a string separated by separator.

```rust
lodashrust::join(&vec![1, 2, 3], "~"); // output: "1~2~2"
lodashrust::join(&vec![1, 2, 3], "+"); // output: "1+2+3"
lodashrust::join(&vec![1, 2, 3], ""); // output: "123"
```


### last

Gets the last element of array.

```rust
lodashrust::last(&vec![1, 2, 3]); // output: 3
```

### pull

Removes all given values from array using [SameValueZero](https://262.ecma-international.org/7.0/#sec-samevaluezero) for equality comparisons.

```rust
lodashrust::pull(&vec![1, 2, 3], &vec![1, 4, 5]); // output: [2, 3]
lodashrust::pull(&vec![1, 2, 3], &vec![2, 3]); // output: [1]
```

### remove

Removes all elements from array that predicate returns truthy for and returns an array of the removed elements.

```rust
lodashrust::remove_if(&vec![1, 2, 3], |&n| n % 2 == 0); // output: [1, 3]
lodashrust::remove_if(&vec![1, 2, 3, 4, 5, 6], |&n| n == 6); // output: [1, 2, 3, 4, 5]
```

### reverse

Reverses array so that the first element becomes the last, the second element becomes the second to last, and so on.

```rust
lodashrust::reverse(&vec![1, 2, 3]); // output: [3, 2, 1]
```

### slice

Creates a slice of array from start up to, but not including, end.

```rust
let animals = vec!["ant", "bison", "camel", "duck", "elephant"];
lodashrust::slice(&animals, Some(2), None); // output: ["camel", "duck", "elephant"]
lodashrust::slice(&animals, Some(2), Some(4)); // output: ["camel", "duck"]
```

### sort

```rust
let numbers = vec![1, 3, 4, 2, 5];
lodashrust::sort(&numbers); // output: [1, 2, 3, 4, 5]
```

### uniq

Creates a duplicate-free version of an array, using [SameValueZero](https://262.ecma-international.org/7.0/#sec-samevaluezero) for equality comparisons, in which only the first occurrence of each element is kept. The order of result values is determined by the order they occur in the array.

```rust
let numbers = vec![1, 3, 3, 4, 4, 2, 5];
lodashrust::uniq(&numbers); // output: [1, 3, 4, 2, 5]
```

### xor

Creates an array of unique values that is the [symmetric difference](https://en.wikipedia.org/wiki/Symmetric_difference) of the given arrays. The order of result values is determined by the order they occur in the arrays.

```rust
let arr1 = vec![3, 1];
let arr2 = vec![1, 2];
lodashrust::xor(&[&arr1, &arr2]); // [3, 2]
lodashrust::xor(&[&arr2, &arr1]); // [2, 3]
```

## String

### camel_case

Converts string to[ camel case](https://en.wikipedia.org/wiki/Camel_case).

```rust
lodashrust::camel_case("Geeks for Geeks"); // output: "geeksForGeeks"
lodashrust::camel_case("Geeks-for-Geeks"); // output: "geeksForGeeks"
lodashrust::camel_case("Geeks_for_Geeks"); // output: "geeksForGeeks"
```

### capitalize

Converts the first character of string to upper case and the remaining to lower case.

```rust
lodashrust::capitalize("Geeks for Geeks"); // output: "Geeks for geeks"
lodashrust::capitalize("GeeksforGeeks"); // output: "Geeksforgeeks"
```

### ends_with

Checks if string ends with the given target string.

```rust
lodashrust::ends_with("abc", "c", None); // true
lodashrust::ends_with("abc", "b", Some(2)); // true
lodashrust::ends_with("abc", "a", None); // false
```

### escape

Converts the characters "&", "<", ">", '"', and "'" in string to their corresponding HTML entities.

```rust
lodashrust::escape("fred, barney, & pebbles"); // "fred, barney, &amp; pebbles"
lodashrust::escape("fred, barney, <script>aaa</script> pebbles"); // "fred, barney, &lt;script&gt;aaa&lt;/script&gt; pebbles"
 ```

### lower_case

Converts string, as space separated words, to lower case.

```rust
lodashrust::lower_case("--Foo-Bar--"); // "foo bar"
lodashrust::lower_case("fooBar"); // "foobar"
lodashrust::lower_case("__FOO_BAR__"); // "foo bar"
```

### repeat

Repeats the given string n times.

```rust
lodashrust::repeat("*", 3); // "***"
lodashrust::repeat("abc", 2); // "abcabc"
lodashrust::repeat("abc", 0); // ""
```

### snake_case

Converts string to [snake case](https://en.wikipedia.org/wiki/Snake_case).

```rust
lodashrust::snake_case("Foo Bar"); // "foo_bar"
lodashrust::snake_case("fooBar"); // "foobar"
lodashrust::snake_case("--FOO-BAR--"); // "foo_bar"
```

### split

Splits string by separator.

```rust
lodashrust::split("Foo~Bar", "~", None); // vec!["Foo", "Bar"])
lodashrust::split("Foo~Bar~The", "~", Some(2)); // vec!["Foo", "Bar"]
```

### starts_with

Checks if string starts with the given target string.

```rust
lodashrust::starts_with("abc", "a", None); //  true
lodashrust::starts_with("abc", "b", None); //  false
lodashrust::starts_with("abc", "b", Some(1)); //  true
lodashrust::starts_with("abc", "ab", Some(0)); //  true
lodashrust::starts_with("abc", "abc", Some(0)); //  true
lodashrust::starts_with("abc", "c", Some(2)); //  true
lodashrust::starts_with("abc", "a", Some(1)); //  false
lodashrust::starts_with("abc", "", Some(1)); //  true
```

### to_lower

Converts string, as a whole, to lower case just like [String#toLowerCase](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/toLowerCase).

```rust
lodashrust::to_lower("--Foo-Bar--"); // "--foo-bar--"
lodashrust::to_lower("fooBar"); // "foobar"
lodashrust::to_lower("__FOO_BAR__"); // "__foo_bar__"
lodashrust::to_lower("HELLO WORLD!"); // "hello world!"
lodashrust::to_lower("Rust Is Awesome!"); // "rust is awesome"
```

### to_upper

Converts string, as a whole, to upper case just like [String#toUpperCase](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/toUpperCase).

```rust
lodashrust::to_upper("--foo-bar--"); // "--FOO-BAR--"
lodashrust::to_upper("fooBar"); // "FOOBAR"
lodashrust::to_upper("__foo_bar__"); // "__FOO_BAR__"
lodashrust::to_upper("hello world!"); // "HELLO WORLD!"
lodashrust::to_upper("Rust is awesome!"); // "RUST IS AWESOME!"
```

### trim

Removes leading and trailing whitespace or specified characters from string.

```rust
lodashrust::trim("  abc  "); // "abc"
lodashrust::trim("\t\nabc\r\n"); // "abc"
lodashrust::trim("abc"); // "abc"
lodashrust::trim(""); // ""
lodashrust::trim("   "); // ""
```
### truncate

Truncates string if it's longer than the given maximum string length. The last characters of the truncated string are replaced with the omission string which defaults to "...".

```rust
lodashrust::truncate("hi-diddly-ho there, neighborino", None); // "hi-diddly-ho there, neighbo..."
lodashrust::truncate(
    "hi-diddly-ho there, neighborino",
    Some(lodashrust::TruncateOptions {
        length: 24,
        separator: Some(" "),
        omission: "..."
    })
); // "hi-diddly-ho there,..."
```

### unescape


```rust
lodashrust::unescape("fred, barney, &amp; pebbles"); // "fred, barney, & pebbles"
lodashrust::unescape("&lt;div&gt;Hello &amp; welcome&lt;/div&gt;"); // "<div>Hello & welcome</div>"
```
### What this does
Finds frequency of the unique elements present in a list (Array or Vector).

It returns a hashmap, with each unique item and its frequency as `key:value` pair.

### Features:
1. Parallel frequency counting. List items are equally distributed in each logical threads.
2. Fast hashing algorithm ([XxHash](https://https://github.com/shepmaster/twox-hash)).
3. Expressive and clean code.

### Efficiency
Time Complexity: `O(N)`
Space Complexity: `O(N)`

**Version Note:**  Performance improvement and change in api.

### User Guide

This crate exports a function **`frequency_finder`**. It takes a slice as parameter, that means you can pass a slice to an Array or Vector. It will return a hashmap that will contain each unique item and its frequency as key value pair.

The items can be anything that implements `Copy`! Such as, `i32` or `&str` or others.
## Quick Start
```rust
use elements_frequency::interface::frequency_finder;

fn main () {
    let myList = ["hi", "who", "me", "me", "hi"];

    let frequency_map = frequency_finder(&myList);

    println!("{:?}", frequency_map);

    // Output:

    // { "hi": 2, "me": 2, "who": 1 }
}
```
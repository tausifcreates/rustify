### What it does
Finds a pair in a list that has the closest sum to a given number. The list can be an Array or Vector.
The code is simple and free of recursion.

It expects a **sorted list**. Time complexity `O(N)`. Space complexity `O(1)`.

### Features
1. Multiple types are supported, including floating points. But `usize`, `u64`, `u128`, `i64`, `i128` and `f64` are not supported due to [f64 cannot exactly capture these  values](https://www.reddit.com/r/rust/comments/js1avn/the_trait_stdconvertfromi64_is_not_implemented/gbxbtff/?utm_source=reddit&utm_medium=web2x&context=3), which is relevant in this context because various
types are attemped to be converted into `f64` as a general type in this crate.

	All other types are supported, eg. `i8`, `u8`, `i16`, `u16`, `i32`, `u32` and `f32`.

	So, if your variable type is **`u64`** or **`i64`**, you should cast them to **`u32`** and **`i32`** respectively before passing it to the function.

2. Expressive and clean code.

**version note:** remove unnecessary bloat and improved performance.

### How to use
This crate exports a function called **`find_pair`** that expects a reference to a **sorted list** (can be vector or array),and your desired sum (The sum you are looking for).

If there is no exact match, it will return the closest sum possible.

Please make sure to sort your list before passing it as an argument to the function.

### Quick Start

```rust
use closest_sum_pair::interface::find_pair;

fn main() {
    let mut list = [-2, -4, -7, -2, -5, -13, -7];

    list.sort();

    let desired_sum = -1;

    let pair = find_pair(&list, desired_sum);

    println!("pair {:?}", pair) // (-2, -2)
}
```

**Example with floating point:**

```rust
use closest_sum_pair::interface::find_pair;

fn main() {
    let mut list: [f32; 7] =
    [-2.2, -4.0, -7.9, -2.1, -5.5, -13.0, -7.1];

    list.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let desired_sum = -16.7;

    let pair = find_pair(&list, desired_sum);

    println!("pair {:?}", pair) // (-13.0, -4.0)
}
```
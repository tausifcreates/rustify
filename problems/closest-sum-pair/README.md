### closest_sum_pair

Fast. Clever. Free of Recursion. No **`while`** bound loops, so least runtime strikes.

It expects a **sorted list**. Time complexity `O(N)`. Space complexity `O(1)`.

### Update !

From now multiple types are supported, including floating point numbers. But `usize`, `u64`, `u128`, `i64`, `i128` and `f64` are not supported due to [f64 cannot exactly capture these  values](https://www.reddit.com/r/rust/comments/js1avn/the_trait_stdconvertfromi64_is_not_implemented/gbxbtff/?utm_source=reddit&utm_medium=web2x&context=3), which is relevant in this context because various
types are attemped to be converted into `f64` as a general type in this crate. 

But all other types are supported, eg. `i8`, `u8`, `i16`, `u16`, `i32`, `u32` and `f32`.

### Quick Start

Due to **`find_pair`** function expects a **sorted list** (can be vector or array),
you should sort your list using any preferred algorithm, before passing it as parameter.

The **`find_pair`** function also expects your desired sum (The sum you are looking for)
If there is no exact match, it will return the closest sum possible.

```rust
use closest_sum_pair::interface::find_pair;

fn main() {
    let mut list = [-2, -4, -7, -2, -5, -13, -7];

    list.sort();

    let desired_sum = -1;

    let pair = find_pair(&mut list, desired_sum);

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

    let pair = find_pair(&mut list, desired_sum);

    println!("pair {:?}", pair) // (-13.0, -4.0)
}
```

**version note:** Support multiple types.
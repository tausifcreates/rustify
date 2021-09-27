### closest_sum_pair

Fast. Clever. Free of Recursion. No **`while`** bound loops, so least runtime strikes.

As it expects unsorted list, The time complexity is `O(NlogN)`. Space complexity `O(1)`.

### Quick Start

```rust
use closest_sum_pair::interface::find_pair;

fn main() {
    let mut list = [-2, -4, -7, -2, -5, -13, -7];

    let desired_sum = -1;

    let pair = find_pair(&mut list, desired_sum);

    println!("pair {:?}", pair) // (-2, -2)
}
```

**version note:** Made the api easier to use.
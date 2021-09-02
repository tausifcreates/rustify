### closest_sum_pair

Fast. Clever. Free of Recursion. No **`while this { do that }`** fuckery, so least runtime strikes.

As it expects unsorted list, The time complexity is `O(NlogN)`. Space complexity `O(1)`.

### Quick Start

```rust
use closest_sum_pair::interface::Elements;

fn main() {
    let mut list = [-2, -4, -7, -2, -5, -13, -7];

    let desired_sum = -1;

    let mut elements = Elements::new(&mut list, desired_sum);

    let pair = elements.init_distance().find_pair();

    println!("closest pair: {}", pair); // (-2, -2)
}

```


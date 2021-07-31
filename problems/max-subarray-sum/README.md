# max_subarray_sum
Finds maximum subarray sum in a list. If there are multiple such
subarrays, only the one that comes first is selected.

The algorithm has time complexity of `O(N)` and space complexity
of `O(1)`.

# Quick Start
```rust
use max_subarray_sum::interface::Elements;

fn main() {
    let list = vec![-2, -3, 4, -1, -2, 1, 5, -3];

    //Or you can use an array instead:
    let list = [-2, -3, 4, -1, -2, 1, 5, -3];

    let elements = Elements::new(&mut list);

    let max_sum = elements.find_max_sum().result();

    assert_eq!(7, max_sum);
}

```
Far across the distance,
And spaces, between us 🎵

### Functionality
Finds the maximmum **`right - left`**, such that **`list[right] > list[left]`**.

**Time Complexity :** **`O(n)`** (2 traversals)

**Space Complexity :** **`O(n)`** (1 extra list)

### How to use
The **`find_distance`** function takes a ref to an array or a vector as a paramaeter, and finds the maximum distance of two such elements.

It returns an `Option<usize>` type as a result, because two such numbers that satisfy the condition might not exist. In that case, it returns `None`.

**Quick Start:**
```rust
use between_us::interface::find_distance;

fn main() {
	let list = [5, 3, 7, 1, 6, 8, 4];

	let result = find_distance(&list);

	println!("{:?}", result);

	// Output: Some(5)
}
```

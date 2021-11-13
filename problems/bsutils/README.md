### What this Does
If your **sorted** array/vec has duplicate items, a binary search may pick the desired item from an arbitrary position of that array. This crate provide you some utility functions, that will let you know what was the first or last index that item was found in.

For one, see this sorted arry example:
```rs
let arr = [1, 3, 4, 6, 6, 6, 6, 8, 12]
```
If we look up for the item **`6`**, a simple binary search will return an index ranging 3~6. It can be 4 or 5. If you precisely need to know the first/last (or both) index **`6`** was found in, this crate will be helpful for you.

### Features
1. Support multiple types.
2. Clean and expressive code.
3. Time Complexity: **`O(logn)`**, Space Complexity: **`O(1)`**

### How to use
This crate exports 3 functions, namely:

**`find_first_idx` :** Finds the index where the item was first found.

**`find_last_idx` :** Finds the index where the item was last found.

**`first_last_idx` :** Finds both indexes where the item was first and last found. Returns a tuple.

All of the 3 functions expects a ref to a **sorted** Array/Vector, and the item that you are looking for.

### Quick Start
**Find first index :**
```rs
use bsutils::interface::find_first_idx;

fn main() {
	let list = [1, 1, 4, 6, 7, 7, 7, 7, 9, 9, 11];
    let given = 7; // the item we look for
	let first_idx = find_first_idx(&list, given);
	println!("{}", first_idx);

	// Output: 4
}
```
**Find last index :**
```rs
use bsutils::interface::find_last_idx;

fn main() {
	let list = [1, 1, 4, 6, 7, 7, 7, 7, 9, 9, 11];
    let given = 7; // the item we look for
	let last_idx = find_last_idx(&list, given);
	println!("{}", last_idx);

	// Output: 7
}
```
**Find first & last both indexes :**
```rs
use bsutils::interface::first_last_idxs;

fn main() {
	let list = [1, 1, 4, 6, 7, 7, 7, 7, 9, 9, 11];
    let given = 7; // the item we look for
	let both_idxs = first_last_idxs(&list, given);
	println!("{}", both_idxs);

	// Output: (4, 7)
}
```
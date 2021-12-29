## What it does
Finds the longest increasing subsequence (lis).

## Features
1. Time complexity `O(nlogn)`
2. Space complexity `O(n)`
3. Supports multiple types

## Example

```rs
use lisa::interface::find_lis;

fn main() {
	let slice = [3, 10, 2, 1, 20];
	let lis = find_lis(&slice);

	println!("{:?}", lis); // [3, 10, 20]
 }
```

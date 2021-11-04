### Find all occurances of an element in row-wise and column-wise sorted matrix.

Time complexity: **`O(m+n)`** where, m=number of columns, n=number of rows.
Space complexity: **`O(m+n)`**

#### What is special about this crate
The time complexity is not even linear, it's almost the square root of liner time complexity. The linear time complexity is **`O(m*n)`**.

#### How do you use it?
You don't pass a matrix! Rather you pass an array(or a vec) that emulates a 2D matrix. How? Count each index of that collection as (X + nY).

For one, this is a simple matrix, that is sorted in both directions:

```rs
let matrix =

	| 1  2  4  5 |
	| 3  4  4  5 |
	| 5  5  8  9 |
```
This matrix has 3 rows and 4 columns. Now, consider the element 8, that is at 3rd row and 3rd column.

Then we translate this 2D matrix into a single array like this:
```rs
let matrix = [1, 2, 4, 5,  3, 4, 4, 5,  5, 5, 8, 9]
```
How are we going to translate 8's position in this array?

Simple! `columns x (8's row_position - 1) + 8's (column position - 1)`
That is 10.

This crate has a function, `equal_is_bigger`. The name is funny, but this is what it actually does. So how would you use it? Lets look at an example.

We call our matrix a sandwich (A sorted matrix reminds me of stacked sandwiches)

You define how many rows and columns are in your matrix.

You pass the element you want to search.

The function returns a pattern. Lets see what it is.

```rust
use stacked_sandwich::interface::equal_is_bigger;

fn main() {
	let sandwich = [1, 2, 4, 5,  3, 4, 4, 5,  5, 5, 8, 9]; // Your translated matrix
	let rows = 3;
	let cols = 4;
	let search_element = 5;

	let result = equal_is_bigger(&sandwich, cols, rows, search_element);
}
```
So, How does the result look like? Somewhat like this:
```rs
[
 OccurancePattern { row: 0, leftbound_col: 1, rightbound_col: 4 },
 OccurancePattern { row: 1, leftbound_col: 1, rightbound_col: 3 },
 OccurancePattern { row: 2, leftbound_col: 0, rightbound_col: 0 }
]
```
Rows and Columns are both indexed from 0. That means, 1st row is the 0th row, 1st column is 0th column. Now take your original matrix and match it with the result!

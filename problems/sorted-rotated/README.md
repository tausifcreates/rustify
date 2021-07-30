# sorted_rotated

Suppose you have a sorted, then rotated list of numbers,
It will find the number you searching in `O(logN)` time.

The crate will not fail if the list is not rotated, or it
has duplicate items. In this case it will travel the list
twice, causing some performance drop. But it will surely 
fail if the list is not sorted (in ascending order).

The method is simple: perform a binary search to find the pivot,
then minimize the range for binary search to find your desired
number. If there's no pivot it will simply do binary search on 
the list again.

In both cases, it returns an `Option` type either with the index
of the found number or `None`. So you have to explicitly set up
`match` arms to extract the value.

# Quick Start
```rust
fn main() {
    let list: Vec<i32> = vec![5, 6, 1, 2, 3, 4];

    // Or an array can be used as well:
    let list: [i32; 6] = [5, 6, 1, 2, 3, 4];

    let desired: i32 = 2;
    
    let mut element = Elements::new(list, desired);
    
    match element.find_pivot().find_desired().result() {
        Some(idx) => println!("index: {}", idx),
        None => println!("Not found"),
    }
}
```
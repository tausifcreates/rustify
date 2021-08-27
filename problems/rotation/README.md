## rotation
The idea for more efficient algorithm was derived from [this](https://stackoverflow.com/a/32698823) stack overflow answer.

To quote from the answerer, *'Reversing three times is simplest but moves every element exactly twice, takes `O(N)` time and `O(1)` space. It is possible to circle shift an array moving each element exactly once also in `O(N)` time and `O(1)` space.'*

For more info on how it adds up together, please refer to the answer (and other answers as well).

This crate exposes 2 apis, namely `rotate_clock` function that you will use to rotate your list clockwise, and `rotate_anticlock` for vice versa. below 2 examples are given to demonstrate how to use them.


### Quick Start

```rust
use rotation::interface as rotator;

fn main() {
    let mut list = [1, 2, 3, 4, 5];

    let spin = 2;

    // rotate clockwise
    let rotated = rotator::rotate_clock(&mut list, spin);

    println!("{:?}", rotated); // [4, 5, 1, 2, 3];
 
    // rotate anti clockwise
    let rotated = rotator::rotate_anticlock(&mut list, spin);

    println!("{:?}", rotated); // [3, 4, 5, 1, 2];
}
```
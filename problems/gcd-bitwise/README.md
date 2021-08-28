### gcd_bitwise

**Disclaimer: The code is not mine.**

The code is part of the [coreutils](https://github.com/uutils/coreutils/blob/15da98d84e9a094ea72c5f51efcc2d8aa9e9184f/src/uu/factor/src/numeric/gcd.rs) project. I have forked it for ease of use, for those who dont want to pull in big dependencies for calculating gcd.

## Big Update!
You can pass any numeric type into `gcd()` function. If you pass 2 `u16` type, the result you will get will be
the same `u16`. You can pass `u8`, `u16`, `u32`, `u64` and hell, even `usize`. Please have a look at the **Quick Start** part below for examples.

### Some Notes
This code uses stein's algorithm, that replaces division with arithmetic shifts, comparisons, and subtraction, for optimization of performance. For more info on how efficient this algorithm is, please refer to [this page](https://en.wikipedia.org/wiki/Binary_GCD_algorithm).

### Quick Start
```rust
use gcd_bitwise::interface::gcd;

fn main() {
    // For `u8` type
    let num1: u8 = 15;

    let num2: u8 = 51;
     
    let gcd: u8 = gcd(num1, num2);
     
    println!("gcd: {}", gcd); // 3   

    // For `u16` type
    let num1: u16 = 65535;

    let num2: u16 = 125;
     
    let gcd: u16 = gcd(num1, num2);
     
    println!("gcd: {}", gcd); // 5 

    // For `u32` type
    let num1: u32 = 65535;

    let num2: u32 = 125;
     
    let gcd: u32 = gcd(num1, num2);
     
    println!("gcd: {}", gcd); // 5 

    // And on it goes...
}
```

***version_note:*** Made the api simpler to use. Please refer to the [repository](https://github.com/Inoshy/rust-book-helper/tree/master/problems/gcd-bitwis) for the documentation, if you find that `docs.rs` didn't build the docs for latest version.
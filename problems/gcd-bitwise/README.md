### gcd_bitwise

**Disclaimer: The code is not mine.**

The underlying code is part of the [coreutils](https://github.com/uutils/coreutils/blob/15da98d84e9a094ea72c5f51efcc2d8aa9e9184f/src/uu/factor/src/numeric/gcd.rs) project. I have forked it for ease of use, for those who dont want to pull in big dependencies. I modified some parts for general use cases, eg. implementing generic types. This crate is dependency free.

### Update!
numeric types of arguments will be cast to **`usize`** instead of **`u32`** from now.

You can pass **`u8`**, **`u16`**, **`u32`**, **`u64`** and **`usize`** numeric types into **`gcd()`** function. But please note that the 2 numbers that you pass must have the same type. Passing any signed type (like `i32`) may give unexpected results. Please have a look at the **Quick Start** section below for examples.

### Some Notes
This code uses stein's algorithm, that replaces division with arithmetic shifts, comparisons, and subtraction, for optimization of performance. For more info please refer to [this page](https://en.wikipedia.org/wiki/Binary_GCD_algorithm).

### Quick Start
```rust
use gcd_bitwise::interface::gcd;

fn main() {
    // For `u8` type
    let num1: u8 = 15;

    let num2: u8 = 51;
     
    let gcd: u8 = gcd(num1, num2);
     
    println!("gcd: {}", gcd); // 3   

    // For `usize` type
    let num1: usize = 65535;

    let num2: usize = 125;
     
    let gcd: usize = gcd(num1, num2);
     
    println!("gcd: {}", gcd); // 5 

    // And on it goes...
}
```

**`Final Note:`** If you find any problems dont hesitate to open an issue on github.
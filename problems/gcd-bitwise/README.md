### gcd_bitwise

**Disclaimer: The code is not mine.**

The code is part of the [coreutils](https://github.com/uutils/coreutils/blob/15da98d84e9a094ea72c5f51efcc2d8aa9e9184f/src/uu/factor/src/numeric/gcd.rs) project. I have forked it for ease of use, for those who dont want to pull in big dependencies for calculating gcd.

**Some notes:** This code uses stein's algorithm, that replaces division with arithmetic shifts, comparisons, and subtraction, for optimization of performance. For more info on how efficient this algorithm is, please refer to [this page](https://en.wikipedia.org/wiki/Binary_GCD_algorithm#Efficiency).
 
### Quick Start
```rust
use gcd_bitwise::interface::GcdBuilder;

fn main() {
    let num1 = 15;

    let num2 = 51;
     
    let gcd = GcdBuilder::new(num1, num2);
     
    let gcd = gcd.build();
     
    println!("gcd: {}", gcd); // 3   
}
```

The specific rust implementation from [coreutils](https://github.com/uutils/coreutils/blob/15da98d84e9a094ea72c5f51efcc2d8aa9e9184f/src/uu/factor/src/numeric/gcd.rs) project showcases several performance optimisations:

* Trial division by `2` is eschewed in favour of a single bitshift and the count trailing zeros primitive `u64::trailing_zeros`. This performs the equivalent of applying repeatedly identity `gcd(2u, v) = gcd(u, v)`, in a much smaller amount of time.
  
* The loop is laid out so as to avoid repeated work; for instance, eliminating `2` as a factor of `num2` was moved to the back of the loop, and after the exit condition, as `num2` is known to be odd upon entering the loop.
  
* The body of the loop is branch-free except for its exit condition `(num2 == 0)`, as the exchange of `num1` and `num2` (ensuring `num1 <= num2`) compiles down to conditional moves. Hard-to-predict branches can have a large, negative impact on performance.
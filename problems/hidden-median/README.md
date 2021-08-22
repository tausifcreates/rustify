## hidden_median

Finds the hidden median of 2 sorted arrays in `Olog(min(a, b))` time.

This crate works with both arrays and vectors. Also, I have avoided recursion 
and while-loop for minimizing runtime strikes, instead used a plain for loop 
that runs only `Olog(min(a, b))` times.

In short, this is `hidden_median` crate for you. lets look at a quick example!

### Quick Start

```rust
use hidden_median::interface::Holder;
     
fn main() {
    let list_a = [-3, -1, 5, 6];
    let list_b = [-7, -2, 4, 8, 11];

    let mut holder = Holder::new(&list_a, &list_b);

    let result = holder.init().result();
    
    // `result` is a tuple, that comes with a help text!
    
    println!("{:?}", result);
    
    // output: (4, 4, "median is a single number.")
}
```

**Version Note:** changed the `readme`
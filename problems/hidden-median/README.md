## hidden_median

you have 2 sorted lists of different size (or similar), and you want to find the median
when these 2 lists are merged without breaking sorted state. Also, you might want to
find median in the fastest time, that is `Olog(min(a, b))`, whereas `a` and `b` are the
length of your arrays or vectors, whatever.

Yes, this crate uses an algorithm that performs the search in shortest time. Also, I
have avoided recursion and while-loop for minimizing runtime strikes, instead used a 
plain for loop that runs only `Olog(min(a, b))` times.

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
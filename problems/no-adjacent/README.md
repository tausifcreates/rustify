### no_adjacent

Finds Maximum sum when no two elements are adjacent in a list.

Time complexity `O(N)`, Space complexity `O(1)`.

### Quick Start

```rust
use no_adjacent::interface::calculate;

fn main() {
    let list = [2, 1, 4, 6, 8, 9, 18];

    let sum = calculate(&list);

    println!("sum {}", sum); // 32
}

```
**version note:** Fix a stupid design
Finds frequency of the unique elements present in the list.

### Update: Using Mutex<T> instead of Arc<Mutex<T>>.

Arc  has some runtime performance cost. Usage of Arc is dropped from now on.

~~This lbrary can work with any types that implement `Clone`.~~
~~So it is expected to work with Strings, slices, integers etc.~~

Not anymore. From now, it works with all types that implement `Copy`. Also it became kinda bloated, this small crate has got a dependency, `crossbeam`.

Good news is, now its multithreaded, and you can decided how many threads it will use. the **`find_frequency`** function takes **`2`** arguments, the first one is the array or vector containing elements, and second argument is the number of threads.

you can specify any number of threads you want, and performance may vary upon that. The function will just return a hashmap, that is, unique elements hashed with their frequencies.

The function doesn't do anything else, for example, arrange them in a special order. it leaves the rest upto the user, they can do what they want with it. This improves perfomance.

* Time Complexity: `O(N)`
* Space Complexity: `O(N)`

# Quick Start
```rust
use elements_frequency::interface::frequency_finder;
 
fn main () {
    let myList = [1, 1, -6, 2, 6, 2, 7, 1];
 
    let myThreads = 6;
 
    let frequency_map = frequency_finder(&myList, myThreads);
 
    println!("{:?}", frequency_map);

    // Output:
    // {6: 1, -6: 1, 2: 2, 7: 1, 1: 3}
}
```
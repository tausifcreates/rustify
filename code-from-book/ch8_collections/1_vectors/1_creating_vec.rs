fn main() {
    // Vectors allow you to store more than one value in a
    // single data structure that puts all the values next
    // to each other in memory. Vectors can only store values
    // of the same type.

    // To create a new, empty vector, we can call the
    // `Vec::new` function
    let v: Vec<i32> = Vec::new();

    // Note that we added a type annotation here. Because we 
    // aren’t inserting any values into this vector, Rust doesn’t
    // know what kind of elements we intend to store.

    // `Vec<T>` type provided by the standard library can hold any 
    // type, and when a specific vector holds a specific type, the 
    // type is specified within angle brackets

    // Because we’ve given initial `i32` values, Rust can infer that
    // the type of `v` is `Vec<i32>`, and the type annotation isn’t 
    // necessary.
    let v = vec![1, 2, 3];
}

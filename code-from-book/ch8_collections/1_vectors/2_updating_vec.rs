fn main() {
    // As with any variable, if we want to be able to change its
    // value, we need to make it mutable using the `mut` keyword.
    let mut v = Vec::new();

    v.push(3);
    v.push(6);

    // The numbers we place inside are all of type `i32`, and Rust
    // infers this from the data, so we don’t need the `Vec<i32>` 
    // annotation.
}

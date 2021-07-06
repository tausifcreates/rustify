#[allow(unused_variables)]
fn main() {
    // you can use [] with a range to create a string slice
    // containing particular bytes:
    let s = String::from("Здравствуйте");
    let some_slice = &s[0..2]; // `3`

    // Here, `s` will be a &str that contains the first 2 bytes
    // of the string. Earlier, we mentioned that each of these
    // characters was 2 bytes, which means `s` will be `З`.

    // if we used &hello[0..1], rust would panic at runtime in 
    // the same way as if an invalid index were accessed in a vector.
    // --------------------------------
    // let some_slice = &s[0..1];
    // --------------------------------
    // panic: thread 'main' panicked at 'byte index 1 is not a char
    // boundary; it is inside 'З' (bytes 0..2) of `Здравствуйте`'
}
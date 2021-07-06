#[allow(unused_variables)]
fn main() {
    // If you need to perform operations on individual Unicode
    // scalar values, the best way to do so is to use the `chars`
    // method.

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    // न म स  ् त  े

    // The `bytes` method returns each raw byte.
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
    // This code will print the 18 bytes that make up this String:
    //
    // 224
    // 164
    // --snip--
    // 165
    // 135
}
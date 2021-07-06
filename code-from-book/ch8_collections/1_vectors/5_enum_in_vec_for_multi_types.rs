fn main() {
    // Vectors can only store values that are the same type.
    // Fortunately, the variants of an enum are defined under
    // the same enum type, so when we need to store elements
    // of a different type in a vector, we can define and use an enum.

    // If Rust allowed a vector to hold any type, there would be a
    // chance that one or more of the types would cause errors with
    // the operations performed on the elements of the vector. Using
    // an enum plus a match expression means that Rust will ensure at
    // compile time that every possible case is handled.

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    use SpreadsheetCell::{Float, Int, Text};

    let v = vec![Int(3), Float(10.5), Text(String::from("blue"))];

    println!("{:?}", v); // [Int(3), Float(10.5), Text("blue")]
}

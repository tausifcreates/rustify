fn main() {
    // This line will create an empty `String` that we can load data into.
    let s = String::new();

    let data = "initial contents";

    let s = data.to_string();

    // The method also works on a literal directly
    let s = "initial contents".to_string();
    
    // We can also use the function `Sring::from` to
    // create `String` from a string literal
    let s = String::from("initial contents");
}

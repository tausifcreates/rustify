fn main() {
    let v = vec![
        String::from("ino"),
        String::from("is it"),
        String::from("you"),
    ];

    // This commented out code will give us error:
    // cannot move out of index of `Vec<String>`,
    // because `String` doesn't implement `Copy` trait.
    // ------------------------
    // let a = v[1]; https://stackoverflow.com/questions/27904864/what-does-cannot-move-out-of-index-of-mean
    // ------------------------

    // To fix this, we can take reference of the indexed objects.

    // Get the third element by using `&` and `[]`,
    // which gives us a reference.
    let third = &v[2];
    println!("The third element is: {}", third);

    // Also we can use `get` method with the index passed
    // as an argument, which gives us an `Option<&T>`
    match v.get(2) {
        Some(val) => println!("Third element is {}", val),
        None => println!("There is no third element"),
    }

    // let’s see what happens if we try to access an element at
    // index 100, which doesn't exist.

    // This method will cause the program to panic because it 
    // references a nonexistent element.
    let hundredth = &v[100];


    // When the get method is passed an index that is outside 
    // the vector, it returns None without panicking. Your code 
    // needs to have logic to handle having either `Some(&element)`
    // or `None`.
    let hundredth = v.get(100);
}

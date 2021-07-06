fn first_word(some_string: &str) -> &str {
    let bytes = some_string.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &some_string[0..i];
        }
    }

    some_string
}

fn main() {
    let s = String::from("hello world");

    let word = first_word(&s[..]);

    // The type of `my_string_literal` here is `&str`: it’s a slice pointing
    // to that specific point of the binary. This is also why string literals
    // are immutable.
    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let word = first_word(&my_string_literal[..]);

    // Because string literals are string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}

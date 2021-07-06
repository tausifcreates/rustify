fn first_word(some_string: &String) -> &str {
    let bytes = some_string.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &some_string[0..i];
        }
    }

    &some_string[..]
}

fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear();

    println!("{}", word); // Error! `word` is connected with the state of `s`.
                          // So mutating `s` will produce error.
}

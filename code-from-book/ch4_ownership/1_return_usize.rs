fn first_word(some_string: &String) -> usize {
    let bytes = some_string.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    some_string.len()
}

fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear();

    // `word` still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!

    println!("{}", word); // `s` is not connected to the state of `word`.
                          // `word` is an independent `usize`. So using
                          // it after flushing the string will not produce
                          // error.
}

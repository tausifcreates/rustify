use std::fs::File;
use std::io;
use std::io::Read;
fn main() {
    let r = read_user_name_from_file();

    println!("{:?}", r);
}

fn read_user_name_from_file() -> Result<String, io::Error> {
    // The `?` placed after a Result value is defined to work in almost
    // the same way as the match expressions defined to handle the `Result`
    // values.
    //
    // If the value of the `Result` is an `Ok`, The `?` operator
    // returns the value inside `Ok` from the expression.
    //
    // If the value is an `Err`, the `Err` will be returned from
    // the whole function as if we had used the `return` keyword
    // so the error value gets propagated to the calling code.
    let mut f: File = File::open("hello.txt")?;

    let mut s = String::new();

    f.read_to_string(&mut s)?;

    Ok(s)
}

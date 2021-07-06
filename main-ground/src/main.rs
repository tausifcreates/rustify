use std::fs::File;
use std::io;
use std::io::Read;
fn main() {
    let r = read_user_name_from_file();

    println!("{:?}", r);
}

fn read_user_name_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    // We’ve chained the call to `read_to_string` directly onto the result
    // of `File::open("hello.txt")?`
    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

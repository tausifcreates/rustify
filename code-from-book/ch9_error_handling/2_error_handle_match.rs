use std::fs::File;
use std::io;
use std::io::Read;
fn main() {
    let val = read_username_from_file();

    println!("{:?}", val);
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        // Returns `File` type to `f`.
        Ok(file) => file,

        // Returns from the `read_username_from_file` function that expects
        // `Result<String, io::Error>`.
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

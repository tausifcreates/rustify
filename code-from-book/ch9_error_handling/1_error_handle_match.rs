use std::fs::File;
use std::io::ErrorKind;
fn main() {
    let r = File::open("hello.txt");

    let f: File = match r {
        Ok(file) => file,

        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("./hello.txt") {
                Ok(fc) => fc,

                Err(err) => {
                    panic!("Problem creating the file {:?}", err);
                }
            },

            other_error => {
                panic!("{:?}", other_error);
            }
        },
    };

    println!("{:?}", f);
}

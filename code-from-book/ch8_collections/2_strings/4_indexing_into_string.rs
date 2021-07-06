fn main() {
    // A String is a wrapper over a `Vec<u8>`.

    // length and bytes
    let s = String::from("hola");
    println!("{}", s.len()); // 4
    println!("{:?}", s.as_bytes()); // [104, 111, 108, 97]

    // Here, `len` will is 4, which means the vector storing the
    // string “Hola” is 4 bytes long. Each of these letters takes
    // 1 byte when encoded in UTF-8.

    // An unusual case
    let s = String::from("Здравствуйте");
    println!("{}", s.len()); // 24
    println!("{:?}", s.as_bytes());
    // [208, 151, 208, 180, 209, 128, 208, 176, 208, 178, 209, 129,
    // 209, 130, 208, 178, 209, 131, 208, 185, 209, 130, 208, 181]

    // The reason it's 24 bytes long because each Unicode scalar
    // value in that string takes 2 bytes of storage.

    // Lets check the commented out code below.
    // ------------------------------
    // let first_index = &s[0];
    // ------------------------------

    // What could be the answer? When encoded in UTF-8, the first byte of
    // `З` is 208 and the second is 151, so answer should in fact be 208,
    // but 208 is not a valid character on its own.

    // To avoid returning an unexpected value and causing bugs that might
    // not be discovered immediately, Rust doesn’t compile this code. So
    // trying to access parts of a String using indexing syntax in Rust
    // will produce error.
}

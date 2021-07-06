fn main() {
    let mut s = String::from("ino");

    // We can grow a String by using the `push_str`
    // method to append a string slice
    s.push_str("suke"); // `inosuke`

    // `push_str` takes a single slice so it doesn't
    // take ownership of the parameter.
    let mut s = String::from("ino");

    let some_slice = "suke";

    s.push_str(some_slice); // `inosuke`

    // As `push_str` doesn't take ownership, we can
    // reuse `some_slice` later.
    println!("`some_slice` is {}", some_slice);

    // The push method takes a single character as a
    // parameter and adds it to the `String`.
    let mut s = String::from("lo");

    s.push('l') // `lol`
}

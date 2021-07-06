fn main() {
    let s1 = String::from("ino");
    let s2 = String::from("suke");
    let combo = s1 + &s2; // `inosuke`

    // Trying to reuse `s1` will produce error:
    // --------------------------------
    // println!("{}", s1);
    // --------------------------------
    // Error: borrow of moved value: `s1`

    // The reason `s1` is no longer valid after the addition
    // and the reason we used a reference to `s2` has to do with
    // the signature of the method that gets called when we use
    // the `+` operator.

    // The + operator uses the add method, whose signature looks
    // something like this:
    // -----------------------------------
    // fn add(self, s: &str) -> String {
    // -----------------------------------

    // So `let s3 = s1 + &s2` statement actually takes ownership
    // of `s1`, appends a copy of the contents of `s2`, and then
    // returns ownership of the result.

    // Concat multiple strings
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3; // `tic-tac-toe`

    // For more complicated string combining like this, we can use
    // the `format!` macro:
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3); // `tic-tac-toe`

    // using `format!` is much easier to read and doesn’t take
    // ownership of any of its parameters.
}

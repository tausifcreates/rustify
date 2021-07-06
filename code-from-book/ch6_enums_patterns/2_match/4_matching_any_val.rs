fn main() {
    let some_u8 = 0u8;

    // a `u8` can have valid values of `0` through `255`. If we only
    // care about the values `1`, `3`, and `5`, we don’t want to have
    // to list out `0`, `2`, `4`, `6`, all the way up to `255`. We can
    // use the special pattern `_` instead:
    match some_u8 {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),

        // The `_` pattern will match any value. By putting it after our
        // other arms, the `_` will match all the possible cases that aren’t
        // specified before it. The `()` is just the unit value, so nothing
        // will happen in the `_` case. As a result, we can say that we want
        // to do nothing for all the possible values that we don’t list before
        // the `_` placeholder.
        _ => (),
    }
}

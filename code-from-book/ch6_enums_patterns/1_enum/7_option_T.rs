// Rust does not have nulls, but it does have an enum that
// can encode the concept of a value being present or absent.
// This enum is `Option<T>`, and it is defined by the standard
// library as follows:
//
// ----------------------------------------------------------------
// | enum Option<T> {
// |   // <T> is a generic type parameter, it  means the `Some`
// |   // variant of the `Option` enum can hold one piece of data
// |   // of any type.
// |   Some(T),
// |   None,
// | }
// ---------------------------------------------------------------

// The `Option<T>` enum is so useful that it’s even included in
// the prelude; you don’t need to bring it into scope explicitly.
// In addition, so are its variants: you can use `Some` and `None`
// directly without the `Option::` prefix.

fn main() {
    let some_number = Some(5);

    let some_string = Some(String::from("ino"));

    // If we use None rather than Some, we need to tell Rust what
    // type of `Option<T>` we have, because the compiler can’t
    // infer the type that the Some variant will hold by looking
    // only at a `None` value.
    let absent_number: Option<i32> = None;

    // `Option<T>` and `T` (where `T` can be any type) are
    // different types, the compiler won’t let us use an
    // `Option<T>` value as if it were definitely a valid
    // value. For example,

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // Error! Rust doesn’t understand how to add an `i8` and
    // an `Option<i8>`, because they’re different types.
    let sum = x + y;

    // In general, in order to use an `Option<T>` value,
    // you want to have code that will handle each variant.
    // The match expression is a control flow construct that
    // will run different code depending on which variant of
    // the enum it has, and that code can use the data inside
    // the matching value.
}

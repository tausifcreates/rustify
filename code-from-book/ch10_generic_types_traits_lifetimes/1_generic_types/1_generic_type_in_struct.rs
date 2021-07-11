#[allow(dead_code)]
struct Point<T> {
    x: T,
    y: T,
}

#[allow(unused_variables)]
fn main() {
    let integer: Point<i32> = Point { x: 5, y: 10 };

    let float: Point<f32> = Point { x: 3.5, y: 7.5 };

    // `Point<T>` struct is generic over some type `T`, and the fields
    // `x` and `y` are both that same type, whatever that type may be.
    // If we create an instance of a `Point<T>` that has values of
    // different types, the code won’t compile.
    //
    // -------------------------------------
    // let mixed = Point {x: 5, y: 6.5};
    // -------------------------------------
    // Error: Mismatched types
}

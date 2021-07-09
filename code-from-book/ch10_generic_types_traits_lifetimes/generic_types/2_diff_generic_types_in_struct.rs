#[allow(dead_code)]
struct Point<T, U> {
    x: T,
    y: U,
}

#[allow(unused_variables)]
fn main() {
    let both_integer = Point { x: 5, y: 10 };

    let both_float = Point { x: 3.5, y: 7.5 };

    let int_and_float = Point { x: 5, y: 7.5 };
}

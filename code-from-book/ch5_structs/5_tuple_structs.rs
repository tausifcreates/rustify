#[derive(Debug)]
struct Point (i32, i32, i32);

struct Color (i32, i32, i32);

fn main() {
    let origin = Point(0, 0, 0);
    let white = Color(255, 255, 255);

    point_operation(&origin); // Ok

    point_operation(&white); // Error!

    // Each struct you define is its own type, even though 
    // the fields within the struct have the same types. 
    // A function that takes a parameter of type `Color` 
    // cannot take a `Point` as an argument, even though 
    // both types are made up of three `i32` values.

}

fn point_operation(pt: &Point) {}
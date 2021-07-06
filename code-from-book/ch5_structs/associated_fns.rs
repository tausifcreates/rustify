#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    // we’re allowed to define functions within `impl` blocks
    // that don’t take `self` as a parameter. These are called
    // *associated functions* because they’re associated with
    // the struct. They’re still functions, not methods, because
    // they don’t have an instance of the struct to work with.

    // Associated functions are often used for constructors that
    // will return a new instance of the struct.

    fn square(size: u32) -> Rectangle {
        Rectangle {
            height: size,
            width: size,
        }
    }
}

fn main() {
    let rect = Rectangle::square(5);

    // To call this associated function,
    // we use the `::` syntax with the struct name. This function
    // is namespaced by the struct: the `::` syntax is used for
    // both associated functions and namespaces created by modules.

    println!("{:?}", rect);
}

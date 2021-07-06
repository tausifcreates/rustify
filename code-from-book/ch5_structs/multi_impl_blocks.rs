#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            height: size,
            width: size,
        }
    }
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.height * self.width;
    }
}

fn main() {
    let rect = Rectangle::square(5);

    // To call this associated function,
    // we use the `::` syntax with the struct name. This function
    // is namespaced by the struct: the `::` syntax is used for
    // both associated functions and namespaces created by modules.

    println!("{:?}", rect);

    println!("{}", rect.area());
}

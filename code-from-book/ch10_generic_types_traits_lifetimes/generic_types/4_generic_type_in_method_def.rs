#[allow(dead_code)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

#[allow(unused_variables)]
fn main() {
    let integer = Point { x: 5, y: 10 };

    let x: &i32 = integer.x();

    println!("p.x = {}", x);
}

#[allow(dead_code)]
struct Point<T, U> {
    x: T,
    y: U,
}

// Here, the generic parameters `T` and `U` are declared after `impl`,
// because they go with the struct definition. The generic parameters
// `V` and `W` are declared after `fn mixup`, because they’re only 
// relevant to the method.
impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

#[allow(unused_variables)]
fn main() {
    let p1: Point<i32, &str> = Point { x: 5, y: "hello" };

    let p2: Point<f64, &str> = Point { x: 2.5, y: "ïno" };

    let p3: Point<i32, &str> = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

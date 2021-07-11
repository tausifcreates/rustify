trait Measurement {
    fn cmp(&self) -> String;
}

trait Initialize<T> {
    fn new(x: T, y: T) -> Self;
}

struct Point<T> {
    x: T,
    y: T,
}

// `Point<T>` will always implement the `Initialize` trait.
impl<T> Initialize<T> for Point<T> {
    fn new(x: T, y: T) -> Self {
        Point {x, y}
    }
}

// `Point<T>` will implement the `Measurement` trait if `T` 
// implements the `PartialOrd` trait.
impl<T: PartialOrd> Measurement for Point<T> {
    fn cmp(&self) -> String {
        if self.x > self.y {
            format!("Ordinate is greater than abscissa")
        } else if self.x < self.y {
            format!("Abscissa is greater than ordinate")
        } else {
            format!("Abscissa and ordinate are equal")
        }
    }
}

fn main() {
    let p = Point::<i32>::new(2, 5);

    let result = p.cmp();

    println!("Comparison result: {}", result);
}

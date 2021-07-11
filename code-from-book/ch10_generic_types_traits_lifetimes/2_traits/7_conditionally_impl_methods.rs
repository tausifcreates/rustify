use ::std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

// `Pair<T>` always implements the `new` function
#[allow(dead_code)]
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Pair<T> {
        Pair { x, y }
    }
}

// `Pair<T>` only implements the `cmp_display` method if its inner type `T`
// implements the `PartialOrd` and the `Display` trait.
#[allow(dead_code)]
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x > self.y {
            println!("The largest number is {}", self.x);
        } else {
            println!("The largest number is {}", self.y);
        }
    }
}

fn main() {
    let pair: Pair<i32> = Pair::new(1, 2);

    pair.cmp_display();
}

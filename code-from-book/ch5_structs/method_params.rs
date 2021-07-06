#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    // passing ownership of struct instance
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.height > other.height && self.width > other.width
    }
}

fn main() {
    let rect1 = Rectangle {
        height: 6,
        width: 5,
    };

    let rect2 = Rectangle {
        height: 5,
        width: 4,
    };

    println!("Can rect1 hold rect2?\nResult: {}", rect1.can_hold(&rect2));
}

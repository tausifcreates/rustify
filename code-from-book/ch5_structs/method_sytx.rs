#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    // passing ownership of struct instance
    fn area(self) -> u32 {
        self.height * self.width
    }
}

fn main() {
    let r = Rectangle {
        height: 10,
        width: 12,
    };

    let area = r.area();

    println!("{:?}", r); // Will show error because already we have
                         // taken ownership in the impl block.
                         // to prevent this, we can pass `&self` as
                         // an argument, instead of `self`.
}

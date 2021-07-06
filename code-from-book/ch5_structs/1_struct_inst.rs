#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let ino = User {
        username: String::from("ino"),
        email: String::from("ino@mail.com"),
        active: false,
        sign_in_count: 1,
    };

    println!("{:?}", ino);
}

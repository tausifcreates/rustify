#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut ino = User {
        username: String::from("ino"),
        email: String::from("ino@mail.com"),
        active: false,
        sign_in_count: 1,
    };

    ino.username = String::from("inosuke");

    println!("{:?}", ino);
}
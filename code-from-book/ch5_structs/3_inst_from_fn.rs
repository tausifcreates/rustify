#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        active: false,
        sign_in_count: 0,
    }
}

fn main() {
    let ino = build_user(String::from("ino"), String::from("ino@mail.com"));

    println!("{:?}", ino);
}
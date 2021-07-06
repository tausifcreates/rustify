struct Point(i32, i32);

struct Player {
    name: String,
    position: (i32, i32),
}

fn main() {
    let mut flag = Point(12, 15);

    flag.1 = 16;

    let ino = Player {
        name: String::from("ino"),
        position: (4, 7),
    };

    assert_eq!(12, flag.0);

    assert_eq!(16, flag.1);

    assert_eq!((4, 7), ino.position);

    assert_eq!(String::from("ino"), ino.name);
}

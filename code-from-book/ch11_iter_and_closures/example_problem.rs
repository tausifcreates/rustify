#[derive(Debug)]
struct Shoe {
    size: u32,
    style: &'static str,
}

fn shoes_in_size(shoes: Vec<Shoe>, my_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == my_size).collect()
}

fn main() {
    let shoes = vec![
        Shoe { 
            size: 10,
            style: "sneaker"
        },
        Shoe {
            size: 12,
            style: "sandal"
        },
        Shoe {
            size: 10,
            style: "boot"
        }
    ];

    let my_size = 10;

    let result = shoes_in_size(shoes, my_size);

    println!("{:?}", result);
}

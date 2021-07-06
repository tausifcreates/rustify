#[derive(Debug)]
enum UsSate {
    Alabama,
    Alaska,
    // --snip--
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsSate),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // If you want to run multiple lines of code in a match arm,
        // you need to use curly brackets.
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}

fn main() {
    let some_coin = Coin::Quarter(UsSate::Alaska);
    value_in_cents(some_coin); // `State quarter from Alaska`
}

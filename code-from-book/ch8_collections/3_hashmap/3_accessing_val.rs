use std::collections::HashMap;

#[allow(unused_variables)]
fn main() {
    // We can get a value out of the hash map by providing
    // its key to the get method.

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");

    let score = scores.get(&team_name); // Some(10)

    match score {
        Some(val) => println!("{}", val),
        None => println!("Team doesn't exist")
    } // Output: 10

    // We can iterate over each key/value pair in a hash map,
    // in a similar manner as we do with vectors.
    for (key, val) in &scores {
        println!("Team: {}, Score: {}", key, val);
    }
}

use std::collections::HashMap;

#[allow(unused_variables)]
fn main() {
    // For types that implement the Copy trait, like `i32`,
    // the values are copied into the hash map. For owned
    // values like String, the values will be moved and
    // the hash map will be the owner of those values.

    let field_name = String::from("Fav Color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();

    map.insert(field_name, field_value);

    // This commented out code will produce error.
    // ---------------------------------------------
    // println!("{}, {}", field_name, field_value);
    // ---------------------------------------------
    // Error: borrow of moved value
}

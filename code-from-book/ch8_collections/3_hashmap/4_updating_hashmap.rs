use std::collections::HashMap;

#[allow(unused_variables)]
fn main() {
    // Overwriting a value
    //
    // If we insert a key and a value into a hash map and then
    // insert that same key with a different value, the value
    // associated with that key will be replaced.

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    // {"Blue": 25}

    // Only Inserting a Value If the Key Has No Value
    //
    // It’s common to check whether a particular key has a value and,
    // if it doesn’t, insert a value for it. Hash maps have a special
    // API for this called `entry` that takes the key you want to check
    // as a parameter. The return value of the `entry` method is an enum
    // called `Entry` that represents a value that might or might not exist.
    //
    // The `or_insert` method on `Entry` is defined to return a mutable
    // reference to the value for the corresponding `Entry` key if that
    // key exists, and if not, inserts the parameter as the new value
    // for this key and returns a mutable reference to the new value.

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Gray")).or_insert(25);
    // {"Gray": 25, "Blue": 10}

    scores.entry(String::from("Blue")).or_insert(15);
    // Unchanged

    // Updating a value based on old value
    //
    // Another common use case for hash maps is to look up a key’s value
    // and then update it based on the old value. This example shows code
    // that counts how many times each word appears in some text.

    let text = "you are not guilty are you";

    let mut occurance_map = HashMap::new();

    for word in text.split_whitespace() {
        println!("{:?}", occurance_map);
        let occurance_count: &mut i32 = occurance_map.entry(word).or_insert(0);
        *occurance_count += 1;
    }
    // `{"guilty": 1, "not": 1, "are": 2, "you": 2}`

    // The `or_insert` method actually returns a mutable reference `(&mut V)`
    // to the value for this key. Here we store that mutable reference in the
    // `occurance_count` variable, so in order to assign to that value, we 
    // must first dereference `occurance_count` using the asterisk (`*`). The 
    // mutable reference goes out of scope at the end of the for loop, so all
    // of these changes are safe and allowed by the borrowing rules.
}

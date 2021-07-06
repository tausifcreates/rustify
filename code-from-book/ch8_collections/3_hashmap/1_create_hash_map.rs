use std::collections::HashMap;

#[allow(unused_variables)]
fn main() {
    // Hash maps are useful when you want to look up data not by
    // using an index, but by using a key that can be of any type.

    // like vectors, hash maps store their data on the heap, and
    // homogeneous: all of the keys must have the same type, and
    // all of the values must have the same type.

    // Creating a New Hash Map
    let mut scores = HashMap::new();

    scores.insert(String::from("ino"), 10);
    scores.insert(String::from("kai"), 12);

    // {"ino": 10, "kai": 12}

    // Another way of constructing a hash map is by using iterators
    // and the `collect` method on a vector of tuples, where each
    // tuple consists of a key and its value.

    // The `collect` method gathers data into a number of collection
    // types, including `HashMap`.

    // We could use the `zip` method to create a vector of tuples.

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 20];

    let zipped = teams.into_iter().zip(initial_scores.into_iter());
    // Zip { a: IntoIter(["Blue", "Yellow"]), b: IntoIter([10, 20]) }

    let map: HashMap<String, i32> = zipped.collect();
    // {"Blue": 10, "Yellow": 20}

    // The type annotation `HashMap<_, _>` is needed here because
    // it’s possible to collect into many different data structures.
    // For the parameters for the key and value types, however, we
    // use underscores, and Rust can infer the types that the hash
    // map contains based on the types of the data in the vectors.

    // In this case, the key type will be `String` and the value type
    // will be `i32`, just as the types were in previous example.
}

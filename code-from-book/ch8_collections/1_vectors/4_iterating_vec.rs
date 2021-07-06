fn main() {
    let v = vec![1, 2, 3, 4];

    for i in v {
        println!("{}", i);
    }
    // 1, 2, 3, 4

    let v = vec!["hi", "there", "anyone"];

    for i in v {
        println!("{}", i);
    }
    // "hi", "there", "anyone"

    let v = vec![
        String::from("ino"),
        String::from("is it"),
        String::from("you"),
    ];

    // Move of `v` occurs here because of implicit call
    // to `.into_iter()`
    for i in v {
        println!("{}", i);
    }

    // This commented out code will give us error that:
    // `value borrowed here after move`
    // ------------------------
    // println!("{:?}", v);
    // ------------------------

    // to use the vector afterwards, we need to iterate 
    // over references to each element.

    let v = vec![
        String::from("ino"),
        String::from("is it"),
        String::from("you"),
    ];

    for i in &v {
        println!("{:?}", i);
    }
    // "ino", "is it", "you"

    println!("{:?}", v); // ["ino", "is it", "you"]

    // Iterating over mutable references to elements in vector:
    for i in &mut v {
        (*i).push_str(" ");
    } 
    // ["ino ", "is it ", "you "]

    let mut v = vec![1, 2, 3, 4];

    for i in &mut v {
        *i += 1;
    }
    // [2, 3, 4, 5]
}

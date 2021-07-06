fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        // `Some(5)` match `Some(i)`, because They are the same variant.
        // The `i` binds to the value contained in `Some`, so `i` takes 
        // the value `5`. The code in the match arm is then executed, so
        // we add `1` to the value of `i` and create a new Some value with
        // our total 6 inside.
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let five = Some(5);
    let six = plus_one(five); // `Some(6)`
    let none = plus_one(None); // `None`
}

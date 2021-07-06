fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
    }
    // Error! non-exhaustive patterns: `None` not covered.

    // In the case of `Option<T>`, Rust prevents us from forgetting to
    // explicitly handle the `None` case, it protects us from assuming
    // that we have a value when we might have null.
}

fn main() {}

#[allow(dead_code)]
pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
}

// Because `back_of_house::Breakfast` has a private field, the struct
// needs to provide a public associated function that constructs an
// instance of `Breakfast` (we’ve named it `summer` here). If `Breakfast`
// didn’t have such a function, we couldn’t create an instance of
// `Breakfast` in `eat_at_restaurant` because we couldn’t set the value
// of the private `seasonal_fruit` field in `eat_at_restaurant`.
impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            seasonal_fruit: String::from("peaches"),
        }
    }
}

// In contrast, if we make an enum public, all of its variants are
// then public. We only need the `pub` before the `enum` keyword.
// Paths brought into scope with use also check privacy, like any
// other paths.
pub enum Apetizer {
    Soup,
    Salad,
}

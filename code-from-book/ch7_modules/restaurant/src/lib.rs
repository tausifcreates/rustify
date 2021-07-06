mod back_of_house;
mod front_of_house;

// We can bring a path into a scope once and then call the items
// in that path as if they’re local items with the `use`keyword.
// It's the idiomatic way to bring a function into scope with `use`.

// By using `pub use`, external code can now call the `add_to_waitlist`
// function using `hosting::add_to_waitlist`. This technique is called
// *re-exporting* because we’re bringing an item into scope but also
// making that item available for others to bring into their scope.
pub use crate::front_of_house::hosting;

// You can also bring an item into scope with `use` and a relative
// path. We have used `as` keyword to create an alias for the type
// `hosting`, otherwise we would have ended up with two `hosting`
// types the same scope and Rust wouldn’t know which one we meant
// when we used `hosting`.
#[allow(unused_imports)]
use self::front_of_house::hosting as hosting_customers;

// Unidiomatic way to bring a function into scope with `use`.
use crate::front_of_house::hosting::add_to_waitlist;

// Idiomatic way to bringing in structs, enums or other items is to
// specify the full path.
#[allow(unused_imports)]
use crate::back_of_house::Breakfast;

use crate::back_of_house::Apetizer;

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // By adding use `crate::front_of_house::hosting` in the crate root,
    // hosting is now a valid name in that scope
    hosting::add_to_waitlist();

    // This will work, because we have brought the
    // `add_to_waitlist()` function into scope with `use`. But this
    // time it isn't clear if the function is locally defined or not.
    add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = Breakfast::summer("Rye");

    // Change our mind about what bread we'd like
    meal.toast = String::from("wheat");

    // Place an order for it:
    println!("I would like a {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}

#[allow(unused_variables)]
pub fn take_apetizers() {
    // Because we made the `Appetizer` enum public, we can use the `Soup`
    // and `Salad` variants in `eat_at_restaurant`
    let order1 = Apetizer::Salad;
    let order2 = Apetizer::Soup;
}

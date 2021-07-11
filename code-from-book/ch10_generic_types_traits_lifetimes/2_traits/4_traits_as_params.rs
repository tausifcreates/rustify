use std::fmt;
use std::fmt::{Debug, Display};

trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("Read more from {}", self.summarize_author())
    }
}

struct NewsArticle {
    pub author: String,
    pub content: String,
    pub location: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

impl Display for NewsArticle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn main() {
    // This parameter accepts any type that implements the `Summary` trait.
    // In the body of `func_1`, we can call any methods on item that come
    // from the `Summary` trait, such as `summarize`.
    fn func_1(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize())
    }

    // Trait bound syntax
    fn func_2<T: Summary>(item: &T) {
        println!("Breaking news! {}", item.summarize())
    }

    // Multiple params with `impl Trait` syntax. `item1` and `item2` params
    // accept type that implements `Summary` trait.
    fn func_3(item1: &impl Summary, item2: &impl Summary) {}

    // Forcing both parameters to have the same type, that’s only possible
    // to express using a trait bound.
    fn func_4<T: Summary>(item1: &T, item2: &T) {}

    // Specifying Multiple Traits with `+` operator in `impl Trait` syntax
    fn func_5(item: &(impl Summary, impl Display)) {}

    // The `+` syntax is also valid with trait bounds on generic types:
    fn func_6<T: Summary + Display>(item: &T) {}

    // Multiple generic types:
    fn func_7<T: Display + Clone, U: Clone + Debug>(item1: &T, item2: &U) {}

    // Clearer Trait Bounds with where Clauses
    fn func_8<T, U>(item1: &T, item2: &U)
    where
        T: Display + Clone,
        U: Clone + Debug,
    {
    }
}

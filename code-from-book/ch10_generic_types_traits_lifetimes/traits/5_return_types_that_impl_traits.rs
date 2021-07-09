trait Summary {
    fn summarize_author(&self) -> String;
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

#[allow(dead_code)]
#[allow(unused_variables)]
fn main() {
    fn returns_summarizable() -> impl Summary {
        NewsArticle {
            author: String::from("ino"),
            content: String::from("Today its raining heavily."),
            location: String::from("Dhaka"),
        }
    }
    // However, you can only use `impl Trait` if you’re 
    // returning a single type.
}

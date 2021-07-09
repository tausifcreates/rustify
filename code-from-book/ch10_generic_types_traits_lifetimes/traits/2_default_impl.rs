pub trait Summary {
    fn summarize(&self) -> String {
        format!("(Read more...)")
    }
}

pub struct NewsArticle {
    pub author: String,
    pub content: String,
}

// empty `impl` block for default implementation
impl Summary for NewsArticle {}

fn main() {
    let article = NewsArticle {
        author: String::from("ino"),
        content: String::from("mara bos is dumb"),
    };

    let summary = article.summarize();

    println!("New article available! {}", summary);
}

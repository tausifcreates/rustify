pub trait Summary {
    // method signature
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("Read more from {}", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub author: String,
    pub content: String,
    pub location: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
}

// We must implement method signatures in trait items.
impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("Read more...{}", self.author)
    }

    // To override default implementation, you have to preserve the
    // method signature. You are only allowed to change the method
    // body. This will fail:
    // -----------------------------------
    // fn summarize(&self) {
    //     println!("Will not work");
    // }
    // -----------------------------------
    // Error: method `summarize` has incompatible type for trait

    // Following will work, because we only changed method body:

    fn summarize(&self) -> String {
        format!("{}, {}", self.summarize_author(), self.location)
    }
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}, {}", self.summarize_author(), self.content)
    }
}

fn main() {
    let article = NewsArticle {
        author: String::from("ino"),
        content: String::from("Today its raining heavily."),
        location: String::from("Dhaka"),
    };

    let tweet = Tweet {
        username: String::from("ino"),
        content: String::from("mara bos is dumb."),
    };

    let article_summary = article.summarize();

    let tweet_summary = tweet.summarize();

    println!("New article available! {}", article_summary);

    println!("New tweet! {}", tweet_summary);
}

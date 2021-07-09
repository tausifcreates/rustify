pub trait Summary {
    // method signature
    fn summarize(&self) -> String;
}

pub struct Tweet {
    pub username: String,
    pub content: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.content, self.username)
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("ino"),
        content: String::from("mara bos is dumb"),
    };

    let summary = tweet.summarize();

    println!("{}", summary);
}

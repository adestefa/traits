/**
* A simple example using traits.
* 6/19/2024 7:12 PM
*/

// define a news article
pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

// now let's implement the Summary for the News Article
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        // last statement without ; is returned!
        format!("{}, by {}", self.headline, self.author)
    }
}

// define a tweet
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// implement the summary trait for our tweet as well
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: by {}", self.content, self.username)
    }
}

// Define a summary trait
// traits allow us to define shared methods across different types
pub trait Summary {
    // summarize the data
    fn summarize(&self) -> String;
}

fn main() {
    println!("Hello, world!");

    let tweet = Tweet {
        username: String::from("@johndoe"),
        content: String::from("Beep Boop, I'm a Tweet."),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        author: String::from("@johnDoe"),
        headline: String::from("Don't look now!"),
        content: String::from("Let's go!"),
    };

    println!("Tweet summary: {}", tweet.summarize());
    println!("News Article summary: {}", article.summarize());
}

use aggregator::{Summary, Tweet, NewsArticle, notify};

fn main() {
    let tweet = Tweet {
        username: String::from("hello_durian_ebooks"),
        content: String::from(
            "hello world"
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley cup Championship"),
        location: String::from("Pittsburgh, PA, USA"),
        content: String::from("Article content"),
        author: String::from(
            "The Pittsburgh Penguins once again are the best hocket team in the NHL."
        )
    };

    println!("New article available! {}", article.summarize());

    notify(&tweet);
    notify(&article);
}
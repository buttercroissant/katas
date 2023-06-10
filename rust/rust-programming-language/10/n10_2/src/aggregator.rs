use core::fmt::{Debug, Display};

pub trait Summary {
    // fn summarize(&self) -> String;

    // fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }

    // fn summarize(&self) -> String {
    //     String::from("(Read more from {}...)", self.summarize_author())
    // }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// impl Summary for NewsArticle {
//     fn summarize(&self) -> String {
//         format!("{}, by {} ({})", self.headline, self.author, self.location)
//     }
// }

impl Summary for NewsArticle {}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    // fn summarize_author(&self) -> String {
    //     format!("@{}", self.username)
    // }

    // fn summarize(&self) -> String {
    //     format!("{}: {}", self.username, self.content)
    // }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Trait bound syntax alternative to the syntax sugar for impl Trait above
// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }

// Multiple Trait Bounds with the + Syntax
pub fn notifyWithMupltipleTraitBounds(item: &(impl Summary + Display)) {}
pub fn notifyWithMultipleTraitBounds2WithGenericTypes<T: Summary + Display>(item: &T) {}

// Clearer Trait Bounds with where Clauses
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 { 123 }
// with where Clause usage
fn some_function2<T, U>(t: &T, u: &U) -> i32 
where
    T: Display + Clone,
    U: Clone + Debug,
{ 123 }

// Returning Types that implement Traits 
// note: impl Trait return type only applicable for single type being returned
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("hello_world_ebooks"),
        content: String::from("some content about hello world"),
        reply: false,
        retweet: false,
    }
}

// NOT OK
// fn returns_summarizable() -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: String::from ("Two penguins watching the sunset together"),
//             location: String::from("Penguin island"),
//             author: String::from("Humanoid#301"),
//             content: String::from("Why can't we, humanoids, just enjoy the sunset like the penguins do?")
//         }
//     }
//     else { 
//         Tweet {
//         username: String::from("hello_world_ebooks"),
//         content: String::from("some content about hello world"),
//         reply: false,
//         retweet: false,
//         }
//     }
// }
use _10_2_Traits_DefiningSharedBehavior::{Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize_author());
}

// Trait As Parameters

pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Trait Bounds Syntax

pub fn notify<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

// Trait Bounds with Multiple Parameters

pub fn notify(item: &(impl Summary + Display)) {
    println!("Breaking news! {}", item.summarize());
}

// Generic Types with Trait Bounds

pub fn notify<T: Summary + Display>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Returning Types that Implement a Trait

pub fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

// Clearer Trait Bounds with Where Clauses

pub fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    // ...
}

pub fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    // ...
}

// Using Trait Bounds to Conditionally Implement Methods

use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

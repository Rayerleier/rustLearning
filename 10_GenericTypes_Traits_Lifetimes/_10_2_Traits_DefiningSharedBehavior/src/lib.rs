/*
    A trait defines the functionality a particular type has and can share with other types.
    We can use traits to define shared behavior in an abstract way.
    We can use trait bounds to specify that a generic type can be any type that has certain behavior.
*/

// Defining a trait
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

// Implementing a trait on a type
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    // Add this implementation to override the default
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
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

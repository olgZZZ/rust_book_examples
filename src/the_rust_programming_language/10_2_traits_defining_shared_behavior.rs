// Traits: Defining Shared Behavior
// https://doc.rust-lang.org/stable/book/ch10-02-traits.html





// Listing 10-12: A Summary trait that consists of the behavior provided by a summarize method
pub trait Summary {
    fn summarize(&self) -> String;
}






// Implementing a Trait on a Type

// Listing 10-13: Implementing the Summary trait on the NewsArticle and Tweet types
pub trait Summary {
    fn summarize(&self) -> String;
}

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
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}






// The only difference is that the user must bring the trait into scope as well 
// as the types. Here’s an example of how a binary crate could use our aggregator library crate:
use aggregator::{Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}





// Default Implementations

// Listing 10-14: Defining a Summary trait with a default implementation of the summarize method
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}







// As a result, we can still call the summarize method on an instance of NewsArticle, like this:
use aggregator::{self, NewsArticle, Summary};

fn main() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());
}





// For example, we could define the Summary trait to have a summarize_author method 
// whose implementation is required, and then define a summarize method that has a default 
// implementation that calls the summarize_author method:
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
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
}


// To use this version of Summary, we only need to define summarize_author when 
// we implement the trait on a type:
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
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
}





// Because we’ve implemented summarize_author, the Summary trait has given us 
// the behavior of the summarize method without requiring us to write any more code.
use aggregator::{self, Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}
// This code prints 1 new tweet: (Read more from @horse_ebooks...).









// Traits as Parameters
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}



// Trait Bound Syntax
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}




// Doing so with the impl Trait syntax looks like this:
pub fn notify(item1: &impl Summary, item2: &impl Summary) {}


// If we want to force both parameters to have the same type, however, we must use a trait bound, like this:
pub fn notify<T: Summary>(item1: &T, item2: &T) {}



// Specifying Multiple Trait Bounds with the + Syntax

// We can do so using the + syntax:
pub fn notify(item: &(impl Summary + Display)) {}


// The + syntax is also valid with trait bounds on generic types:
pub fn notify<T: Summary + Display>(item: &T) {}




// Clearer Trait Bounds with where Clauses

// So instead of writing this:
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}

// we can use a where clause, like this:
fn some_function<T, U>(t: &T, u: &U) -> i32 {
   where T: Display + Clone,
         U: Clone + Debug
}





// Returning Types that Implement Traits
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}



// However, you can only use impl Trait if you’re returning a single type. For example, 
// this code that returns either a NewsArticle or a Tweet with the return type specified 
// as impl Summary wouldn’t work:
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        }
    }
}





// Using Trait Bounds to Conditionally Implement Methods

// Listing 10-15: Conditionally implementing methods on a generic type depending on trait bounds
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
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





// The impl block in the standard library looks similar to this code:
impl<T: Display> ToString for T {
    // --snip--
}


// For example, we can turn integers into their corresponding String values like 
// this because integers implement Display:
#![allow(unused)]
fn main() {
let s = 3.to_string();
}
























// Implementing an Object-Oriented Design Pattern
// https://doc.rust-lang.org/stable/book/ch17-03-oo-design-patterns.html





/*
First, we’re going to implement the state pattern in a more traditional object-oriented way, then we’ll use an approach that’s a bit more natural in Rust. Let’s dig in to incrementally implementing a blog post workflow using the state pattern.

The final functionality will look like this:

    A blog post starts as an empty draft.
    When the draft is done, a review of the post is requested.
    When the post is approved, it gets published.
    Only published blog posts return content to print, so unapproved posts can’t accidentally be published.

*/




/*
Any other changes attempted on a post should have no effect. For example, if we try to approve a draft blog post before we’ve requested a review, the post should remain an unpublished draft.

Listing 17-11 shows this workflow in code form: this is an example usage of the API we’ll implement in a library crate named blog. This won’t compile yet because we haven’t implemented the blog crate.

Listing 17-11: Code that demonstrates the desired behavior we want our blog crate to have
Filename: src/main.rs
*/
 [This code does not compile!] 
use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}






// Defining Post and Creating a New Instance in the Draft State

/*
Then Post will hold a trait object of Box<dyn State> inside an Option<T> in a private field named state to hold the state object. You’ll see why the Option<T> is necessary in a bit.

Listing 17-12: Definition of a Post struct and a new function that creates a new Post instance, a State trait, and a Draft struct
Filename: src/lib.rs
*/
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
}

trait State {}

struct Draft {}

impl State for Draft {}







// Storing the Text of the Post Content

/*
We saw in Listing 17-11 that we want to be able to call a method named add_text and pass it a &str that is then added as the text content of the blog post. We implement this as a method, rather than exposing the content field as pub, so that later we can implement a method that will control how the content field’s data is read. The add_text method is pretty straightforward, so let’s add the implementation in Listing 17-13 to the impl Post block:

Listing 17-13: Implementing the add_text method to add text to a post’s content
Filename: src/lib.rs
*/
impl Post {
    // --snip--
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
}




// Ensuring the Content of a Draft Post Is Empty

/*
Even after we’ve called add_text and added some content to our post, we still want the content method to return an empty string slice because the post is still in the draft state, as shown on line 7 of Listing 17-11. For now, let’s implement the content method with the simplest thing that will fulfill this requirement: always returning an empty string slice. We’ll change this later once we implement the ability to change a post’s state so it can be published. So far, posts can only be in the draft state, so the post content should always be empty. Listing 17-14 shows this placeholder implementation:

Listing 17-14: Adding a placeholder implementation for the content method on Post that always returns an empty string slice
Filename: src/lib.rs
*/
impl Post {
    // --snip--
    pub fn content(&self) -> &str {
        ""
    }
}






// Requesting a Review of the Post Changes Its State

/*
Next, we need to add functionality to request a review of a post, which should change its state from Draft to PendingReview. Listing 17-15 shows this code:

Listing 17-15: Implementing request_review methods on Post and the State trait
Filename: src/lib.rs
*/
impl Post {
    // --snip--
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
}





// Adding approve to Change the Behavior of content

/*
The approve method will be similar to the request_review method: it will set state to the value that the current state says it should have when that state is approved, as shown in Listing 17-16:

Listing 17-16: Implementing the approve method on Post and the State trait
Filename: src/lib.rs
*/
impl Post {
    // --snip--
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
}

struct Draft {}

impl State for Draft {
    // --snip--
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    // --snip--
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}





/*
Now we need to update the content method on Post. We want the value returned from content to depend on the current state of the Post, so we’re going to have the Post delegate to a content method defined on its state, as shown in Listing 17-17:

Listing 17-17: Updating the content method on Post to delegate to a content method on State
Filename: src/lib.rs
*/
 [This code does not compile!] 
impl Post {
    // --snip--
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }
    // --snip--
}






/*
At this point, when we call content on the &Box<dyn State>, deref coercion will take effect on the & and the Box so the content method will ultimately be called on the type that implements the State trait. That means we need to add content to the State trait definition, and that is where we’ll put the logic for what content to return depending on which state we have, as shown in Listing 17-18:

Listing 17-18: Adding the content method to the State trait
Filename: src/lib.rs
*/
trait State {
    // --snip--
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

// --snip--
struct Published {}

impl State for Published {
    // --snip--
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}






/*

    Why Not An Enum?

    You may have been wondering why we didn’t use an enum with the different possible post states as variants. That’s certainly a possible solution, try it and compare the end results to see which you prefer! One disadvantage of using an enum is every place that checks the value of the enum will need a match expression or similar to handle every possible variant. This could get more repetitive than this trait object solution.

*/












// Encoding States and Behavior as Types

/*
Let’s consider the first part of main in Listing 17-11:

Filename: src/main.rs
*/
fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());
}







/*
We still enable the creation of new posts in the draft state using Post::new and the ability to add text to the post’s content. But instead of having a content method on a draft post that returns an empty string, we’ll make it so draft posts don’t have the content method at all. That way, if we try to get a draft post’s content, we’ll get a compiler error telling us the method doesn’t exist. As a result, it will be impossible for us to accidentally display draft post content in production, because that code won’t even compile. Listing 17-19 shows the definition of a Post struct and a DraftPost struct, as well as methods on each:

Listing 17-19: A Post with a content method and a DraftPost without a content method
Filename: src/lib.rs
*/
pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
}











// Implementing Transitions as Transformations into Different Types

/*
So how do we get a published post? We want to enforce the rule that a draft post has to be reviewed and approved before it can be published. A post in the pending review state should still not display any content. Let’s implement these constraints by adding another struct, PendingReviewPost, defining the request_review method on DraftPost to return a PendingReviewPost, and defining an approve method on PendingReviewPost to return a Post, as shown in Listing 17-20:

Listing 17-20: A PendingReviewPost that gets created by calling request_review on DraftPost and an approve method that turns a PendingReviewPost into a published Post
Filename: src/lib.rs
*/
impl DraftPost {
    // --snip--
    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}








/*
But we also have to make some small changes to main. The request_review and approve methods return new instances rather than modifying the struct they’re called on, so we need to add more let post = shadowing assignments to save the returned instances. We also can’t have the assertions about the draft and pending review posts’ contents be empty strings, nor do we need them: we can’t compile code that tries to use the content of posts in those states any longer. The updated code in main is shown in Listing 17-21:

Listing 17-21: Modifications to main to use the new implementation of the blog post workflow
Filename: src/main.rs
*/
use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
}





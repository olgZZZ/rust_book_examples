// Meta
// https://doc.rust-lang.org/rust-by-example/meta/doc.html

/*
    Some topics aren't exactly relevant to how you program but provide you tooling or infrastructure support which just makes things better for everyone. These topics include:

        Documentation: Generate library documentation for users via the included rustdoc.
        Playpen: Integrate the Rust Playpen (also known as the Rust Playground) in your documentation.
*/


/*
    Documentation

    Use cargo doc to build documentation in target/doc.

    Use cargo test to run all tests (including documentation tests), and cargo test --doc to only run documentation tests.

    These commands will appropriately invoke rustdoc (and rustc) as required.
*/


/*
    Doc comments

    Doc comments are very useful for big projects that require documentation. When running rustdoc, these are the comments that get compiled into documentation. They are denoted by a ///, and support Markdown.
*/

#![crate_name = "doc"]

/// A human being is represented here
pub struct Person {
    /// A person must have a name, no matter how much Juliet may hate it
    name: String,
}

impl Person {
    /// Returns a person with the name given them
    ///
    /// # Arguments
    ///
    /// * `name` - A string slice that holds the name of the person
    ///
    /// # Examples
    ///
    /// ```
    /// // You can have rust code between fences inside the comments
    /// // If you pass --test to `rustdoc`, it will even test it for you!
    /// use doc::Person;
    /// let person = Person::new("name");
    /// ```
    pub fn new(name: &str) -> Person {
        Person {
            name: name.to_string(),
        }
    }

    /// Gives a friendly hello!
    ///
    /// Says "Hello, [name]" to the `Person` it is called on.
    pub fn hello(& self) {
        println!("Hello, {}!", self.name);
    }
}

fn main() {
    let john = Person::new("John");

    john.hello();
}



/*
    To run the tests, first build the code as a library, then tell rustdoc where to find the library so it can link it into each doctest program:

    $ rustc doc.rs --crate-type lib
    $ rustdoc --test --extern doc="libdoc.rlib" doc.rs
*/





/*
    Doc attributes

    Below are a few examples of the most common #[doc] attributes used with rustdoc.
    inline

    Used to inline docs, instead of linking out to separate page.
*/

#[doc(inline)]
pub use bar::Bar;

/// bar docs
mod bar {
    /// the docs for Bar
    pub struct Bar;
}





/*
    no_inline

    Used to prevent linking out to separate page or anywhere.
*/

// Example from libcore/prelude
#[doc(no_inline)]
pub use crate::mem::drop;







/*
    hidden

    Using this tells rustdoc not to include this in documentation:
*/

// Example from the futures-rs library
#[doc(hidden)]
pub use self::async_await::*;



/*
    For documentation, rustdoc is widely used by the community. It's what is used to generate the std library docs.
    See also:

        The Rust Book: Making Useful Documentation Comments
        The rustdoc Book
        The Reference: Doc comments
        RFC 1574: API Documentation Conventions
        RFC 1946: Relative links to other items from doc comments (intra-rustdoc links)
        Is there any documentation style guide for comments? (reddit)
*/


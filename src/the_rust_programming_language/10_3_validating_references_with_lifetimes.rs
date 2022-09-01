// Validating References with Lifetimes
// https://doc.rust-lang.org/stable/book/ch10-03-lifetime-syntax.html



// Preventing Dangling References with Lifetimes

// Listing 10-16: An attempt to use a reference whose value has gone out of scope
fn main() {
    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r: {}", r);
}

/* Note: The examples in Listings 10-16, 10-17, and 10-23 declare variables without giving them an initial value, so the variable name exists in the outer scope. At first glance, this might appear to be in conflict with Rust’s having no null values. However, if we try to use a variable before giving it a value, we’ll get a compile-time error, which shows that Rust indeed does not allow null values.
*/

/*
$ cargo run
   Compiling chapter10 v0.1.0 (file:///projects/chapter10)
error[E0597]: `x` does not live long enough
 --> src/main.rs:6:13
  |
6 |         r = &x;
  |             ^^ borrowed value does not live long enough
7 |     }
  |     - `x` dropped here while still borrowed
8 | 
9 |     println!("r: {}", r);
  |                       - borrow later used here

For more information about this error, try `rustc --explain E0597`.
error: could not compile `chapter10` due to previous error

*/





// The Borrow Checker

// Listing 10-17: Annotations of the lifetimes of r and x, named 'a and 'b, respectively
fn main() {
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
}                         // ---------+



// Listing 10-18 fixes the code so it doesn’t have a dangling reference and compiles without any errors.
// Listing 10-18: A valid reference because the data has a longer lifetime than the reference
fn main() {
    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;           // --+-- 'a  |
                          //   |       |
    println!("r: {}", r); //   |       |
                          // --+       |
}                         // ----------+







// Generic Lifetimes in Functions

// Listing 10-19: A main function that calls the longest function to find the longer of two string slices
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}



// Listing 10-20: An implementation of the longest function that returns the 
// longer of two string slices but does not yet compile
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}


// Instead, we get the following error that talks about lifetimes:

/*
$ cargo run
   Compiling chapter10 v0.1.0 (file:///projects/chapter10)
error[E0106]: missing lifetime specifier
 --> src/main.rs:9:33
  |
9 | fn longest(x: &str, y: &str) -> &str {
  |               ----     ----     ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
help: consider introducing a named lifetime parameter
  |
9 | fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  |           ++++     ++          ++          ++

For more information about this error, try `rustc --explain E0106`.
error: could not compile `chapter10` due to previous error

*/







// Lifetime Annotation Syntax

// Here are some examples: a reference to an i32 without a lifetime parameter, a reference 
// to an i32 that has a lifetime parameter named 'a, and a mutable reference to an i32 that 
// also has the lifetime 'a.
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime





// Lifetime Annotations in Function Signatures

// Listing 10-21: The longest function definition specifying that all the references 
// in the signature must have the same lifetime 'a
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}






// Listing 10-22: Using the longest function with references to String values that 
// have different concrete lifetimes
fn main() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}






// Listing 10-23: Attempting to use result after string2 has gone out of scope
fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}


// When we try to compile this code, we get this error:

/*
$ cargo run
   Compiling chapter10 v0.1.0 (file:///projects/chapter10)
error[E0597]: `string2` does not live long enough
 --> src/main.rs:6:44
  |
6 |         result = longest(string1.as_str(), string2.as_str());
  |                                            ^^^^^^^^^^^^^^^^ borrowed value does not live long enough
7 |     }
  |     - `string2` dropped here while still borrowed
8 |     println!("The longest string is {}", result);
  |                                          ------ borrow later used here

For more information about this error, try `rustc --explain E0597`.
error: could not compile `chapter10` due to previous error

*/



// Thinking in Terms of Lifetimes
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}




// Consider this attempted implementation of the longest function that won’t compile:
fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}

/*
$ cargo run
   Compiling chapter10 v0.1.0 (file:///projects/chapter10)
error[E0515]: cannot return reference to local variable `result`
  --> src/main.rs:11:5
   |
11 |     result.as_str()
   |     ^^^^^^^^^^^^^^^ returns a reference to data owned by the current function

For more information about this error, try `rustc --explain E0515`.
error: could not compile `chapter10` due to previous error

*/






// Lifetime Annotations in Struct Definitions

// Listing 10-24: A struct that holds a reference, requiring a lifetime annotation
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}






// Lifetime Elision

// Listing 10-25: A function we defined in Listing 4-9 that compiled without 
// lifetime annotations, even though the parameter and return type are references
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}


// At that time, the function signature would have been written like this:
fn first_word<'a>(s: &'a str) -> &'a str {}




// The signature starts without any lifetimes associated with the references:
fn first_word(s: &str) -> &str {}


// Then the compiler applies the first rule, which specifies that each parameter gets 
// its own lifetime. We’ll call it 'a as usual, so now the signature is this:
fn first_word<'a>(s: &'a str) -> &str {}


// The second rule applies because there is exactly one input lifetime. The second rule 
// specifies that the lifetime of the one input parameter gets assigned to the output 
// lifetime, so the signature is now this:
fn first_word<'a>(s: &'a str) -> &'a str {}


// Let’s look at another example, this time using the longest function that had no lifetime 
// parameters when we started working with it in Listing 10-20:
fn longest(x: &str, y: &str) -> &str {}


// Let’s apply the first rule: each parameter gets its own lifetime. This time we have two 
// parameters instead of one, so we have two lifetimes:
fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {}





// Lifetime Annotations in Method Definitions

// First, we’ll use a method named level whose only parameter is a reference to self and 
// whose return value is an i32, which is not a reference to anything:
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}


// Here is an example where the third lifetime elision rule applies:
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}




// The Static Lifetime
let s: &'static str = "I have a static lifetime.";






// Generic Type Parameters, Trait Bounds, and Lifetimes Together
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}




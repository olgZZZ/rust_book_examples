// Bringing Paths into Scope with the use Keyword
// https://doc.rust-lang.org/stable/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html




// Listing 7-11: Bringing a module into scope with use
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}



// Listing 7-12: A use statement only applies in the scope it’s in
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

mod customer {
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}




// The compiler error shows that the shortcut no longer applies within the customer module:

/*
$ cargo build
   Compiling restaurant v0.1.0 (file:///projects/restaurant)
error[E0433]: failed to resolve: use of undeclared crate or module `hosting`
  --> src/lib.rs:11:9
   |
11 |         hosting::add_to_waitlist();
   |         ^^^^^^^ use of undeclared crate or module `hosting`

warning: unused import: `crate::front_of_house::hosting`
 --> src/lib.rs:7:5
  |
7 | use crate::front_of_house::hosting;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

For more information about this error, try `rustc --explain E0433`.
warning: `restaurant` (lib) generated 1 warning
error: could not compile `restaurant` due to previous error; 1 warning emitted

*/






// Creating Idiomatic use Paths

// Listing 7-13: Bringing the add_to_waitlist function into scope with use, which is unidiomatic
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    add_to_waitlist();
}



// Listing 7-14: Bringing HashMap into scope in an idiomatic way
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}




// Listing 7-15: Bringing two types with the same name into the same scope requires using their parent modules.
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
    Ok(())
}

fn function2() -> io::Result<()> {
    // --snip--
    Ok(())
}





// Providing New Names with the as Keyword

// Listing 7-16: Renaming a type when it’s brought into scope with the as keyword
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
    Ok(())
}

fn function2() -> IoResult<()> {
    // --snip--
    Ok(())
}






// Re-exporting Names with pub use

// Listing 7-17: Making a name available for any code to use from a new scope with pub use
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}






// Using External Packages

// Filename: Cargo.toml
rand = "0.8.3"


// Recall that in the “Generating a Random Number” section in Chapter 2, we brought 
// the Rng trait into scope and called the rand::thread_rng function:
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}




// This is an absolute path starting with std, the name of the standard library crate.
#![allow(unused)]
fn main() {
use std::collections::HashMap;
}






// Using Nested Paths to Clean Up Large use Lists


// For example, these two use statements we had in the Guessing Game in 
// Listing 2-4 bring items from std into scope:

// --snip--
use std::cmp::Ordering;
use std::io;
// --snip--






// Listing 7-18: Specifying a nested path to bring multiple items with the same prefix into scope

// --snip--
use std::{cmp::Ordering, io};
// --snip--



// Listing 7-19: Two use statements where one is a subpath of the other
// Filename: src/lib.rs
use std::io;
use std::io::Write;



// Listing 7-20: Combining the paths in Listing 7-19 into one use statement
// Filename: src/lib.rs
// This line brings std::io and std::io::Write into scope.
use std::io::{self, Write};





// The Glob Operator
use std::collections::*;












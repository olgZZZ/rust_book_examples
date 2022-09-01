// Defining Modules to Control Scope and Privacy
// https://doc.rust-lang.org/stable/book/ch07-02-defining-modules-to-control-scope-and-privacy.html



/*
backyard
├── Cargo.lock
├── Cargo.toml
└── src
    ├── garden
    │   └── vegetables.rs
    ├── garden.rs
    └── main.rs
*/



// The crate root file in this case is src/main.rs, and it contains:
use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}




// Filename: src/garden.rs
pub mod vegetables;



// Here, pub mod vegetables; means the code in src/garden/vegetables.rs is included too. That code is:
#[derive(Debug)]
pub struct Asparagus {}






// Grouping Related Code in Modules

// Listing 7-1: A front_of_house module containing other modules that then contain functions
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}




// Listing 7-2 shows the module tree for the structure in Listing 7-1.

/*
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment

*/


































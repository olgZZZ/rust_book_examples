// Separating Modules into Different Files
// https://doc.rust-lang.org/stable/book/ch07-05-separating-modules-into-different-files.html




// Listing 7-21: Declaring the front_of_house module whose body will be in src/front_of_house.rs
// Filename: src/lib.rs
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}





// Listing 7-22: Definitions inside the front_of_house module in src/front_of_house.rs
// Filename: src/front_of_house.rs
pub mod hosting {
    pub fn add_to_waitlist() {}
}


// Filename: src/front_of_house.rs
pub mod hosting;

// Filename: src/front_of_house/hosting.rs
pub fn add_to_waitlist() {}










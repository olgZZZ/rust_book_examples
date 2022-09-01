// Publishing a Crate to Crates.io
// https://doc.rust-lang.org/stable/book/ch14-02-publishing-to-crates-io.html






// Making Useful Documentation Comments

/*
Documentation comments use three slashes, ///, instead of two and support Markdown notation for formatting the text. Place documentation comments just before the item they’re documenting. Listing 14-1 shows documentation comments for an add_one function in a crate named my_crate.

Filename: src/lib.rs
*/

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}





// Commonly Used Sections

// Documentation Comments as Tests

/*
If we run cargo test with the documentation for the add_one function from Listing 14-1, we will see a section in the test results like this:

   Doc-tests my_crate

running 1 test
test src/lib.rs - add_one (line 5) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.27s

*/







// Commenting Contained Items

/*
For example, to add documentation that describes the purpose of the my_crate crate that contains the add_one function, we add documentation comments that start with //! to the beginning of the src/lib.rs file, as shown in Listing 14-2:

Filename: src/lib.rs
*/

//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.

/// Adds one to the number given.
// --snip--





// Exporting a Convenient Public API with pub use

/*
For example, say we made a library named art for modeling artistic concepts. Within this library are two modules: a kinds module containing two enums named PrimaryColor and SecondaryColor and a utils module containing a function named mix, as shown in Listing 14-3:

Filename: src/lib.rs
*/

//! # Art
//!
//! A library for modeling artistic concepts.

pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        // --snip--
        unimplemented!();
    }
}









/*
Another crate that depends on this library would need use statements that bring the items from art into scope, specifying the module structure that’s currently defined. Listing 14-4 shows an example of a crate that uses the PrimaryColor and mix items from the art crate:

Filename: src/main.rs
*/
use art::kinds::PrimaryColor;
use art::utils::mix;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}




/*
To remove the internal organization from the public API, we can modify the art crate code in Listing 14-3 to add pub use statements to re-export the items at the top level, as shown in Listing 14-5:

Filename: src/lib.rs
*/

//! # Art
//!
//! A library for modeling artistic concepts.

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    // --snip--
}

pub mod utils {
    // --snip--
}





/*
The art crate users can still see and use the internal structure from Listing 14-3 as demonstrated in Listing 14-4, or they can use the more convenient structure in Listing 14-5, as shown in Listing 14-6:

Filename: src/main.rs
*/

use art::mix;
use art::PrimaryColor;

fn main() {
    // --snip--
}




// Setting Up a Crates.io Account

// $ cargo login abcdefghijklmnopqrstuvwxyz012345







// Adding Metadata to a New Crate

/*
If the name has been used, you will need to find another name and edit the name field in the Cargo.toml file under the [package] section to use the new name for publishing, like so:

Filename: Cargo.toml

[package]
name = "guessing_game"
*/


/*
$ cargo publish
    Updating crates.io index
warning: manifest has no description, license, license-file, documentation, homepage or repository.
See https://doc.rust-lang.org/cargo/reference/manifest.html#package-metadata for more info.
--snip--
error: failed to publish to registry at https://crates.io

Caused by:
  the remote server responded with an error: missing or empty metadata fields: description, license. Please see https://doc.rust-lang.org/cargo/reference/manifest.html for how to upload metadata

*/





/*
The Linux Foundation’s Software Package Data Exchange (SPDX) lists the identifiers you can use for this value. For example, to specify that you’ve licensed your crate using the MIT License, add the MIT identifier:

Filename: Cargo.toml

[package]
name = "guessing_game"
license = "MIT"
*/






/*
With a unique name, the version, your description, and a license added, the Cargo.toml file for a project that is ready to publish might look like this:

Filename: Cargo.toml

[package]
name = "guessing_game"
version = "0.1.0"
edition = "2021"
description = "A fun game where you guess what number the computer has chosen."
license = "MIT OR Apache-2.0"

[dependencies]

*/






// Publishing to Crates.io

/*
Run the cargo publish command again. It should succeed now:

$ cargo publish
    Updating crates.io index
   Packaging guessing_game v0.1.0 (file:///projects/guessing_game)
   Verifying guessing_game v0.1.0 (file:///projects/guessing_game)
   Compiling guessing_game v0.1.0
(file:///projects/guessing_game/target/package/guessing_game-0.1.0)
    Finished dev [unoptimized + debuginfo] target(s) in 0.19s
   Uploading guessing_game v0.1.0 (file:///projects/guessing_game)

*/









// Deprecating Versions from Crates.io with cargo yank

/*
To yank a version of a crate, in the directory of the crate that you’ve previously published, run cargo yank and specify which version you want to yank. For example, if we've published a crate named guessing_game version 1.0.1 and we want to yank it, in the project directory for guessing_game we'd run:

$ cargo yank --vers 1.0.1
    Updating crates.io index
        Yank guessing_game:1.0.1


By adding --undo to the command, you can also undo a yank and allow projects to start depending on a version again:

$ cargo yank --vers 1.0.1 --undo
    Updating crates.io index
      Unyank guessing_game_:1.0.1
*/




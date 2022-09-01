// Cargo Workspaces
// https://doc.rust-lang.org/stable/book/ch14-03-cargo-workspaces.html





// Creating a Workspace

/*
$ mkdir add
$ cd add

*/


/*
Filename: Cargo.toml

[workspace]

members = [
    "adder",
]
*/




/*
Next, we’ll create the adder binary crate by running cargo new within the add directory:

$ cargo new adder
     Created binary (application) `adder` package
*/




/*
At this point, we can build the workspace by running cargo build. The files in your add directory should look like this:

├── Cargo.lock
├── Cargo.toml
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
*/









// Creating the Second Package in the Workspace



/*
Next, let’s create another member package in the workspace and call it add_one. Change the top-level Cargo.toml to specify the add_one path in the members list:

Filename: Cargo.toml

[workspace]

members = [
    "adder",
    "add_one",
]
*/



/*
Then generate a new library crate named add_one:

$ cargo new add_one --lib
     Created library `add_one` package
*/




/*
Your add directory should now have these directories and files:

├── Cargo.lock
├── Cargo.toml
├── add_one
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
*/




/*
In the add_one/src/lib.rs file, let’s add an add_one function:

Filename: add_one/src/lib.rs

pub fn add_one(x: i32) -> i32 {
    x + 1
}
*/




/*
Now we can have the adder package with our binary depend on the add_one package that has our library. First, we’ll need to add a path dependency on add_one to adder/Cargo.toml.

Filename: adder/Cargo.toml

[dependencies]
add_one = { path = "../add_one" }
*/




/*
Next, let’s use the add_one function (from the add_one crate) in the adder crate. Open the adder/src/main.rs file and add a use line at the top to bring the new add_one library crate into scope. Then change the main function to call the add_one function, as in Listing 14-7.

Filename: adder/src/main.rs

use add_one;

fn main() {
    let num = 10;
    println!(
        "Hello, world! {num} plus one is {}!",
        add_one::add_one(num)
    );
}
*/



/*
Let’s build the workspace by running cargo build in the top-level add directory!

$ cargo build
   Compiling add_one v0.1.0 (file:///projects/add/add_one)
   Compiling adder v0.1.0 (file:///projects/add/adder)
    Finished dev [unoptimized + debuginfo] target(s) in 0.68s
*/




/*
To run the binary crate from the add directory, we can specify which package in the workspace we want to run by using the -p argument and the package name with cargo run:

$ cargo run -p adder
    Finished dev [unoptimized + debuginfo] target(s) in 0.0s
     Running `target/debug/adder`
Hello, world! 10 plus one is 11!
*/







// Depending on an External Package in a Workspace



/*
Filename: add_one/Cargo.toml

[dependencies]
rand = "0.8.3"
*/


/*
We can now add use rand; to the add_one/src/lib.rs file, and building the whole workspace by running cargo build in the add directory will bring in and compile the rand crate. We will get one warning because we aren’t referring to the rand we brought into scope:

$ cargo build
    Updating crates.io index
  Downloaded rand v0.8.3
   --snip--
   Compiling rand v0.8.3
   Compiling add_one v0.1.0 (file:///projects/add/add_one)
warning: unused import: `rand`
 --> add_one/src/lib.rs:1:5
  |
1 | use rand;
  |     ^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: 1 warning emitted

   Compiling adder v0.1.0 (file:///projects/add/adder)
    Finished dev [unoptimized + debuginfo] target(s) in 10.18s
*/






/*
The top-level Cargo.lock now contains information about the dependency of add_one on rand. However, even though rand is used somewhere in the workspace, we can’t use it in other crates in the workspace unless we add rand to their Cargo.toml files as well. For example, if we add use rand; to the adder/src/main.rs file for the adder package, we’ll get an error:

$ cargo build
  --snip--
   Compiling adder v0.1.0 (file:///projects/add/adder)
error[E0432]: unresolved import `rand`
 --> adder/src/main.rs:2:5
  |
2 | use rand;
  |     ^^^^ no external crate `rand`
*/









// Adding a Test to a Workspace





/*
For another enhancement, let’s add a test of the add_one::add_one function within the add_one crate:

Filename: add_one/src/lib.rs

pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, add_one(2));
    }
}
*/




/*
Now run cargo test in the top-level add directory. Running cargo test in a workspace structured like this one will run the tests for all the crates in the workspace:

$ cargo test
   Compiling add_one v0.1.0 (file:///projects/add/add_one)
   Compiling adder v0.1.0 (file:///projects/add/adder)
    Finished test [unoptimized + debuginfo] target(s) in 0.27s
     Running target/debug/deps/add_one-f0253159197f7841

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running target/debug/deps/adder-49979ff40686fa8e

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests add_one

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
*/





/*
We can also run tests for one particular crate in a workspace from the top-level directory by using the -p flag and specifying the name of the crate we want to test:

$ cargo test -p add_one
    Finished test [unoptimized + debuginfo] target(s) in 0.00s
     Running target/debug/deps/add_one-b3235fea9a156f74

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests add_one

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
*/






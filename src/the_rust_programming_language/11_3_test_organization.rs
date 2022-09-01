// Test Organization
// https://doc.rust-lang.org/stable/book/ch11-03-test-organization.html





// Unit Tests

// The Tests Module and #[cfg(test)]

/*
Recall that when we generated the new adder project in the first section of this chapter, Cargo generated this code for us:

Filename: src/lib.rs
*/
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}




// Testing Private Functions

/*
There’s debate within the testing community about whether or not private functions should be tested directly, and other languages make it difficult or impossible to test private functions. Regardless of which testing ideology you adhere to, Rust’s privacy rules do allow you to test private functions. Consider the code in Listing 11-12 with the private function internal_adder.

Filename: src/lib.rs
*/
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}







// Integration Tests

// The tests Directory

/*
Let’s create an integration test. With the code in Listing 11-12 still in the src/lib.rs file, make a tests directory, and create a new file named tests/integration_test.rs. Your directory structure should look like this:

adder
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    └── integration_test.rs

*/


// Listing 11-13: An integration test of a function in the adder crate
use adder;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}





/*
We don’t need to annotate any code in tests/integration_test.rs with #[cfg(test)]. Cargo treats the tests directory specially and compiles files in this directory only when we run cargo test. Run cargo test now:

$ cargo test
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished test [unoptimized + debuginfo] target(s) in 1.31s
     Running unittests src/lib.rs (target/debug/deps/adder-1082c4b063a8fbe6)

running 1 test
test tests::internal ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/integration_test.rs (target/debug/deps/integration_test-1082c4b063a8fbe6)

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

*/



/*
We can still run a particular integration test function by specifying the test function’s name as an argument to cargo test. To run all the tests in a particular integration test file, use the --test argument of cargo test followed by the name of the file:

$ cargo test --test integration_test
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished test [unoptimized + debuginfo] target(s) in 0.64s
     Running tests/integration_test.rs (target/debug/deps/integration_test-82e7799c1bc62298)

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


*/




// Submodules in Integration Tests

/*
The different behavior of tests directory files is most noticeable when you have a set of helper functions to use in multiple integration test files and you try to follow the steps in the “Separating Modules into Different Files” section of Chapter 7 to extract them into a common module. For example, if we create tests/common.rs and place a function named setup in it, we can add some code to setup that we want to call from multiple test functions in multiple test files:

Filename: tests/common.rs
*/
pub fn setup() {
    // setup code specific to your library's tests would go here
}




/*
When we run the tests again, we’ll see a new section in the test output for the common.rs file, even though this file doesn’t contain any test functions nor did we call the setup function from anywhere:

$ cargo test
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished test [unoptimized + debuginfo] target(s) in 0.89s
     Running unittests src/lib.rs (target/debug/deps/adder-92948b65e88960b4)

running 1 test
test tests::internal ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/common.rs (target/debug/deps/common-92948b65e88960b4)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/integration_test.rs (target/debug/deps/integration_test-92948b65e88960b4)

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

*/



/*
To avoid having common appear in the test output, instead of creating tests/common.rs, we’ll create tests/common/mod.rs. The project directory now looks like this:

├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    ├── common
    │   └── mod.rs
    └── integration_test.rs

*/





/*
After we’ve created tests/common/mod.rs, we can use it from any of the integration test files as a module. Here’s an example of calling the setup function from the it_adds_two test in tests/integration_test.rs:

Filename: tests/integration_test.rs
*/
use adder;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}








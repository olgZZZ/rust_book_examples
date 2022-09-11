// Integration testing
// https://doc.rust-lang.org/rust-by-example/testing/integration_testing.html


/*
    Unit tests are testing one module in isolation at a time: they're small and can test private code. Integration tests are external to your crate and use only its public interface in the same way any other code would. Their purpose is to test that many parts of your library work correctly together.

    Cargo looks for integration tests in tests directory next to src.

    File src/lib.rs:
*/

// Define this in a crate called `adder`.
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}


// File with test: tests/integration_test.rs:

#[test]
fn test_add() {
    assert_eq!(adder::add(3, 2), 5);
}



/*
    Running tests with cargo test command:

    $ cargo test
    running 0 tests

    test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

         Running target/debug/deps/integration_test-bcd60824f5fbfe19

    running 1 test
    test test_add ... ok

    test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

       Doc-tests adder

    running 0 tests

    test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

    Each Rust source file in the tests directory is compiled as a separate crate. One way of sharing some code between integration tests is making a module with public functions, importing and using it within tests.
*/




// File tests/common.rs:

pub fn setup() {
    // some setup code, like creating required files/directories, starting
    // servers, etc.
}




// File with test: tests/integration_test.rs

// importing common module.
mod common;

#[test]
fn test_add() {
    // using common code.
    common::setup();
    assert_eq!(adder::add(3, 2), 5);
}





/*
    Modules with common code follow the ordinary modules rules, so it's ok to create common module as tests/common/mod.rs.
*/



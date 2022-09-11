// Documentation testing
// https://doc.rust-lang.org/rust-by-example/testing/doc_testing.html


/*
    The primary way of documenting a Rust project is through annotating the source code. Documentation comments are written in markdown and support code blocks in them. Rust takes care about correctness, so these code blocks are compiled and used as documentation tests.
*/

/// First line is a short summary describing function.
///
/// The next lines present detailed documentation. Code blocks start with
/// triple backquotes and have implicit `fn main()` inside
/// and `extern crate <cratename>`. Assume we're testing `doccomments` crate:
///
/// ```
/// let result = doccomments::add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Usually doc comments may include sections "Examples", "Panics" and "Failures".
///
/// The next function divides two numbers.
///
/// # Examples
///
/// ```
/// let result = doccomments::div(10, 2);
/// assert_eq!(result, 5);
/// ```
///
/// # Panics
///
/// The function panics if the second argument is zero.
///
/// ```rust,should_panic
/// // panics on division by zero
/// doccomments::div(10, 0);
/// ```
pub fn div(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Divide-by-zero error");
    }

    a / b
}



/*
    Code blocks in documentation are automatically tested when running the regular cargo test command:

    $ cargo test
    running 0 tests

    test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

       Doc-tests doccomments

    running 3 tests
    test src/lib.rs - add (line 7) ... ok
    test src/lib.rs - div (line 21) ... ok
    test src/lib.rs - div (line 31) ... ok

    test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
*/





/*
    Motivation behind documentation tests

    The main purpose of documentation tests is to serve as examples that exercise the functionality, which is one of the most important guidelines. It allows using examples from docs as complete code snippets. But using ? makes compilation fail since main returns unit. The ability to hide some source lines from documentation comes to the rescue: one may write fn try_main() -> Result<(), ErrorType>, hide it and unwrap it in hidden main. Sounds complicated? Here's an example:
*/


/// Using hidden `try_main` in doc tests.
///
/// ```
/// # // hidden lines start with `#` symbol, but they're still compileable!
/// # fn try_main() -> Result<(), String> { // line that wraps the body shown in doc
/// let res = try::try_div(10, 2)?;
/// # Ok(()) // returning from try_main
/// # }
/// # fn main() { // starting main that'll unwrap()
/// #    try_main().unwrap(); // calling try_main and unwrapping
/// #                         // so that test will panic in case of error
/// # }
/// ```
pub fn try_div(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Divide-by-zero"))
    } else {
        Ok(a / b)
    }
}


/*
    See Also

        RFC505 on documentation style
        API Guidelines on documentation guidelines
*/

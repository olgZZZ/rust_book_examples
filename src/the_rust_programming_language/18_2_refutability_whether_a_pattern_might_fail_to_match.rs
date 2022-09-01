// Refutability: Whether a Pattern Might Fail to Match
// https://doc.rust-lang.org/stable/book/ch18-02-refutability.html





/*
Let’s look at an example of what happens when we try to use a refutable pattern where Rust requires an irrefutable pattern and vice versa. Listing 18-8 shows a let statement, but for the pattern we’ve specified Some(x), a refutable pattern. As you might expect, this code will not compile.

Listing 18-8: Attempting to use a refutable pattern with let
*/
    let Some(x) = some_option_value;




/*
If some_option_value was a None value, it would fail to match the pattern Some(x), meaning the pattern is refutable. However, the let statement can only accept an irrefutable pattern because there is nothing valid the code can do with a None value. At compile time, Rust will complain that we’ve tried to use a refutable pattern where an irrefutable pattern is required:

$ cargo run
   Compiling patterns v0.1.0 (file:///projects/patterns)
error[E0005]: refutable pattern in local binding: `None` not covered
   --> src/main.rs:3:9
    |
3   |     let Some(x) = some_option_value;
    |         ^^^^^^^ pattern `None` not covered
    |
    = note: `let` bindings require an "irrefutable pattern", like a `struct` or an `enum` with only one variant
    = note: for more information, visit https://doc.rust-lang.org/book/ch18-02-refutability.html
note: `Option<i32>` defined here
    = note: the matched value is of type `Option<i32>`
help: you might want to use `if let` to ignore the variant that isn't matched
    |
3   |     let x = if let Some(x) = some_option_value { x } else { todo!() };
    |     ++++++++++                                 ++++++++++++++++++++++

For more information about this error, try `rustc --explain E0005`.
error: could not compile `patterns` due to previous error
*/







/*
If we have a refutable pattern where an irrefutable pattern is needed, we can fix it by changing the code that uses the pattern: instead of using let, we can use if let. Then if the pattern doesn’t match, the code will just skip the code in the curly brackets, giving it a way to continue validly. Listing 18-9 shows how to fix the code in Listing 18-8.

Listing 18-9: Using if let and a block with refutable patterns instead of let
*/
    if let Some(x) = some_option_value {
        println!("{}", x);
    }




/*
Rust complains that it doesn’t make sense to use if let with an irrefutable pattern:

$ cargo run
   Compiling patterns v0.1.0 (file:///projects/patterns)
warning: irrefutable `if let` pattern
 --> src/main.rs:2:8
  |
2 |     if let x = 5 {
  |        ^^^^^^^^^
  |
  = note: `#[warn(irrefutable_let_patterns)]` on by default
  = note: this pattern will always match, so the `if let` is useless
  = help: consider replacing the `if let` with a `let`

warning: `patterns` (bin "patterns") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.39s
     Running `target/debug/patterns`
5
*/





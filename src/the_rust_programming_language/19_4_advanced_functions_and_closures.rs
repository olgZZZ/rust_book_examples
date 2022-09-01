// Advanced Functions and Closures
// https://doc.rust-lang.org/stable/book/ch19-05-advanced-functions-and-closures.html





// Function Pointers

/*
The syntax for specifying that a parameter is a function pointer is similar to that of closures, as shown in Listing 19-27, where we’ve defined a function add_one that adds one to its parameter. The function do_twice takes two parameters: a function pointer to any function that takes an i32 parameter and returns an i32, and one i32 value. The do_twice function calls the function f twice, passing it the arg value, then adds the two function call results together. The main function calls do_twice with the arguments add_one and 5.

Listing 19-27: Using the fn type to accept a function pointer as an argument
Filename: src/main.rs
*/
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);
}





/*
As an example of where you could use either a closure defined inline or a named function, let’s look at a use of the map method provided by the Iterator trait in the standard library. To use the map function to turn a vector of numbers into a vector of strings, we could use a closure, like this:
*/
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> =
        list_of_numbers.iter().map(|i| i.to_string()).collect();




/*
Or we could name a function as the argument to map instead of the closure, like this:
*/
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> =
        list_of_numbers.iter().map(ToString::to_string).collect();





/*
Recall from the “Enum values” section of Chapter 6 that the name of each enum variant that we define also becomes an initializer function. We can use these initializer functions as function pointers that implement the closure traits, which means we can specify the initializer functions as arguments for methods that take closures, like so:
*/
    enum Status {
        Value(u32),
        Stop,
    }

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();






// Returning Closures

/*
The following code tries to return a closure directly, but it won’t compile:
*/
fn returns_closure() -> dyn Fn(i32) -> i32 {
    |x| x + 1
}


/*
The compiler error is as follows:

$ cargo build
   Compiling functions-example v0.1.0 (file:///projects/functions-example)
error[E0746]: return type cannot have an unboxed trait object
 --> src/lib.rs:1:25
  |
1 | fn returns_closure() -> dyn Fn(i32) -> i32 {
  |                         ^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
  |
  = note: for information on `impl Trait`, see <https://doc.rust-lang.org/book/ch10-02-traits.html#returning-types-that-implement-traits>
help: use `impl Fn(i32) -> i32` as the return type, as all return paths are of type `[closure@src/lib.rs:2:5: 2:14]`, which implements `Fn(i32) -> i32`
  |
1 | fn returns_closure() -> impl Fn(i32) -> i32 {
  |                         ~~~~~~~~~~~~~~~~~~~

For more information about this error, try `rustc --explain E0746`.
error: could not compile `functions-example` due to previous error
*/






/*
The error references the Sized trait again! Rust doesn’t know how much space it will need to store the closure. We saw a solution to this problem earlier. We can use a trait object:
*/
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}








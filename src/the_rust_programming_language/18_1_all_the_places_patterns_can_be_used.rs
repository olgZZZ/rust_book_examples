// All the Places Patterns Can Be Used
// https://doc.rust-lang.org/stable/book/ch18-01-all-the-places-for-patterns.html






// match Arms

/*
As discussed in Chapter 6, we use patterns in the arms of match expressions. Formally, match expressions are defined as the keyword match, a value to match on, and one or more match arms that consist of a pattern and an expression to run if the value matches that arm’s pattern, like this:

match VALUE {
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
}

*/


/*
For example, here's the match expression from Listing 6-5 that matches on an Option<i32> value in the variable x:
*/
match x {
    None => None,
    Some(i) => Some(i + 1),
}






// Conditional if let Expressions

/*
The code in Listing 18-1 determines what color to make your background based on a series of checks for several conditions. For this example, we’ve created variables with hardcoded values that a real program might receive from user input.

Listing 18-1: Mixing if let, else if, else if let, and else
Filename: src/main.rs
*/
fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}






// while let Conditional Loops

/*
Similar in construction to if let, the while let conditional loop allows a while loop to run for as long as a pattern continues to match. In Listing 18-2 we code a while let loop that uses a vector as a stack and prints the values in the vector in the opposite order in which they were pushed.

Listing 18-2: Using a while let loop to print values for as long as stack.pop() returns Some
*/
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }







// for Loops

/*
In a for loop, the value that directly follows the keyword for is a pattern. For example, in for x in y the x is the pattern. Listing 18-3 demonstrates how to use a pattern in a for loop to destructure, or break apart, a tuple as part of the for loop.

Listing 18-3: Using a pattern in a for loop to destructure a tuple
*/
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }


/*
The code in Listing 18-3 will print the following:

$ cargo run
   Compiling patterns v0.1.0 (file:///projects/patterns)
    Finished dev [unoptimized + debuginfo] target(s) in 0.52s
     Running `target/debug/patterns`
a is at index 0
b is at index 1
c is at index 2
*/









// let Statements

/*
Prior to this chapter, we had only explicitly discussed using patterns with match and if let, but in fact, we’ve used patterns in other places as well, including in let statements. For example, consider this straightforward variable assignment with let:
*/

let x = 5;




/*
Every time you've used a let statement like this you've been using patterns, although you might not have realized it! More formally, a let statement looks like this:

let PATTERN = EXPRESSION;
*/





/*
To see the pattern matching aspect of let more clearly, consider Listing 18-4, which uses a pattern with let to destructure a tuple.

Listing 18-4: Using a pattern to destructure a tuple and create three variables at once
*/
    let (x, y, z) = (1, 2, 3);







/*
If the number of elements in the pattern doesn’t match the number of elements in the tuple, the overall type won’t match and we’ll get a compiler error. For example, Listing 18-5 shows an attempt to destructure a tuple with three elements into two variables, which won’t work.

Listing 18-5: Incorrectly constructing a pattern whose variables don’t match the number of elements in the tuple
*/
    let (x, y) = (1, 2, 3);




/*
Attempting to compile this code results in this type error:

$ cargo run
   Compiling patterns v0.1.0 (file:///projects/patterns)
error[E0308]: mismatched types
 --> src/main.rs:2:9
  |
2 |     let (x, y) = (1, 2, 3);
  |         ^^^^^^   --------- this expression has type `({integer}, {integer}, {integer})`
  |         |
  |         expected a tuple with 3 elements, found one with 2 elements
  |
  = note: expected tuple `({integer}, {integer}, {integer})`
             found tuple `(_, _)`

For more information about this error, try `rustc --explain E0308`.
error: could not compile `patterns` due to previous error
*/







// Function Parameters

/*
Function parameters can also be patterns. The code in Listing 18-6, which declares a function named foo that takes one parameter named x of type i32, should by now look familiar.

Listing 18-6: A function signature uses patterns in the parameters
*/
fn foo(x: i32) {
    // code goes here
}





/*
The x part is a pattern! As we did with let, we could match a tuple in a function’s arguments to the pattern. Listing 18-7 splits the values in a tuple as we pass it to a function.

Listing 18-7: A function with parameters that destructure a tuple
Filename: src/main.rs
*/
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

fn main() {
    let point = (3, 5);
    print_coordinates(&point);
}



/*
This code prints Current location: (3, 5). The values &(3, 5) match the pattern &(x, y), so x is the value 3 and y is the value 5.
*/




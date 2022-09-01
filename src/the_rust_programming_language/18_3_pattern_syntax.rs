// Pattern Syntax
// https://doc.rust-lang.org/stable/book/ch18-03-pattern-syntax.html





// Matching Literals

/*
As you saw in Chapter 6, you can match patterns against literals directly. The following code gives some examples:
*/
fn main() {
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}






// Matching Named Variables

/*
Named variables are irrefutable patterns that match any value, and we’ve used them many times in the book. However, there is a complication when you use named variables in match expressions. Because match starts a new scope, variables declared as part of a pattern inside the match expression will shadow those with the same name outside the match construct, as is the case with all variables. In Listing 18-11, we declare a variable named x with the value Some(5) and a variable y with the value 10. We then create a match expression on the value x. Look at the patterns in the match arms and println! at the end, and try to figure out what the code will print before running this code or reading further.

Listing 18-11: A match expression with an arm that introduces a shadowed variable y
Filename: src/main.rs
*/
fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);
}




// Multiple Patterns

/*
In match expressions, you can match multiple patterns using the | syntax, which is the pattern or operator. For example, in the following code we match the value of x against the match arms, the first of which has an or option, meaning if the value of x matches either of the values in that arm, that arm’s code will run:
*/
fn main() {
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}






// Matching Ranges of Values with ..=

/*
The ..= syntax allows us to match to an inclusive range of values. In the following code, when a pattern matches any of the values within the given range, that arm will execute:
*/
fn main() {
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }
}



/*
The compiler checks that the range isn’t empty at compile time, and because the only types for which Rust can tell if a range is empty or not are char and numeric values, ranges are only allowed with numeric or char values.

Here is an example using ranges of char values:
*/
    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }






// Destructuring Structs

/*
Listing 18-12 shows a Point struct with two fields, x and y, that we can break apart using a pattern with a let statement.

Listing 18-12: Destructuring a struct’s fields into separate variables
Filename: src/main.rs
*/
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
}






/*
This code creates the variables a and b that match the values of the x and y fields of the p struct. This example shows that the names of the variables in the pattern don’t have to match the field names of the struct. However, it’s common to match the variable names to the field names to make it easier to remember which variables came from which fields. Because of this common usage, and because writing let Point { x: x, y: y } = p; contains a lot of duplication, Rust has a shorthand for patterns that match struct fields: you only need to list the name of the struct field, and the variables created from the pattern will have the same names. Listing 18-13 behaves in the same way as the code in Listing 18-12, but the variables created in the let pattern are x and y instead of a and b.

Listing 18-13: Destructuring struct fields using struct field shorthand
Filename: src/main.rs
*/
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);
}






/*
In Listing 18-14, we have a match expression that separates Point values into three cases: points that lie directly on the x axis (which is true when y = 0), on the y axis (x = 0), or neither.

Listing 18-14: Destructuring and matching literal values in one pattern
Filename: src/main.rs
*/
fn main() {
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}






// Destructuring Enums

/*
We've destructured enums in this book (for example, Listing 6-5 in Chapter 6), but haven’t yet explicitly discussed that the pattern to destructure an enum corresponds to the way the data stored within the enum is defined. As an example, in Listing 18-15 we use the Message enum from Listing 6-2 and write a match with patterns that will destructure each inner value.

Listing 18-15: Destructuring enum variants that hold different kinds of values
Filename: src/main.rs
*/
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!(
                "Move in the x direction {} and in the y direction {}",
                x, y
            );
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => println!(
            "Change the color to red {}, green {}, and blue {}",
            r, g, b
        ),
    }
}







// Destructuring Nested Structs and Enums

/*
So far, our examples have all been matching structs or enums one level deep, but matching can work on nested items too! For example, we can refactor the code in Listing 18-15 to support RGB and HSV colors in the ChangeColor message, as shown in Listing 18-16.

Listing 18-16: Matching on nested enums
*/
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn main() {
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => println!(
            "Change the color to red {}, green {}, and blue {}",
            r, g, b
        ),
        Message::ChangeColor(Color::Hsv(h, s, v)) => println!(
            "Change the color to hue {}, saturation {}, and value {}",
            h, s, v
        ),
        _ => (),
    }
}






// Destructuring Structs and Tuples

/*
We can mix, match, and nest destructuring patterns in even more complex ways. The following example shows a complicated destructure where we nest structs and tuples inside a tuple and destructure all the primitive values out:
*/
fn main() {
    struct Point {
        x: i32,
        y: i32,
    }

    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
}







// Ignoring an Entire Value with _

/*
We’ve used the underscore as a wildcard pattern that will match any value but not bind to the value. This is especially useful as the last arm in a match expression, but we can also use it in any pattern, including function parameters, as shown in Listing 18-17.

Listing 18-17: Using _ in a function signature
Filename: src/main.rs
*/
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}

fn main() {
    foo(3, 4);
}

/*
This code will completely ignore the value 3 passed as the first argument, and will print This code only uses the y parameter: 4.
*/







// Ignoring Parts of a Value with a Nested _

/*
We can also use _ inside another pattern to ignore just part of a value, for example, when we want to test for only part of a value but have no use for the other parts in the corresponding code we want to run. Listing 18-18 shows code responsible for managing a setting’s value. The business requirements are that the user should not be allowed to overwrite an existing customization of a setting but can unset the setting and give it a value if it is currently unset.

Listing 18-18: Using an underscore within patterns that match Some variants when we don’t need to use the value inside the Some
*/
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);








/*
In all other cases (if either setting_value or new_setting_value are None) expressed by the _ pattern in the second arm, we want to allow new_setting_value to become setting_value.

We can also use underscores in multiple places within one pattern to ignore particular values. Listing 18-19 shows an example of ignoring the second and fourth values in a tuple of five items.

Listing 18-19: Ignoring multiple parts of a tuple
*/
fn main() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}")
        }
    }
}

/*
This code will print Some numbers: 2, 8, 32, and the values 4 and 16 will be ignored.
*/






// Ignoring an Unused Variable by Starting Its Name with _

/*
If you create a variable but don’t use it anywhere, Rust will usually issue a warning because an unused variable could be a bug. However, sometimes it’s useful to be able to create a variable you won’t use yet, such as when you’re prototyping or just starting a project. In this situation, you can tell Rust not to warn you about the unused variable by starting the name of the variable with an underscore. In Listing 18-20, we create two unused variables, but when we compile this code, we should only get a warning about one of them.

Listing 18-20: Starting a variable name with an underscore to avoid getting unused variable warnings
Filename: src/main.rs
*/
fn main() {
    let _x = 5;
    let y = 10;
}






/*
Note that there is a subtle difference between using only _ and using a name that starts with an underscore. The syntax _x still binds the value to the variable, whereas _ doesn’t bind at all. To show a case where this distinction matters, Listing 18-21 will provide us with an error.

Listing 18-21: An unused variable starting with an underscore still binds the value, which might take ownership of the value
*/
 [This code does not compile!] 
    let s = Some(String::from("Hello!"));

    if let Some(_s) = s {
        println!("found a string");
    }

    println!("{:?}", s);







/*
We’ll receive an error because the s value will still be moved into _s, which prevents us from using s again. However, using the underscore by itself doesn’t ever bind to the value. Listing 18-22 will compile without any errors because s doesn’t get moved into _.

Listing 18-22: Using an underscore does not bind the value
*/
fn main() {
    let s = Some(String::from("Hello!"));

    if let Some(_) = s {
        println!("found a string");
    }

    println!("{:?}", s);
}







// Ignoring Remaining Parts of a Value with ..

/*
With values that have many parts, we can use the .. syntax to use specific parts and ignore the rest, avoiding the need to list underscores for each ignored value. The .. pattern ignores any parts of a value that we haven’t explicitly matched in the rest of the pattern. In Listing 18-23, we have a Point struct that holds a coordinate in three-dimensional space. In the match expression, we want to operate only on the x coordinate and ignore the values in the y and z fields.

Listing 18-23: Ignoring all fields of a Point except for x by using ..
*/
fn main() {
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point { x: 0, y: 0, z: 0 };

    match origin {
        Point { x, .. } => println!("x is {}", x),
    }
}








/*
We list the x value and then just include the .. pattern. This is quicker than having to list y: _ and z: _, particularly when we’re working with structs that have lots of fields in situations where only one or two fields are relevant.

The syntax .. will expand to as many values as it needs to be. Listing 18-24 shows how to use .. with a tuple.

Listing 18-24: Matching only the first and last values in a tuple and ignoring all other values
Filename: src/main.rs
*/
fn main() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }
}




/*
However, using .. must be unambiguous. If it is unclear which values are intended for matching and which should be ignored, Rust will give us an error. Listing 18-25 shows an example of using .. ambiguously, so it will not compile.

Listing 18-25: An attempt to use .. in an ambiguous way
Filename: src/main.rs
*/
 [This code does not compile!] 
fn main() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (.., second, ..) => {
            println!("Some numbers: {}", second)
        },
    }
}



/*
When we compile this example, we get this error:

$ cargo run
   Compiling patterns v0.1.0 (file:///projects/patterns)
error: `..` can only be used once per tuple pattern
 --> src/main.rs:5:22
  |
5 |         (.., second, ..) => {
  |          --          ^^ can only be used once per tuple pattern
  |          |
  |          previously used here

error: could not compile `patterns` due to previous error
*/






// Extra Conditionals with Match Guards

/*
The condition can use variables created in the pattern. Listing 18-26 shows a match where the first arm has the pattern Some(x) and also has a match guard of if x % 2 == 0 (which will be true if the number is even).

Listing 18-26: Adding a match guard to a pattern
*/
fn main() {
    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }
}







/*
In Listing 18-11, we mentioned that we could use match guards to solve our pattern-shadowing problem. Recall that we created a new variable inside the pattern in the match expression instead of using the variable outside the match. That new variable meant we couldn’t test against the value of the outer variable. Listing 18-27 shows how we can use a match guard to fix this problem.

Listing 18-27: Using a match guard to test for equality with an outer variable
Filename: src/main.rs
*/
fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);
}







/*
You can also use the or operator | in a match guard to specify multiple patterns; the match guard condition will apply to all the patterns. Listing 18-28 shows the precedence when combining a pattern that uses | with a match guard. The important part of this example is that the if y match guard applies to 4, 5, and 6, even though it might look like if y only applies to 6.

Listing 18-28: Combining multiple patterns with a match guard
*/
fn main() {
    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
}



/*
 The code moves on to the second arm, which does match, and this program prints no. The reason is that the if condition applies to the whole pattern 4 | 5 | 6, not only to the last value 6. In other words, the precedence of a match guard in relation to a pattern behaves like this:

(4 | 5 | 6) if y => ...

rather than this:

4 | 5 | (6 if y) => ...
*/







// @ Bindings

/*
The at operator @ lets us create a variable that holds a value at the same time as we’re testing that value for a pattern match. In Listing 18-29, we want to test that a Message::Hello id field is within the range 3..=7. We also want to bind the value to the variable id_variable so we can use it in the code associated with the arm. We could name this variable id, the same as the field, but for this example we’ll use a different name.

Listing 18-29: Using @ to bind to a value in a pattern while also testing it
*/
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id: {}", id),
    }








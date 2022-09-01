/// Data types
/// https://doc.rust-lang.org/stable/book/ch03-02-data-types.html



// Here’s an example that shows floating-point numbers in action:
fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
}



// The following code shows how you’d use each numeric operation in a let statement:
fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;
}




// The Boolean type in Rust is specified using bool. For example:
fn main() {
    let t = true;

    let f: bool = false; // with explicit type annotation
}



// Rust’s char type is the language’s most primitive alphabetic type. 
// Here’s some examples of declaring char values:
fn main() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
}


// The Tuple Type
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}



// To get the individual values out of a tuple, we can use pattern 
// matching to destructure a tuple value, like this:
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");
}




// We can also access a tuple element directly by using a period (.) 
// followed by the index of the value we want to access. For example:
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}




// The Array Type
fn main() {
    let a = [1, 2, 3, 4, 5];
}



let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

let a: [i32; 5] = [1, 2, 3, 4, 5];


let a = [3; 5];




// Accessing Array Elements
fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}



// Invalid Array Element Access
use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}

















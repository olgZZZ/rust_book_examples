// Conversion
// https://doc.rust-lang.org/rust-by-example/conversion.html



/*
    Primitive types can be converted to each other through casting.

    Rust addresses conversion between custom types (i.e., struct and enum) by the use of traits. The generic conversions will use the From and Into traits. However there are more specific ones for the more common cases, in particular when converting to and from Strings.
*/





// From and Into
// https://doc.rust-lang.org/rust-by-example/conversion/from_into.html



/*
    The From and Into traits are inherently linked, and this is actually part of its implementation. If you are able to convert type A from type B, then it should be easy to believe that we should be able to convert type B to type A.
*/



/*
    From

    The From trait allows for a type to define how to create itself from another type, hence providing a very simple mechanism for converting between several types. There are numerous implementations of this trait within the standard library for conversion of primitive and common types.

    For example we can easily convert a str into a String
*/

#![allow(unused)]
fn main() {
let my_str = "hello";
let my_string = String::from(my_str);
}





// We can do similar for defining a conversion for our own type.

use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    let num = Number::from(30);
    println!("My number is {:?}", num);
}







/*
    Into

    The Into trait is simply the reciprocal of the From trait. That is, if you have implemented the From trait for your type, Into will call it when necessary.

    Using the Into trait will typically require specification of the type to convert into as the compiler is unable to determine this most of the time. However this is a small trade-off considering we get the functionality for free.
*/


use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    let int = 5;
    // Try removing the type declaration
    let num: Number = int.into();
    println!("My number is {:?}", num);
}



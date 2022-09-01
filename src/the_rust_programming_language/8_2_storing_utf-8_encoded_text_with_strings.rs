// Storing UTF-8 Encoded Text with Strings
// https://doc.rust-lang.org/stable/book/ch08-02-strings.html





// Creating a New String
fn main() {
    let mut s = String::new();
}



// Listing 8-12: Using the to_string method to create a String from a string literal
fn main() {
    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();
}



// Listing 8-13: Using the String::from function to create a String from a string literal
fn main() {
    let s = String::from("initial contents");
}



// Listing 8-14: Storing greetings in different languages in strings
fn main() {
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
}




// Listing 8-15: Appending a string slice to a String using the push_str method
fn main() {
    let mut s = String::from("foo");
    s.push_str("bar");
}



// Listing 8-16: Using a string slice after appending its contents to a String
fn main() {
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);
}




// Listing 8-17: Adding one character to a String value using push
fn main() {
    let mut s = String::from("lo");
    s.push('l');
}



// Concatenation with the + Operator or the format! Macro

// Listing 8-18: Using the + operator to combine two String values into a new String value
Listing 8-18: Using the + operator to combine two String values into a new String value




// If we need to concatenate multiple strings, the behavior of the + operator gets unwieldy:
fn main() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
}



// At this point, s will be tic-tac-toe. With all of the + and " characters, it’s difficult 
// to see what’s going on. For more complicated string combining, we can instead use the format! macro:
fn main() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
}





// Indexing into Strings

// Listing 8-19: Attempting to use indexing syntax with a String
fn main() {
    let s1 = String::from("hello");
    let h = s1[0];
}

// This code will result in the following error:

/*
$ cargo run
   Compiling collections v0.1.0 (file:///projects/collections)
error[E0277]: the type `String` cannot be indexed by `{integer}`
 --> src/main.rs:3:13
  |
3 |     let h = s1[0];
  |             ^^^^^ `String` cannot be indexed by `{integer}`
  |
  = help: the trait `Index<{integer}>` is not implemented for `String`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `collections` due to previous error

*/



// Internal Representation

let hello = String::from("Hola");

let hello = String::from("Здравствуйте");

let hello = "Здравствуйте";
let answer = &hello[0];



// Bytes and Scalar Values and Grapheme Clusters! Oh My!

[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,
224, 165, 135]

['न', 'म', 'स', '्', 'त', 'े']

["न", "म", "स्", "ते"]




// Slicing Strings

// Rather than indexing using [] with a single number, you can use [] with a range to 
// create a string slice containing particular bytes:
#![allow(unused)]
fn main() {
let hello = "Здравствуйте";

let s = &hello[0..4];
}

/*
$ cargo run
   Compiling collections v0.1.0 (file:///projects/collections)
    Finished dev [unoptimized + debuginfo] target(s) in 0.43s
     Running `target/debug/collections`
thread 'main' panicked at 'byte index 1 is not a char boundary; it is inside 'З' (bytes 0..2) of `Здравствуйте`', library/core/src/str/mod.rs:127:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

*/




// Methods for Iterating Over Strings

// Calling chars on “Зд” separates out and returns two values of type char, and you 
// can iterate over the result to access each element:
#![allow(unused)]
fn main() {
for c in "Зд".chars() {
    println!("{}", c);
}
}

/*
    З
    д
*/


// Alternatively, the bytes method returns each raw byte, which might be appropriate for your domain:
#![allow(unused)]
fn main() {
for b in "Зд".bytes() {
    println!("{}", b);
}
}

/*
    208
    151
    208
    180
*/





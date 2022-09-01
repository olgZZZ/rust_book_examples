// Recoverable Errors with Result
// https://doc.rust-lang.org/stable/book/ch09-02-recoverable-errors-with-result.html



// Recall from “Handling Potential Failure with the Result Type” in Chapter 2 that 
// the Result enum is defined as having two variants, Ok and Err, as follows:
#![allow(unused)]
fn main() {
enum Result<T, E> {
    Ok(T),
    Err(E),
}
}


// Listing 9-3: Opening a file
use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");
}


// Listing 9-4: Using a match expression to handle the Result variants that might be returned
use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}

/*
$ cargo run
   Compiling error-handling v0.1.0 (file:///projects/error-handling)
    Finished dev [unoptimized + debuginfo] target(s) in 0.73s
     Running `target/debug/error-handling`
thread 'main' panicked at 'Problem opening the file: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/main.rs:8:23
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

*/





// Matching on Different Errors
// Listing 9-5: Handling different kinds of errors in different ways
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}




// Alternatives to Using match with Result<T, E>
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}




// Shortcuts for Panic on Error: unwrap and expect
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt").unwrap();
}


/*
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os {
code: 2, kind: NotFound, message: "No such file or directory" }',
src/main.rs:4:49

*/



use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}


/*
thread 'main' panicked at 'hello.txt should be included in this project: Os {
code: 2, kind: NotFound, message: "No such file or directory" }',
src/main.rs:5:10

*/




// Propagating Errors

// Listing 9-6: A function that returns errors to the calling code using match

#![allow(unused)]
fn main() {
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
}





// A Shortcut for Propagating Errors: the ? Operator

// Listing 9-7: A function that returns errors to the calling code using the ? operator
#![allow(unused)]
fn main() {
use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
}





// Listing 9-8: Chaining method calls after the ? operator
#![allow(unused)]
fn main() {
use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}
}



// Listing 9-9: Using fs::read_to_string instead of opening and then reading the file
#![allow(unused)]
fn main() {
use std::fs;
use std::io;

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
}




// Where The ? Operator Can Be Used

// Listing 9-10: Attempting to use the ? in the main function that returns () won’t compile
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt")?;
}


/*
$ cargo run
   Compiling error-handling v0.1.0 (file:///projects/error-handling)
error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
 --> src/main.rs:4:48
  |
3 | / fn main() {
4 | |     let greeting_file = File::open("hello.txt")?;
  | |                                                ^ cannot use the `?` operator in a function that returns `()`
5 | | }
  | |_- this function should return `Result` or `Option` to accept `?`
  |
  = help: the trait `FromResidual<Result<Infallible, std::io::Error>>` is not implemented for `()`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `error-handling` due to previous error

*/




// Listing 9-11: Using the ? operator on an Option<T> value
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

fn main() {
    assert_eq!(
        last_char_of_first_line("Hello, world\nHow are you today?"),
        Some('d')
    );

    assert_eq!(last_char_of_first_line(""), None);
    assert_eq!(last_char_of_first_line("\nhi"), None);
}





// Listing 9-12: Changing main to return Result<(), E> allows the use of the ? operator on Result values
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}








// Program arguments
// https://doc.rust-lang.org/rust-by-example/std_misc/arg.html


/*
    Standard Library

    The command line arguments can be accessed using std::env::args, which returns an iterator that yields a String for each argument:
*/

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // The first argument is the path that was used to call the program.
    println!("My path is {}.", args[0]);

    // The rest of the arguments are the passed command line parameters.
    // Call the program like this:
    //   $ ./args arg1 arg2
    println!("I got {:?} arguments: {:?}.", args.len() - 1, &args[1..]);
}


/*
    $ ./args 1 2 3
    My path is ./args.
    I got 3 arguments: ["1", "2", "3"].
*/




/*
    Crates

    Alternatively, there are numerous crates that can provide extra functionality when creating command-line applications. The Rust Cookbook exhibits best practices on how to use one of the more popular command line argument crates, clap.
*/








// Argument parsing
// https://doc.rust-lang.org/rust-by-example/std_misc/arg/matching.html

// Matching can be used to parse simple arguments:

use std::env;

fn increase(number: i32) {
    println!("{}", number + 1);
}

fn decrease(number: i32) {
    println!("{}", number - 1);
}

fn help() {
    println!("usage:
match_args <string>
    Check whether given string is the answer.
match_args {{increase|decrease}} <integer>
    Increase or decrease given integer by one.");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        // no arguments passed
        1 => {
            println!("My name is 'match_args'. Try passing some arguments!");
        },
        // one argument passed
        2 => {
            match args[1].parse() {
                Ok(42) => println!("This is the answer!"),
                _ => println!("This is not the answer."),
            }
        },
        // one command and one argument passed
        3 => {
            let cmd = &args[1];
            let num = &args[2];
            // parse the number
            let number: i32 = match num.parse() {
                Ok(n) => {
                    n
                },
                Err(_) => {
                    eprintln!("error: second argument not an integer");
                    help();
                    return;
                },
            };
            // parse the command
            match &cmd[..] {
                "increase" => increase(number),
                "decrease" => decrease(number),
                _ => {
                    eprintln!("error: invalid command");
                    help();
                },
            }
        },
        // all the other cases
        _ => {
            // show a help message
            help();
        }
    }
}






/*
    $ ./match_args Rust
    This is not the answer.
    $ ./match_args 42
    This is the answer!
    $ ./match_args do something
    error: second argument not an integer
    usage:
    match_args <string>
        Check whether given string is the answer.
    match_args {increase|decrease} <integer>
        Increase or decrease given integer by one.
    $ ./match_args do 42
    error: invalid command
    usage:
    match_args <string>
        Check whether given string is the answer.
    match_args {increase|decrease} <integer>
        Increase or decrease given integer by one.
    $ ./match_args increase 42
    43
*/



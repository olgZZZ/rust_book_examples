// Writing Error Messages to Standard Error Instead of Standard Output
// https://doc.rust-lang.org/stable/book/ch12-06-writing-to-stderr-instead-of-stdout.html




// Checking Where Errors Are Written

/*
To demonstrate this behavior, we’ll run the program with > and the file path, output.txt, that we want to redirect the standard output stream to. We won’t pass any arguments, which should cause an error:

$ cargo run > output.txt
*/

/*
The > syntax tells the shell to write the contents of standard output to output.txt instead of the screen. We didn’t see the error message we were expecting printed to the screen, so that means it must have ended up in the file. This is what output.txt contains:

Problem parsing arguments: not enough arguments
*/





// Printing Errors to Standard Error

/*
We’ll use the code in Listing 12-24 to change how error messages are printed. Because of the refactoring we did earlier in this chapter, all the code that prints error messages is in one function, main. The standard library provides the eprintln! macro that prints to the standard error stream, so let’s change the two places we were calling println! to print errors to use eprintln! instead.

Filename: src/main.rs
*/
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");

        process::exit(1);
    }
}



/*
Let’s now run the program again in the same way, without any arguments and redirecting standard output with >:

$ cargo run > output.txt
Problem parsing arguments: not enough arguments
*/

/*
Let’s run the program again with arguments that don’t cause an error but still redirect standard output to a file, like so:

$ cargo run -- to poem.txt > output.txt
*/

/*
We won’t see any output to the terminal, and output.txt will contain our results:

Filename: output.txt

Are you nobody, too?
How dreary to be somebody!
*/




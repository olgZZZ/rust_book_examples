// Accepting Command Line Arguments
// https://doc.rust-lang.org/stable/book/ch12-01-accepting-command-line-arguments.html




/*
$ cargo new minigrep
     Created binary (application) `minigrep` project
$ cd minigrep
*/


/*
$ cargo run -- searchstring example-filename.txt
*/






// Reading the Argument Values

/*
The code in Listing 12-1 allows your minigrep program to read any command line arguments passed to it and then collect the values into a vector.

Filename: src/main.rs
*/
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);
}




/*
Finally, we print the vector using the debug macro. Let’s try running the code first with no arguments and then with two arguments:

$ cargo run
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.61s
     Running `target/debug/minigrep`
[src/main.rs:5] args = [
    "target/debug/minigrep",
]


$ cargo run -- needle haystack
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 1.57s
     Running `target/debug/minigrep needle haystack`
[src/main.rs:5] args = [
    "target/debug/minigrep",
    "needle",
    "haystack",
]

*/






// Saving the Argument Values in Variables

/*
The program is currently able to access the values specified as command line arguments. Now we need to save the values of the two arguments in variables so we can use the values throughout the rest of the program. We do that in Listing 12-2.

Filename: src/main.rs
*/
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", file_path);
}




/*
We temporarily print the values of these variables to prove that the code is working as we intend. Let’s run this program again with the arguments test and sample.txt:

$ cargo run -- test sample.txt
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.0s
     Running `target/debug/minigrep test sample.txt`
Searching for test
In file sample.txt

*/






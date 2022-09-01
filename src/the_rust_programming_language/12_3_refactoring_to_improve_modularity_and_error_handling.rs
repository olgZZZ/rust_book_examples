// Refactoring to Improve Modularity and Error Handling
// https://doc.rust-lang.org/stable/book/ch12-03-improving-error-handling-and-modularity.html




// Extracting the Argument Parser

/*
We’ll extract the functionality for parsing arguments into a function that main will call to prepare for moving the command line parsing logic to src/lib.rs. Listing 12-5 shows the new start of main that calls a new function parse_config, which we’ll define in src/main.rs for the moment.

Filename: src/main.rs
*/
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (query, file_path) = parse_config(&args);

    // --snip--

    println!("Searching for {}", query);
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let file_path = &args[2];

    (query, file_path)
}






// Grouping Configuration Values

/*
Listing 12-6 shows the improvements to the parse_config function.

Filename: src/main.rs
*/
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");

    // --snip--

    println!("With text:\n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();

    Config { query, file_path }
}








// Creating a Constructor for Config

/*
So now that the purpose of the parse_config function is to create a Config instance, we can change parse_config from a plain function to a function named new that is associated with the Config struct. Making this change will make the code more idiomatic. We can create instances of types in the standard library, such as String, by calling String::new. Similarly, by changing parse_config into a new function associated with Config, we’ll be able to create instances of Config by calling Config::new. Listing 12-7 shows the changes we need to make.

Filename: src/main.rs
*/
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");

    // --snip--
}

// --snip--

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let file_path = args[2].clone();

        Config { query, file_path }
    }
}







// Fixing the Error Handling

/*
$ cargo run
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.0s
     Running `target/debug/minigrep`
thread 'main' panicked at 'index out of bounds: the len is 1 but the index is 1', src/main.rs:27:21
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

*/






// Improving the Error Message

/*
In Listing 12-8, we add a check in the new function that will verify that the slice is long enough before accessing index 1 and 2. If the slice isn’t long enough, the program panics and displays a better error message.

Filename: src/main.rs
*/
    // --snip--
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("not enough arguments");
        }
        // --snip--




/*
With these extra few lines of code in new, let’s run the program without any arguments again to see what the error looks like now:

$ cargo run
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.0s
     Running `target/debug/minigrep`
thread 'main' panicked at 'not enough arguments', src/main.rs:26:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

*/







// Returning a Result Instead of Calling panic!

/*
Listing 12-9 shows the changes we need to make to the return value of the function we’re now calling Config::build and the body of the function needed to return a Result. Note that this won’t compile until we update main as well, which we’ll do in the next listing.

Filename: src/main.rs
*/
impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}





// Calling Config::build and Handling Errors

/*
To handle the error case and print a user-friendly message, we need to update main to handle the Result being returned by Config::build, as shown in Listing 12-10. We’ll also take the responsibility of exiting the command line tool with a nonzero error code away from panic! and instead implement it by hand. A nonzero exit status is a convention to signal to the process that called our program that the program exited with an error state.

Filename: src/main.rs
*/
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // --snip--




/*
This is similar to the panic!-based handling we used in Listing 12-8, but we no longer get all the extra output. Let’s try it:

$ cargo run
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.48s
     Running `target/debug/minigrep`
Problem parsing arguments: not enough arguments

*/





// Extracting Logic from main


/*
Listing 12-11 shows the extracted run function. For now, we’re just making the small, incremental improvement of extracting the function. We’re still defining the function in src/main.rs.

Filename: src/main.rs
*/
fn main() {
    // --snip--

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    run(config);
}

fn run(config: Config) {
    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

// --snip--




// Returning Errors from the run Function

/*
With the remaining program logic separated into the run function, we can improve the error handling, as we did with Config::build in Listing 12-9. Instead of allowing the program to panic by calling expect, the run function will return a Result<T, E> when something goes wrong. This will let us further consolidate the logic around handling errors into main in a user-friendly way. Listing 12-12 shows the changes we need to make to the signature and body of run.

Filename: src/main.rs
*/
use std::error::Error;

// --snip--

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{contents}");

    Ok(())
}


/*
When you run this code, it will compile but will display a warning:

$ cargo run the poem.txt
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
warning: unused `Result` that must be used
  --> src/main.rs:19:5
   |
19 |     run(config);
   |     ^^^^^^^^^^^^
   |
   = note: `#[warn(unused_must_use)]` on by default
   = note: this `Result` may be an `Err` variant, which should be handled

warning: `minigrep` (bin "minigrep") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.71s
     Running `target/debug/minigrep the poem.txt`
Searching for the
In file poem.txt
With text:
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!


*/




// Handling Errors Returned from run in main

/*
We’ll check for errors and handle them using a technique similar to one we used with Config::build in Listing 12-10, but with a slight difference:

Filename: src/main.rs
*/
fn main() {
    // --snip--

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = run(config) {
        println!("Application error: {e}");

        process::exit(1);
    }
}






// Splitting Code into a Library Crate

/*
The contents of src/lib.rs should have the signatures shown in Listing 12-13 (we’ve omitted the bodies of the functions for brevity). Note that this won’t compile until we modify src/main.rs in Listing 12-14.

Filename: src/lib.rs
*/
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        // --snip--
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // --snip--
}





/*
We’ve made liberal use of the pub keyword: on Config, on its fields and its new method, and on the run function. We now have a library crate that has a public API we can test!

Now we need to bring the code we moved to src/lib.rs into the scope of the binary crate in src/main.rs, as shown in Listing 12-14.

Filename: src/main.rs
*/
use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // --snip--
    if let Err(e) = minigrep::run(config) {
        // --snip--
    }
}








// Working with Environment Variables
// https://doc.rust-lang.org/stable/book/ch12-05-working-with-environment-variables.html




// Writing a Failing Test for the Case-Insensitive search Function

/*
We first add a new search_case_insensitive function that will be called when the environment variable has a value. We’ll continue to follow the TDD process, so the first step is again to write a failing test. We’ll add a new test for the new search_case_insensitive function and rename our old test from one_result to case_sensitive to clarify the differences between the two tests, as shown in Listing 12-20.

Filename: src/lib.rs
*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}





// Implementing the search_case_insensitive Function

/*
The search_case_insensitive function, shown in Listing 12-21, will be almost the same as the search function. The only difference is that we’ll lowercase the query and each line so whatever the case of the input arguments, they’ll be the same case when we check whether the line contains the query.

Filename: src/lib.rs
*/
pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}



/*
Let’s see if this implementation passes the tests:

$ cargo test
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished test [unoptimized + debuginfo] target(s) in 1.33s
     Running unittests src/lib.rs (target/debug/deps/minigrep-9cd200e5fac0fc94)

running 2 tests
test tests::case_insensitive ... ok
test tests::case_sensitive ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/minigrep-9cd200e5fac0fc94)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests minigrep

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


*/



/*
Great! They passed. Now, let’s call the new search_case_insensitive function from the run function. First, we’ll add a configuration option to the Config struct to switch between case-sensitive and case-insensitive search. Adding this field will cause compiler errors because we aren’t initializing this field anywhere yet:

Filename: src/lib.rs
*/
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}



/*
We added the ignore_case field that holds a Boolean. Next, we need the run function to check the ignore_case field’s value and use that to decide whether to call the search function or the search_case_insensitive function, as shown in Listing 12-22. This still won’t compile yet.

Filename: src/lib.rs
*/
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}




/*
Finally, we need to check for the environment variable. The functions for working with environment variables are in the env module in the standard library, so we bring that module into scope at the top of src/lib.rs. Then we’ll use the var function from the env module to check to see if any value has been set for an environment variable named IGNORE_CASE, as shown in Listing 12-23.

Filename: src/lib.rs
*/
use std::env;
// --snip--

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}



/*
Let’s give it a try! First, we’ll run our program without the environment variable set and with the query to, which should match any line that contains the word “to” in all lowercase:

$ cargo run -- to poem.txt
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.0s
     Running `target/debug/minigrep to poem.txt`
Are you nobody, too?
How dreary to be somebody!

*/


/*
Looks like that still works! Now, let’s run the program with IGNORE_CASE set to 1 but with the same query to.

$ IGNORE_CASE=1 cargo run -- to poem.txt
*/

/*
If you’re using PowerShell, you will need to set the environment variable and run the program as separate commands:

PS> $Env:IGNORE_CASE=1; cargo run -- to poem.txt
*/

/*
This will make IGNORE_CASE persist for the remainder of your shell session. It can be unset with the Remove-Item cmdlet:

PS> Remove-Item Env:IGNORE_CASE
*/

/*
We should get lines that contain “to” that might have uppercase letters:

Are you nobody, too?
How dreary to be somebody!
To tell your name the livelong day
To an admiring bog!
*/





// Syntax
// https://doc.rust-lang.org/rust-by-example/macros/syntax.html


/*
    In following subsections, we will show how to define macros in Rust. There are three basic ideas:

        Patterns and Designators
        Overloading
        Repetition
*/






// Designators
// https://doc.rust-lang.org/rust-by-example/macros/designators.html

/*
    The arguments of a macro are prefixed by a dollar sign $ and type annotated with a designator:
*/

macro_rules! create_function {
    // This macro takes an argument of designator `ident` and
    // creates a function named `$func_name`.
    // The `ident` designator is used for variable/function names.
    ($func_name:ident) => {
        fn $func_name() {
            // The `stringify!` macro converts an `ident` into a string.
            println!("You called {:?}()",
                     stringify!($func_name));
        }
    };
}

// Create functions named `foo` and `bar` with the above macro.
create_function!(foo);
create_function!(bar);

macro_rules! print_result {
    // This macro takes an expression of type `expr` and prints
    // it as a string along with its result.
    // The `expr` designator is used for expressions.
    ($expression:expr) => {
        // `stringify!` will convert the expression *as it is* into a string.
        println!("{:?} = {:?}",
                 stringify!($expression),
                 $expression);
    };
}

fn main() {
    foo();
    bar();

    print_result!(1u32 + 1);

    // Recall that blocks are expressions too!
    print_result!({
        let x = 1u32;

        x * x + 2 * x - 1
    });
}


/*
    These are some of the available designators:

        block
        expr is used for expressions
        ident is used for variable/function names
        item
        literal is used for literal constants
        pat (pattern)
        path
        stmt (statement)
        tt (token tree)
        ty (type)
        vis (visibility qualifier)

    For a complete list, see the Rust Reference.
*/











// Overload
// https://doc.rust-lang.org/rust-by-example/macros/overload.html


/*
    acros can be overloaded to accept different combinations of arguments. In that regard, macro_rules! can work similarly to a match block:
*/

// `test!` will compare `$left` and `$right`
// in different ways depending on how you invoke it:
macro_rules! test {
    // Arguments don't need to be separated by a comma.
    // Any template can be used!
    ($left:expr; and $right:expr) => {
        println!("{:?} and {:?} is {:?}",
                 stringify!($left),
                 stringify!($right),
                 $left && $right)
    };
    // ^ each arm must end with a semicolon.
    ($left:expr; or $right:expr) => {
        println!("{:?} or {:?} is {:?}",
                 stringify!($left),
                 stringify!($right),
                 $left || $right)
    };
}

fn main() {
    test!(1i32 + 1 == 2i32; and 2i32 * 2 == 4i32);
    test!(true; or false);
}








// Repeat
// https://doc.rust-lang.org/rust-by-example/macros/repeat.html

/*
    Macros can use + in the argument list to indicate that an argument may repeat at least once, or *, to indicate that the argument may repeat zero or more times.

    In the following example, surrounding the matcher with $(...),+ will match one or more expression, separated by commas. Also note that the semicolon is optional on the last case.
*/

// `find_min!` will calculate the minimum of any number of arguments.
macro_rules! find_min {
    // Base case:
    ($x:expr) => ($x);
    // `$x` followed by at least one `$y,`
    ($x:expr, $($y:expr),+) => (
        // Call `find_min!` on the tail `$y`
        std::cmp::min($x, find_min!($($y),+))
    )
}

fn main() {
    println!("{}", find_min!(1));
    println!("{}", find_min!(1 + 2, 2));
    println!("{}", find_min!(5, 2 * 3, 4));
}



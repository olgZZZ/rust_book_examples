// Attributes
// https://doc.rust-lang.org/rust-by-example/attribute.html




/*
    An attribute is metadata applied to some module, crate or item. This metadata can be used to/for:

        conditional compilation of code
        set crate name, version and type (binary or library)
        disable lints (warnings)
        enable compiler features (macros, glob imports, etc.)
        link to a foreign library
        mark functions as unit tests
        mark functions that will be part of a benchmark

    When attributes apply to a whole crate, their syntax is #![crate_attribute], and when they apply to a module or item, the syntax is #[item_attribute] (notice the missing bang !).

    Attributes can take arguments with different syntaxes:

        #[attribute = "value"]
        #[attribute(key = "value")]
        #[attribute(value)]

    Attributes can have multiple values and can be separated over multiple lines, too:
*/

#[attribute(value, value2)]


#[attribute(value, value2, value3,
            value4, value5)]











// dead_code
// https://doc.rust-lang.org/rust-by-example/attribute/unused.html


// The compiler provides a dead_code lint that will warn about unused functions. 
// An attribute can be used to disable the lint.

fn used_function() {}

// `#[allow(dead_code)]` is an attribute that disables the `dead_code` lint
#[allow(dead_code)]
fn unused_function() {}

fn noisy_unused_function() {}
// FIXME ^ Add an attribute to suppress the warning

fn main() {
    used_function();
}


// Note that in real programs, you should eliminate dead code. In these examples we'll 
// allow dead code in some places because of the interactive nature of the examples.



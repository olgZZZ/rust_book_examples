// Conventions
// https://doc.rust-lang.org/rust-by-example/cargo/conventions.html

/*
    In the previous chapter, we saw the following directory hierarchy:

    foo
    ├── Cargo.toml
    └── src
        └── main.rs
*/




/*
    Suppose that we wanted to have two binaries in the same project, though. What then?

    It turns out that cargo supports this. The default binary name is main, as we saw before, but you can add additional binaries by placing them in a bin/ directory:

    foo
    ├── Cargo.toml
    └── src
        ├── main.rs
        └── bin
            └── my_other_bin.rs
*/




/*
    To tell cargo to compile or run this binary as opposed to the default or other binaries, we just pass cargo the --bin my_other_bin flag, where my_other_bin is the name of the binary we want to work with.

    In addition to extra binaries, cargo supports more features such as benchmarks, tests, and examples.

    In the next chapter, we will look more closely at tests.
*/




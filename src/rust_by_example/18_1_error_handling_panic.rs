// Error handling
// https://doc.rust-lang.org/rust-by-example/error.html

/*
    Error handling is the process of handling the possibility of failure. For example, failing to read a file and then continuing to use that bad input would clearly be problematic. Noticing and explicitly managing those errors saves the rest of the program from various pitfalls.

    There are various ways to deal with errors in Rust, which are described in the following subchapters. They all have more or less subtle differences and different use cases. As a rule of thumb:

    An explicit panic is mainly useful for tests and dealing with unrecoverable errors. For prototyping it can be useful, for example when dealing with functions that haven't been implemented yet, but in those cases the more descriptive unimplemented is better. In tests panic is a reasonable way to explicitly fail.

    The Option type is for when a value is optional or when the lack of a value is not an error condition. For example the parent of a directory - / and C: don't have one. When dealing with Options, unwrap is fine for prototyping and cases where it's absolutely certain that there is guaranteed to be a value. However expect is more useful since it lets you specify an error message in case something goes wrong anyway.

    When there is a chance that things do go wrong and the caller has to deal with the problem, use Result. You can unwrap and expect them as well (please don't do that unless it's a test or quick prototype).

    For a more rigorous discussion of error handling, refer to the error handling section in the official book.
*/






// panic
// https://doc.rust-lang.org/rust-by-example/error/panic.html


/*
    The simplest error handling mechanism we will see is panic. It prints an error message, starts unwinding the stack, and usually exits the program. Here, we explicitly call panic on our error condition:
*/

fn drink(beverage: &str) {
    // You shouldn't drink too much sugary beverages.
    if beverage == "lemonade" { panic!("AAAaaaaa!!!!"); }

    println!("Some refreshing {} is all I need.", beverage);
}

fn main() {
    drink("water");
    drink("lemonade");
}

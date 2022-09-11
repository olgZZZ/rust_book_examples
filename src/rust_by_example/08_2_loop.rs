// loop
// https://doc.rust-lang.org/rust-by-example/flow_control/loop.html


/*
    Rust provides a loop keyword to indicate an infinite loop.

    The break statement can be used to exit a loop at anytime, whereas the continue statement can be used to skip the rest of the iteration and start a new one.
*/



fn main() {
    let mut count = 0u32;

    println!("Let's count until infinity!");

    // Infinite loop
    loop {
        count += 1;

        if count == 3 {
            println!("three");

            // Skip the rest of this iteration
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");

            // Exit this loop
            break;
        }
    }
}







// Nesting and labels
// https://doc.rust-lang.org/rust-by-example/flow_control/loop/nested.html


/*
    It's possible to break or continue outer loops when dealing with nested loops. In these cases, the loops must be annotated with some 'label, and the label must be passed to the break/continue statement.
*/


#![allow(unreachable_code)]

fn main() {
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            // This would break only the inner loop
            //break;

            // This breaks the outer loop
            break 'outer;
        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop");
}






// Returning from loops
// https://doc.rust-lang.org/rust-by-example/flow_control/loop/return.html


/*
    One of the uses of a loop is to retry an operation until it succeeds. If the operation returns a value though, you might need to pass it to the rest of the code: put it after the break, and it will be returned by the loop expression.
*/


fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);
}


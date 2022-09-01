// Using Box<T> to Point to Data on the Heap
// https://doc.rust-lang.org/stable/book/ch15-01-box.html





// Using a Box<T> to Store Data on the Heap

/*
Before we discuss the heap storage use case for Box<T>, we’ll cover the syntax and how to interact with values stored within a Box<T>.

Listing 15-1 shows how to use a box to store an i32 value on the heap:

Filename: src/main.rs

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
}
*/





// More Information About the Cons List

/*
For example, here's a pseudocode representation of a cons list containing the list 1, 2, 3 with each pair in parentheses:

(1, (2, (3, Nil)))
*/


/*
Listing 15-2 contains an enum definition for a cons list. Note that this code won’t compile yet because the List type doesn’t have a known size, which we’ll demonstrate.

Filename: src/main.rs

 [This code does not compile!] 
enum List {
    Cons(i32, List),
    Nil,
}
*/




/*
Using the List type to store the list 1, 2, 3 would look like the code in Listing 15-3:

Filename: src/main.rs

 [This code does not compile!] 
use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Cons(2, Cons(3, Nil)));
}
*/



/*
The first Cons value holds 1 and another List value. This List value is another Cons value that holds 2 and another List value. This List value is one more Cons value that holds 3 and a List value, which is finally Nil, the non-recursive variant that signals the end of the list.

If we try to compile the code in Listing 15-3, we get the error shown in Listing 15-4:

$ cargo run
   Compiling cons-list v0.1.0 (file:///projects/cons-list)
error[E0072]: recursive type `List` has infinite size
 --> src/main.rs:1:1
  |
1 | enum List {
  | ^^^^^^^^^ recursive type has infinite size
2 |     Cons(i32, List),
  |               ---- recursive without indirection
  |
help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to make `List` representable
  |
2 |     Cons(i32, Box<List>),
  |               ++++    +

error[E0391]: cycle detected when computing drop-check constraints for `List`
 --> src/main.rs:1:1
  |
1 | enum List {
  | ^^^^^^^^^
  |
  = note: ...which immediately requires computing drop-check constraints for `List` again
  = note: cycle used when computing dropck types for `Canonical { max_universe: U0, variables: [], value: ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: UserFacing, constness: NotConst }, value: List } }`

Some errors have detailed explanations: E0072, E0391.
For more information about an error, try `rustc --explain E0072`.
error: could not compile `cons-list` due to 2 previous errors

*/







// Computing the Size of a Non-Recursive Type

/*
Recall the Message enum we defined in Listing 6-2 when we discussed enum definitions in Chapter 6:

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
*/







// Using Box<T> to Get a Recursive Type with a Known Size

/*
Because Rust can’t figure out how much space to allocate for recursively defined types, the compiler gives an error with this helpful suggestion:

help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to make `List` representable
  |
2 |     Cons(i32, Box<List>),
  |  
*/





/*
We can change the definition of the List enum in Listing 15-2 and the usage of the List in Listing 15-3 to the code in Listing 15-5, which will compile:

Filename: src/main.rs

enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
*/



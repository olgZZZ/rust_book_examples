// Treating Smart Pointers Like Regular References with the Deref Trait
// https://doc.rust-lang.org/stable/book/ch15-02-deref.html





// Following the Pointer to the Value

/*
A regular reference is a type of pointer, and one way to think of a pointer is as an arrow to a value stored somewhere else. In Listing 15-6, we create a reference to an i32 value and then use the dereference operator to follow the reference to the value:

Filename: src/main.rs

fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
*/



/*
If we tried to write assert_eq!(5, y); instead, we would get this compilation error:

$ cargo run
   Compiling deref-example v0.1.0 (file:///projects/deref-example)
error[E0277]: can't compare `{integer}` with `&{integer}`
 --> src/main.rs:6:5
  |
6 |     assert_eq!(5, y);
  |     ^^^^^^^^^^^^^^^^ no implementation for `{integer} == &{integer}`
  |
  = help: the trait `PartialEq<&{integer}>` is not implemented for `{integer}`
  = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)

For more information about this error, try `rustc --explain E0277`.
error: could not compile `deref-example` due to previous error
*/







// Using Box<T> Like a Reference

/*
We can rewrite the code in Listing 15-6 to use a Box<T> instead of a reference; the dereference operator used on the Box<T> in Listing 15-7 functions in the same way as the dereference operator used on the reference in Listing 15-6:

Filename: src/main.rs

fn main() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
*/







// Defining Our Own Smart Pointer

/*
Let’s build a smart pointer similar to the Box<T> type provided by the standard library to experience how smart pointers behave differently from references by default. Then we’ll look at how to add the ability to use the dereference operator.

The Box<T> type is ultimately defined as a tuple struct with one element, so Listing 15-8 defines a MyBox<T> type in the same way. We’ll also define a new function to match the new function defined on Box<T>.

Filename: src/main.rs

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
*/


/*
Let’s try adding the main function in Listing 15-7 to Listing 15-8 and changing it to use the MyBox<T> type we’ve defined instead of Box<T>. The code in Listing 15-9 won’t compile because Rust doesn’t know how to dereference MyBox.

Filename: src/main.rs

 [This code does not compile!] 
fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
*/


/*
Here’s the resulting compilation error:

$ cargo run
   Compiling deref-example v0.1.0 (file:///projects/deref-example)
error[E0614]: type `MyBox<{integer}>` cannot be dereferenced
  --> src/main.rs:14:19
   |
14 |     assert_eq!(5, *y);
   |                   ^^

For more information about this error, try `rustc --explain E0614`.
error: could not compile `deref-example` due to previous error
*/







// Treating a Type Like a Reference by Implementing the Deref Trait

/*
As discussed in the “Implementing a Trait on a Type” section of Chapter 10, to implement a trait, we need to provide implementations for the trait’s required methods. The Deref trait, provided by the standard library, requires us to implement one method named deref that borrows self and returns a reference to the inner data. Listing 15-10 contains an implementation of Deref to add to the definition of MyBox:

Filename: src/main.rs

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
*/





/*
Without the Deref trait, the compiler can only dereference & references. The deref method gives the compiler the ability to take a value of any type that implements Deref and call the deref method to get a & reference that it knows how to dereference.

When we entered *y in Listing 15-9, behind the scenes Rust actually ran this code:

*(y.deref())
*/






// Implicit Deref Coercions with Functions and Methods

/*
To see deref coercion in action, let’s use the MyBox<T> type we defined in Listing 15-8 as well as the implementation of Deref that we added in Listing 15-10. Listing 15-11 shows the definition of a function that has a string slice parameter:

Filename: src/main.rs

fn hello(name: &str) {
    println!("Hello, {name}!");
}
*/



/*
We can call the hello function with a string slice as an argument, such as hello("Rust"); for example. Deref coercion makes it possible to call hello with a reference to a value of type MyBox<String>, as shown in Listing 15-12:

Filename: src/main.rs

fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}
*/



/*
If Rust didn’t implement deref coercion, we would have to write the code in Listing 15-13 instead of the code in Listing 15-12 to call hello with a value of type &MyBox<String>.

Filename: src/main.rs

fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&(*m)[..]);
}
*/




// How Deref Coercion Interacts with Mutability

/*
Rust does deref coercion when it finds types and trait implementations in three cases:

    From &T to &U when T: Deref<Target=U>
    From &mut T to &mut U when T: DerefMut<Target=U>
    From &mut T to &U when T: Deref<Target=U>

*/




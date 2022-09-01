// Unsafe Rust
// https://doc.rust-lang.org/stable/book/ch19-01-unsafe-rust.html




// Unsafe Superpowers

/*
To switch to unsafe Rust, use the unsafe keyword and then start a new block that holds the unsafe code. You can take five actions in unsafe Rust that you can’t in safe Rust, which we call unsafe superpowers. Those superpowers include the ability to:

    Dereference a raw pointer
    Call an unsafe function or method
    Access or modify a mutable static variable
    Implement an unsafe trait
    Access fields of unions

*/






// Dereferencing a Raw Pointer

/*
Different from references and smart pointers, raw pointers:

    Are allowed to ignore the borrowing rules by having both immutable and mutable pointers or multiple mutable pointers to the same location
    Aren’t guaranteed to point to valid memory
    Are allowed to be null
    Don’t implement any automatic cleanup

*/


/*
By opting out of having Rust enforce these guarantees, you can give up guaranteed safety in exchange for greater performance or the ability to interface with another language or hardware where Rust’s guarantees don’t apply.

Listing 19-1 shows how to create an immutable and a mutable raw pointer from references.

Listing 19-1: Creating raw pointers from references
*/
fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
}






/*
To demonstrate this, next we’ll create a raw pointer whose validity we can’t be so certain of. Listing 19-2 shows how to create a raw pointer to an arbitrary location in memory. Trying to use arbitrary memory is undefined: there might be data at that address or there might not, the compiler might optimize the code so there is no memory access, or the program might error with a segmentation fault. Usually, there is no good reason to write code like this, but it is possible.

Listing 19-2: Creating a raw pointer to an arbitrary memory address
*/
fn main() {
    let address = 0x012345usize;
    let r = address as *const i32;
}




/*
Recall that we can create raw pointers in safe code, but we can’t dereference raw pointers and read the data being pointed to. In Listing 19-3, we use the dereference operator * on a raw pointer that requires an unsafe block.

Listing 19-3: Dereferencing raw pointers within an unsafe block
*/
fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}







// Calling an Unsafe Function or Method

/*
Here is an unsafe function named dangerous that doesn’t do anything in its body:
*/
fn main() {
    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }
}



/*
We must call the dangerous function within a separate unsafe block. If we try to call dangerous without the unsafe block, we’ll get an error:

$ cargo run
   Compiling unsafe-example v0.1.0 (file:///projects/unsafe-example)
error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
 --> src/main.rs:4:5
  |
4 |     dangerous();
  |     ^^^^^^^^^^^ call to unsafe function
  |
  = note: consult the function's documentation for information on how to avoid undefined behavior

For more information about this error, try `rustc --explain E0133`.
error: could not compile `unsafe-example` due to previous error
*/






// Creating a Safe Abstraction over Unsafe Code

/*
Just because a function contains unsafe code doesn’t mean we need to mark the entire function as unsafe. In fact, wrapping unsafe code in a safe function is a common abstraction. As an example, let’s study the split_at_mut function from the standard library, which requires some unsafe code. We’ll explore how we might implement it. This safe method is defined on mutable slices: it takes one slice and makes it two by splitting the slice at the index given as an argument. Listing 19-4 shows how to use split_at_mut.

Listing 19-4: Using the safe split_at_mut function
*/
fn main() {
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}




/*
We can’t implement this function using only safe Rust. An attempt might look something like Listing 19-5, which won’t compile. For simplicity, we’ll implement split_at_mut as a function rather than a method and only for slices of i32 values rather than for a generic type T.

Listing 19-5: An attempted implementation of split_at_mut using only safe Rust
*/
 [This code does not compile!] 
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();

    assert!(mid <= len);

    (&mut values[..mid], &mut values[mid..])
}


/*
When we try to compile the code in Listing 19-5, we’ll get an error.

$ cargo run
   Compiling unsafe-example v0.1.0 (file:///projects/unsafe-example)
error[E0499]: cannot borrow `*values` as mutable more than once at a time
 --> src/main.rs:6:31
  |
1 | fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
  |                         - let's call the lifetime of this reference `'1`
...
6 |     (&mut values[..mid], &mut values[mid..])
  |     --------------------------^^^^^^--------
  |     |     |                   |
  |     |     |                   second mutable borrow occurs here
  |     |     first mutable borrow occurs here
  |     returning this value requires that `*values` is borrowed for `'1`

For more information about this error, try `rustc --explain E0499`.
error: could not compile `unsafe-example` due to previous error
*/







/*
Listing 19-6 shows how to use an unsafe block, a raw pointer, and some calls to unsafe functions to make the implementation of split_at_mut work.

Listing 19-6: Using unsafe code in the implementation of the split_at_mut function
*/
use std::slice;

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn main() {
    let mut vector = vec![1, 2, 3, 4, 5, 6];
    let (left, right) = split_at_mut(&mut vector, 3);
}









/*
In contrast, the use of slice::from_raw_parts_mut in Listing 19-7 would likely crash when the slice is used. This code takes an arbitrary memory location and creates a slice 10,000 items long.

Listing 19-7: Creating a slice from an arbitrary memory location
*/
fn main() {
    use std::slice;

    let address = 0x01234usize;
    let r = address as *mut i32;

    let values: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };
}






// Using extern Functions to Call External Code

/*
Listing 19-8 demonstrates how to set up an integration with the abs function from the C standard library. Functions declared within extern blocks are always unsafe to call from Rust code. The reason is that other languages don’t enforce Rust’s rules and guarantees, and Rust can’t check them, so responsibility falls on the programmer to ensure safety.

Listing 19-8: Declaring and calling an extern function defined in another language
Filename: src/main.rs
*/
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}






// Calling Rust Functions from Other Languages

/*
In the following example, we make the call_from_c function accessible from C code, after it’s compiled to a shared library and linked from C:

This usage of extern does not require unsafe.
*/
#![allow(unused)]
fn main() {
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}
}






// Accessing or Modifying a Mutable Static Variable

/*
In Rust, global variables are called static variables. Listing 19-9 shows an example declaration and use of a static variable with a string slice as a value.

Listing 19-9: Defining and using an immutable static variable
Filename: src/main.rs
*/
static HELLO_WORLD: &str = "Hello, world!";

fn main() {
    println!("name is: {}", HELLO_WORLD);
}





/*
A subtle difference between constants and immutable static variables is that values in a static variable have a fixed address in memory. Using the value will always access the same data. Constants, on the other hand, are allowed to duplicate their data whenever they’re used. Another difference is that static variables can be mutable. Accessing and modifying mutable static variables is unsafe. Listing 19-10 shows how to declare, access, and modify a mutable static variable named COUNTER.

Listing 19-10: Reading from or writing to a mutable static variable is unsafe
Filename: src/main.rs
*/
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}








// Implementing an Unsafe Trait

/*
We can use unsafe to implement an unsafe trait. A trait is unsafe when at least one of its methods has some invariant that the compiler can’t verify. We declare that a trait is unsafe by adding the unsafe keyword before trait and marking the implementation of the trait as unsafe too, as shown in Listing 19-11.

Listing 19-11: Defining and implementing an unsafe trait
*/
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}

fn main() {}




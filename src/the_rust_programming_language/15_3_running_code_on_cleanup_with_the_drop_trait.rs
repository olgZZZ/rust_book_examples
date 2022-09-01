// Running Code on Cleanup with the Drop Trait
// https://doc.rust-lang.org/stable/book/ch15-03-drop.html




/*
Listing 15-14 shows a CustomSmartPointer struct whose only custom functionality is that it will print Dropping CustomSmartPointer! when the instance goes out of scope, to show when Rust runs the drop function.

Filename: src/main.rs

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
}
*/





/*
In main, we create two instances of CustomSmartPointer and then print CustomSmartPointers created. At the end of main, our instances of CustomSmartPointer will go out of scope, and Rust will call the code we put in the drop method, printing our final message. Note that we didn’t need to call the drop method explicitly.

When we run this program, we’ll see the following output:

$ cargo run
   Compiling drop-example v0.1.0 (file:///projects/drop-example)
    Finished dev [unoptimized + debuginfo] target(s) in 0.60s
     Running `target/debug/drop-example`
CustomSmartPointers created.
Dropping CustomSmartPointer with data `other stuff`!
Dropping CustomSmartPointer with data `my stuff`!
*/








// Dropping a Value Early with std::mem::drop

/*
If we try to call the Drop trait’s drop method manually by modifying the main function from Listing 15-14, as shown in Listing 15-15, we’ll get a compiler error:

Filename: src/main.rs

 [This code does not compile!] 
fn main() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    c.drop();
    println!("CustomSmartPointer dropped before the end of main.");
}
*/


/*
When we try to compile this code, we’ll get this error:

$ cargo run
   Compiling drop-example v0.1.0 (file:///projects/drop-example)
error[E0040]: explicit use of destructor method
  --> src/main.rs:16:7
   |
16 |     c.drop();
   |     --^^^^--
   |     | |
   |     | explicit destructor calls not allowed
   |     help: consider using `drop` function: `drop(c)`

For more information about this error, try `rustc --explain E0040`.
error: could not compile `drop-example` due to previous error
*/




/*
We can’t disable the automatic insertion of drop when a value goes out of scope, and we can’t call the drop method explicitly. So, if we need to force a value to be cleaned up early, we use the std::mem::drop function.

The std::mem::drop function is different from the drop method in the Drop trait. We call it by passing as an argument the value we want to force drop. The function is in the prelude, so we can modify main in Listing 15-15 to call the drop function, as shown in Listing 15-16:

Filename: src/main.rs

fn main() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}
*/


/*
Running this code will print the following:

$ cargo run
   Compiling drop-example v0.1.0 (file:///projects/drop-example)
    Finished dev [unoptimized + debuginfo] target(s) in 0.73s
     Running `target/debug/drop-example`
CustomSmartPointer created.
Dropping CustomSmartPointer with data `some data`!
CustomSmartPointer dropped before the end of main.
*/




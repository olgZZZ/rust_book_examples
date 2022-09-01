// Closures: Anonymous Functions that Capture Their Environment
// https://doc.rust-lang.org/stable/book/ch13-01-closures.html






// Capturing the Environment with Closures

// Listing 13-1: Shirt company giveaway situation
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}



/*
Running this code prints:

$ cargo run
   Compiling shirt-company v0.1.0 (file:///projects/shirt-company)
    Finished dev [unoptimized + debuginfo] target(s) in 0.27s
     Running `target/debug/shirt-company`
The user with preference Some(Red) gets Red
The user with preference None gets Blue

*/





// Closure Type Inference and Annotation

// Listing 13-2: Adding optional type annotations of the parameter and return value types in the closure
// Filename: src/main.rs
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };





// This illustrates how closure syntax is similar to function syntax except for the use of 
// pipes and the amount of syntax that is optional:
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;







// Listing 13-3: Attempting to call a closure whose types are inferred with two different types
// Filename: src/main.rs
    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));
    let n = example_closure(5);



/*
The compiler gives us this error:

$ cargo run
   Compiling closure-example v0.1.0 (file:///projects/closure-example)
error[E0308]: mismatched types
 --> src/main.rs:5:29
  |
5 |     let n = example_closure(5);
  |                             ^- help: try using a conversion method: `.to_string()`
  |                             |
  |                             expected struct `String`, found integer

For more information about this error, try `rustc --explain E0308`.
error: could not compile `closure-example` due to previous error

*/






// Capturing References or Moving Ownership

// In Listing 13-4, we define a closure that captures an immutable reference to the vector 
// named list because it only needs an immutable reference to print the value:
// Filename: src/main.rs
fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);
}



/*
This code compiles, runs, and prints:

$ cargo run
   Compiling closure-example v0.1.0 (file:///projects/closure-example)
    Finished dev [unoptimized + debuginfo] target(s) in 0.43s
     Running `target/debug/closure-example`
Before defining closure: [1, 2, 3]
Before calling closure: [1, 2, 3]
From closure: [1, 2, 3]
After calling closure: [1, 2, 3]

*/







// Next, in Listing 13-5, we change the closure body so that it adds an element to 
// the list vector. The closure now captures a mutable reference:
// Filename: src/main.rs
fn main() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("After calling closure: {:?}", list);
}


/*
This code compiles, runs, and prints:

$ cargo run
   Compiling closure-example v0.1.0 (file:///projects/closure-example)
    Finished dev [unoptimized + debuginfo] target(s) in 0.43s
     Running `target/debug/closure-example`
Before defining closure: [1, 2, 3]
After calling closure: [1, 2, 3, 7]

*/









// Moving Captured Values Out of Closures and the Fn Traits

// Let’s look at the definition of the unwrap_or_else method on Option<T> that we used in Listing 13-6:
impl<T> Option<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }
}



/*
This function is useful when you want to sort a slice by a particular attribute of each item. In Listing 13-x, we have a list of Rectangle instances and we use sort_by_key to order them by their width attribute from low to high:

Filename: src/main.rs
*/
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);
}




/*
This code prints:

$ cargo run
   Compiling rectangles v0.1.0 (file:///projects/rectangles)
    Finished dev [unoptimized + debuginfo] target(s) in 0.41s
     Running `target/debug/rectangles`
[
    Rectangle {
        width: 3,
        height: 5,
    },
    Rectangle {
        width: 7,
        height: 12,
    },
    Rectangle {
        width: 10,
        height: 1,
    },
]

*/







/*
In contrast, Listing 13-8 shows an example of a closure that implements just the FnOnce trait, because it moves a value out of the environment. The compiler won’t let us use this closure with sort_by_key:

Filename: src/main.rs
*/
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    let mut sort_operations = vec![];
    let value = String::from("by key called");

    list.sort_by_key(|r| {
        sort_operations.push(value);
        r.width
    });
    println!("{:#?}", list);
}






/*
When we try to compile this code, we get this error that value can’t be moved out of the closure because the closure must implement FnMut:

$ cargo run
   Compiling rectangles v0.1.0 (file:///projects/rectangles)
error[E0507]: cannot move out of `value`, a captured variable in an `FnMut` closure
  --> src/main.rs:27:30
   |
24 |       let value = String::from("by key called");
   |           ----- captured outer variable
25 | 
26 |       list.sort_by_key(|r| {
   |  ______________________-
27 | |         sort_operations.push(value);
   | |                              ^^^^^ move occurs because `value` has type `String`, which does not implement the `Copy` trait
28 | |         r.width
29 | |     });
   | |_____- captured by this `FnMut` closure

For more information about this error, try `rustc --explain E0507`.
error: could not compile `rectangles` due to previous error

*/







/*
The closure in Listing 13-x works with sort_by_key because it is only capturing a mutable reference to the num_sort_operations counter and can therefore be called more than once:

Filename: src/main.rs
*/
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{:#?}, sorted in {num_sort_operations} operations", list);
}




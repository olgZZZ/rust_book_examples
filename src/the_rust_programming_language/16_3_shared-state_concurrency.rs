// Shared-State Concurrency
// https://doc.rust-lang.org/stable/book/ch16-03-shared-state.html





// Using Mutexes to Allow Access to Data from One Thread at a Time

/*
Mutex is an abbreviation for mutual exclusion, as in, a mutex allows only one thread to access some data at any given time. To access the data in a mutex, a thread must first signal that it wants access by asking to acquire the mutex’s lock. The lock is a data structure that is part of the mutex that keeps track of who currently has exclusive access to the data. Therefore, the mutex is described as guarding the data it holds via the locking system.

Mutexes have a reputation for being difficult to use because you have to remember two rules:

    You must attempt to acquire the lock before using the data.
    When you’re done with the data that the mutex guards, you must unlock the data so other threads can acquire the lock.

*/










// The API of Mutex<T>

/*
As an example of how to use a mutex, let’s start by using a mutex in a single-threaded context, as shown in Listing 16-12:

Filename: src/main.rs
*/
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}








// Sharing a Mutex<T> Between Multiple Threads

/*
Now, let’s try to share a value between multiple threads using Mutex<T>. We’ll spin up 10 threads and have them each increment a counter value by 1, so the counter goes from 0 to 10. The next example in Listing 16-13 will have a compiler error, and we’ll use that error to learn more about using Mutex<T> and how Rust helps us use it correctly.

Filename: src/main.rs
*/
 [This code does not compile!] 
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Mutex::new(0);
    let mut handles = vec![];

    for _ in 0..10 {
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}







/*
In the main thread, we collect all the join handles. Then, as we did in Listing 16-2, we call join on each handle to make sure all the threads finish. At that point, the main thread will acquire the lock and print the result of this program.

We hinted that this example wouldn’t compile. Now let’s find out why!

$ cargo run
   Compiling shared-state v0.1.0 (file:///projects/shared-state)
error[E0382]: use of moved value: `counter`
  --> src/main.rs:9:36
   |
5  |     let counter = Mutex::new(0);
   |         ------- move occurs because `counter` has type `Mutex<i32>`, which does not implement the `Copy` trait
...
9  |         let handle = thread::spawn(move || {
   |                                    ^^^^^^^ value moved into closure here, in previous iteration of loop
10 |             let mut num = counter.lock().unwrap();
   |                           ------- use occurs due to use in closure

For more information about this error, try `rustc --explain E0382`.
error: could not compile `shared-state` due to previous error
*/








// Multiple Ownership with Multiple Threads

/*
In Chapter 15, we gave a value multiple owners by using the smart pointer Rc<T> to create a reference counted value. Let’s do the same here and see what happens. We’ll wrap the Mutex<T> in Rc<T> in Listing 16-14 and clone the Rc<T> before moving ownership to the thread.

Filename: src/main.rs
*/
 [This code does not compile!] 
use std::rc::Rc;
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Rc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Rc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}



/*
Once again, we compile and get... different errors! The compiler is teaching us a lot.

$ cargo run
   Compiling shared-state v0.1.0 (file:///projects/shared-state)
error[E0277]: `Rc<Mutex<i32>>` cannot be sent between threads safely
   --> src/main.rs:11:22
    |
11  |           let handle = thread::spawn(move || {
    |  ______________________^^^^^^^^^^^^^_-
    | |                      |
    | |                      `Rc<Mutex<i32>>` cannot be sent between threads safely
12  | |             let mut num = counter.lock().unwrap();
13  | |
14  | |             *num += 1;
15  | |         });
    | |_________- within this `[closure@src/main.rs:11:36: 15:10]`
    |
    = help: within `[closure@src/main.rs:11:36: 15:10]`, the trait `Send` is not implemented for `Rc<Mutex<i32>>`
    = note: required because it appears within the type `[closure@src/main.rs:11:36: 15:10]`
note: required by a bound in `spawn`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `shared-state` due to previous error
*/






// Atomic Reference Counting with Arc<T>

/*
Let’s return to our example: Arc<T> and Rc<T> have the same API, so we fix our program by changing the use line, the call to new, and the call to clone. The code in Listing 16-15 will finally compile and run:

Filename: src/main.rs
*/
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}


/*
This code will print the following:

Result: 10

We did it! We counted from 0 to 10, which may not seem very impressive, but it did teach us a lot about Mutex<T> and thread safety. You could also use this program’s structure to do more complicated operations than just incrementing a counter. Using this strategy, you can divide a calculation into independent parts, split those parts across threads, and then use a Mutex<T> to have each thread update the final result with its part.
*/




// Similarities Between RefCell<T>/Rc<T> and Mutex<T>/Arc<T>

/*
You might have noticed that counter is immutable but we could get a mutable reference to the value inside it; this means Mutex<T> provides interior mutability, as the Cell family does. In the same way we used RefCell<T> in Chapter 15 to allow us to mutate contents inside an Rc<T>, we use Mutex<T> to mutate contents inside an Arc<T>.

Another detail to note is that Rust can’t protect you from all kinds of logic errors when you use Mutex<T>. Recall in Chapter 15 that using Rc<T> came with the risk of creating reference cycles, where two Rc<T> values refer to each other, causing memory leaks. Similarly, Mutex<T> comes with the risk of creating deadlocks. These occur when an operation needs to lock two resources and two threads have each acquired one of the locks, causing them to wait for each other forever. If you’re interested in deadlocks, try creating a Rust program that has a deadlock; then research deadlock mitigation strategies for mutexes in any language and have a go at implementing them in Rust. The standard library API documentation for Mutex<T> and MutexGuard offers useful information.

We’ll round out this chapter by talking about the Send and Sync traits and how we can use them with custom types.
*/












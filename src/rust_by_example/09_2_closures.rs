// Closures
// https://doc.rust-lang.org/rust-by-example/fn/closures.html


/*
    Closures are functions that can capture the enclosing environment. For example, a closure that captures the x variable:

    |val| val + x

    The syntax and capabilities of closures make them very convenient for on the fly usage. Calling a closure is exactly like calling a function. However, both input and return types can be inferred and input variable names must be specified.

    Other characteristics of closures include:

        using || instead of () around input variables.
        optional body delimination ({}) for a single expression (mandatory otherwise).
        the ability to capture the outer environment variables.
*/


fn main() {
    // Increment via closures and functions.
    fn function(i: i32) -> i32 { i + 1 }

    // Closures are anonymous, here we are binding them to references
    // Annotation is identical to function annotation but is optional
    // as are the `{}` wrapping the body. These nameless functions
    // are assigned to appropriately named variables.
    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred  = |i     |          i + 1  ;

    let i = 1;
    // Call the function and closures.
    println!("function: {}", function(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(i));

    // A closure taking no arguments which returns an `i32`.
    // The return type is inferred.
    let one = || 1;
    println!("closure returning one: {}", one());

}






// Capturing
// https://doc.rust-lang.org/rust-by-example/fn/closures/capture.html



/*
    Closures are inherently flexible and will do what the functionality requires to make the closure work without annotation. This allows capturing to flexibly adapt to the use case, sometimes moving and sometimes borrowing. Closures can capture variables:

        by reference: &T
        by mutable reference: &mut T
        by value: T

    They preferentially capture variables by reference and only go lower when required.
*/

fn main() {
    use std::mem;
    
    let color = String::from("green");

    // A closure to print `color` which immediately borrows (`&`) `color` and
    // stores the borrow and closure in the `print` variable. It will remain
    // borrowed until `print` is used the last time. 
    //
    // `println!` only requires arguments by immutable reference so it doesn't
    // impose anything more restrictive.
    let print = || println!("`color`: {}", color);

    // Call the closure using the borrow.
    print();

    // `color` can be borrowed immutably again, because the closure only holds
    // an immutable reference to `color`. 
    let _reborrow = &color;
    print();

    // A move or reborrow is allowed after the final use of `print`
    let _color_moved = color;


    let mut count = 0;
    // A closure to increment `count` could take either `&mut count` or `count`
    // but `&mut count` is less restrictive so it takes that. Immediately
    // borrows `count`.
    //
    // A `mut` is required on `inc` because a `&mut` is stored inside. Thus,
    // calling the closure mutates the closure which requires a `mut`.
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    // Call the closure using a mutable borrow.
    inc();

    // The closure still mutably borrows `count` because it is called later.
    // An attempt to reborrow will lead to an error.
    // let _reborrow = &count; 
    // ^ TODO: try uncommenting this line.
    inc();

    // The closure no longer needs to borrow `&mut count`. Therefore, it is
    // possible to reborrow without an error
    let _count_reborrowed = &mut count; 

    
    // A non-copy type.
    let movable = Box::new(3);

    // `mem::drop` requires `T` so this must take by value. A copy type
    // would copy into the closure leaving the original untouched.
    // A non-copy must move and so `movable` immediately moves into
    // the closure.
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    // `consume` consumes the variable so this can only be called once.
    consume();
    // consume();
    // ^ TODO: Try uncommenting this line.
}





// Using move before vertical pipes forces closure to take ownership of captured variables:

fn main() {
    // `Vec` has non-copy semantics.
    let haystack = vec![1, 2, 3];

    let contains = move |needle| haystack.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&4));

    // println!("There're {} elements in vec", haystack.len());
    // ^ Uncommenting above line will result in compile-time error
    // because borrow checker doesn't allow re-using variable after it
    // has been moved.
    
    // Removing `move` from closure's signature will cause closure
    // to borrow _haystack_ variable immutably, hence _haystack_ is still
    // available and uncommenting above line will not cause an error.
}












// As input parameters
// https://doc.rust-lang.org/rust-by-example/fn/closures/input_parameters.html


/*
    While Rust chooses how to capture variables on the fly mostly without type annotation, this ambiguity is not allowed when writing functions. When taking a closure as an input parameter, the closure's complete type must be annotated using one of a few traits, and they're determined by what the closure does with captured value. In order of decreasing restriction, they are:

        Fn: the closure uses the captured value by reference (&T)
        FnMut: the closure uses the captured value by mutable reference (&mut T)
        FnOnce: the closure uses the captured value by value (T)

    On a variable-by-variable basis, the compiler will capture variables in the least restrictive manner possible.

    For instance, consider a parameter annotated as FnOnce. This specifies that the closure may capture by &T, &mut T, or T, but the compiler will ultimately choose based on how the captured variables are used in the closure.

    This is because if a move is possible, then any type of borrow should also be possible. Note that the reverse is not true. If the parameter is annotated as Fn, then capturing variables by &mut T or T are not allowed.

    In the following example, try swapping the usage of Fn, FnMut, and FnOnce to see what happens:
*/


// A function which takes a closure as an argument and calls it.
// <F> denotes that F is a "Generic type parameter"
fn apply<F>(f: F) where
    // The closure takes no input and returns nothing.
    F: FnOnce() {
    // ^ TODO: Try changing this to `Fn` or `FnMut`.

    f();
}

// A function which takes a closure and returns an `i32`.
fn apply_to_3<F>(f: F) -> i32 where
    // The closure takes an `i32` and returns an `i32`.
    F: Fn(i32) -> i32 {

    f(3)
}

fn main() {
    use std::mem;

    let greeting = "hello";
    // A non-copy type.
    // `to_owned` creates owned data from borrowed one
    let mut farewell = "goodbye".to_owned();

    // Capture 2 variables: `greeting` by reference and
    // `farewell` by value.
    let diary = || {
        // `greeting` is by reference: requires `Fn`.
        println!("I said {}.", greeting);

        // Mutation forces `farewell` to be captured by
        // mutable reference. Now requires `FnMut`.
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        // Manually calling drop forces `farewell` to
        // be captured by value. Now requires `FnOnce`.
        mem::drop(farewell);
    };

    // Call the function which applies the closure.
    apply(diary);

    // `double` satisfies `apply_to_3`'s trait bound
    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to_3(double));
}










// Type anonymity
// https://doc.rust-lang.org/rust-by-example/fn/closures/anonymity.html


/*
    Closures succinctly capture variables from enclosing scopes. Does this have any consequences? It surely does. Observe how using a closure as a function parameter requires generics, which is necessary because of how they are defined:
*/


#![allow(unused)]
fn main() {
// `F` must be generic.
fn apply<F>(f: F) where
    F: FnOnce() {
    f();
}
}



/*
    When a closure is defined, the compiler implicitly creates a new anonymous structure to store the captured variables inside, meanwhile implementing the functionality via one of the traits: Fn, FnMut, or FnOnce for this unknown type. This type is assigned to the variable which is stored until calling.

    Since this new type is of unknown type, any usage in a function will require generics. However, an unbounded type parameter <T> would still be ambiguous and not be allowed. Thus, bounding by one of the traits: Fn, FnMut, or FnOnce (which it implements) is sufficient to specify its type.
*/


// `F` must implement `Fn` for a closure which takes no
// inputs and returns nothing - exactly what is required
// for `print`.
fn apply<F>(f: F) where
    F: Fn() {
    f();
}

fn main() {
    let x = 7;

    // Capture `x` into an anonymous type and implement
    // `Fn` for it. Store it in `print`.
    let print = || println!("{}", x);

    apply(print);
}








// Input functions
// https://doc.rust-lang.org/rust-by-example/fn/closures/input_functions.html


/*
    Since closures may be used as arguments, you might wonder if the same can be said about functions. And indeed they can! If you declare a function that takes a closure as parameter, then any function that satisfies the trait bound of that closure can be passed as a parameter.
*/

// Define a function which takes a generic `F` argument
// bounded by `Fn`, and calls it
fn call_me<F: Fn()>(f: F) {
    f();
}

// Define a wrapper function satisfying the `Fn` bound
fn function() {
    println!("I'm a function!");
}

fn main() {
    // Define a closure satisfying the `Fn` bound
    let closure = || println!("I'm a closure!");

    call_me(closure);
    call_me(function);
}


/*
    As an additional note, the Fn, FnMut, and FnOnce traits dictate how a closure captures variables from the enclosing scope.
*/








// As output parameters
// https://doc.rust-lang.org/rust-by-example/fn/closures/output_parameters.html


/*
    Closures as input parameters are possible, so returning closures as output parameters should also be possible. However, anonymous closure types are, by definition, unknown, so we have to use impl Trait to return them.

    The valid traits for returning a closure are:

        Fn
        FnMut
        FnOnce

    Beyond this, the move keyword must be used, which signals that all captures occur by value. This is required because any captures by reference would be dropped as soon as the function exited, leaving invalid references in the closure.
*/

fn create_fn() -> impl Fn() {
    let text = "Fn".to_owned();

    move || println!("This is a: {}", text)
}

fn create_fnmut() -> impl FnMut() {
    let text = "FnMut".to_owned();

    move || println!("This is a: {}", text)
}

fn create_fnonce() -> impl FnOnce() {
    let text = "FnOnce".to_owned();

    move || println!("This is a: {}", text)
}

fn main() {
    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();
    let fn_once = create_fnonce();

    fn_plain();
    fn_mut();
    fn_once();
}








// Examples in std
// https://doc.rust-lang.org/rust-by-example/fn/closures/closure_examples.html

// This section contains a few examples of using closures from the std library.




// Iterator::any
// https://doc.rust-lang.org/rust-by-example/fn/closures/closure_examples/iter_any.html

/*
    Iterator::any is a function which when passed an iterator, will return true if any element satisfies the predicate. Otherwise false. Its signature:
*/


pub trait Iterator {
    // The type being iterated over.
    type Item;

    // `any` takes `&mut self` meaning the caller may be borrowed
    // and modified, but not consumed.
    fn any<F>(&mut self, f: F) -> bool where
        // `FnMut` meaning any captured variable may at most be
        // modified, not consumed. `Self::Item` states it takes
        // arguments to the closure by value.
        F: FnMut(Self::Item) -> bool {}
}




fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // `iter()` for vecs yields `&i32`. Destructure to `i32`.
    println!("2 in vec1: {}", vec1.iter()     .any(|&x| x == 2));
    // `into_iter()` for vecs yields `i32`. No destructuring required.
    println!("2 in vec2: {}", vec2.into_iter().any(| x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // `iter()` for arrays yields `&i32`.
    println!("2 in array1: {}", array1.iter()     .any(|&x| x == 2));
    // `into_iter()` for arrays yields `i32`.
    println!("2 in array2: {}", array2.into_iter().any(|x| x == 2));
}






// Searching through iterators
// https://doc.rust-lang.org/rust-by-example/fn/closures/closure_examples/iter_find.html

/*
    Iterator::find is a function which iterates over an iterator and searches for the first value which satisfies some condition. If none of the values satisfy the condition, it returns None. Its signature:
*/

pub trait Iterator {
    // The type being iterated over.
    type Item;

    // `find` takes `&mut self` meaning the caller may be borrowed
    // and modified, but not consumed.
    fn find<P>(&mut self, predicate: P) -> Option<Self::Item> where
        // `FnMut` meaning any captured variable may at most be
        // modified, not consumed. `&Self::Item` states it takes
        // arguments to the closure by reference.
        P: FnMut(&Self::Item) -> bool {}
}



fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // `iter()` for vecs yields `&i32`.
    let mut iter = vec1.iter();
    // `into_iter()` for vecs yields `i32`.
    let mut into_iter = vec2.into_iter();

    // `iter()` for vecs yields `&i32`, and we want to reference one of its
    // items, so we have to destructure `&&i32` to `i32`
    println!("Find 2 in vec1: {:?}", iter     .find(|&&x| x == 2));
    // `into_iter()` for vecs yields `i32`, and we want to reference one of
    // its items, so we have to destructure `&i32` to `i32`
    println!("Find 2 in vec2: {:?}", into_iter.find(| &x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // `iter()` for arrays yields `&i32`
    println!("Find 2 in array1: {:?}", array1.iter()     .find(|&&x| x == 2));
    // `into_iter()` for arrays yields `i32`
    println!("Find 2 in array2: {:?}", array2.into_iter().find(|&x| x == 2));
}




/*
    Iterator::find gives you a reference to the item. But if you want the index of the item, use Iterator::position.
*/

fn main() {
    let vec = vec![1, 9, 3, 3, 13, 2];

    // `iter()` for vecs yields `&i32` and `position()` does not take a reference, so
    // we have to destructure `&i32` to `i32`
    let index_of_first_even_number = vec.iter().position(|&x| x % 2 == 0);
    assert_eq!(index_of_first_even_number, Some(5));
    
    // `into_iter()` for vecs yields `i32` and `position()` does not take a reference, so
    // we do not have to destructure    
    let index_of_first_negative_number = vec.into_iter().position(|x| x < 0);
    assert_eq!(index_of_first_negative_number, None);
}




/*
    ### [See also:](https://doc.rust-lang.org/rust-by-example/fn/closures/closure_examples/iter_find.html#see-also)

    [`std::iter::Iterator::find`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.find)

    [`std::iter::Iterator::find_map`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.find_map)

    [`std::iter::Iterator::position`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.position)

    [`std::iter::Iterator::rposition`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.rposition)
*/




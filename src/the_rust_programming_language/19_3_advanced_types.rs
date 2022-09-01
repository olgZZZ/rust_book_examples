// Advanced Types
// https://doc.rust-lang.org/stable/book/ch19-04-advanced-types.html





// Creating Type Synonyms with Type Aliases


/*
Rust provides the ability to declare a type alias to give an existing type another name. For this we use the type keyword. For example, we can create the alias Kilometers to i32 like so:
*/
    type Kilometers = i32;



/*
Now, the alias Kilometers is a synonym for i32; unlike the Millimeters and Meters types we created in Listing 19-15, Kilometers is not a separate, new type. Values that have the type Kilometers will be treated the same as values of type i32:
*/
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);



/*
The main use case for type synonyms is to reduce repetition. For example, we might have a lengthy type like this:
*/
Box<dyn Fn() + Send + 'static>





/*
Writing this lengthy type in function signatures and as type annotations all over the code can be tiresome and error prone. Imagine having a project full of code like that in Listing 19-24.

Listing 19-24: Using a long type in many places
*/
    let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));

    fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {
        // --snip--
    }

    fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {
        // --snip--
    }



/*
A type alias makes this code more manageable by reducing the repetition. In Listing 19-25, we’ve introduced an alias named Thunk for the verbose type and can replace all uses of the type with the shorter alias Thunk.

Listing 19-25: Introducing a type alias Thunk to reduce repetition
*/
    type Thunk = Box<dyn Fn() + Send + 'static>;

    let f: Thunk = Box::new(|| println!("hi"));

    fn takes_long_type(f: Thunk) {
        // --snip--
    }

    fn returns_long_type() -> Thunk {
        // --snip--
    }





/*
Type aliases are also commonly used with the Result<T, E> type for reducing repetition. Consider the std::io module in the standard library. I/O operations often return a Result<T, E> to handle situations when operations fail to work. This library has a std::io::Error struct that represents all possible I/O errors. Many of the functions in std::io will be returning Result<T, E> where the E is std::io::Error, such as these functions in the Write trait:
*/
use std::fmt;
use std::io::Error;

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
    fn flush(&mut self) -> Result<(), Error>;

    fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
}





/*
The Result<..., Error> is repeated a lot. As such, std::io has this type alias declaration:
*/
type Result<T> = std::result::Result<T, std::io::Error>;



/*
Because this declaration is in the std::io module, we can use the fully qualified alias std::io::Result<T>; that is, a Result<T, E> with the E filled in as std::io::Error. The Write trait function signatures end up looking like this:
*/
pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
}




// The Never Type that Never Returns

/*
Rust has a special type named ! that’s known in type theory lingo as the empty type because it has no values. We prefer to call it the never type because it stands in the place of the return type when a function will never return. Here is an example:
*/
fn bar() -> ! {
    // --snip--
}




/*
This code is read as “the function bar returns never.” Functions that return never are called diverging functions. We can’t create values of the type ! so bar can never possibly return.

But what use is a type you can never create values for? Recall the code from Listing 2-5, part of the number guessing game; we’ve reproduced a bit of it here in Listing 19-26.

Listing 19-26: A match with an arm that ends in continue
*/
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };





/*
At the time, we skipped over some details in this code. In Chapter 6 in “The match Control Flow Operator” section, we discussed that match arms must all return the same type. So, for example, the following code doesn’t work:
*/
 [This code does not compile!] 
    let guess = match guess.trim().parse() {
        Ok(_) => 5,
        Err(_) => "hello",
    };




/*
The never type is useful with the panic! macro as well. Recall the unwrap function that we call on Option<T> values to produce a value or panic with this definition:
*/
impl<T> Option<T> {
    pub fn unwrap(self) -> T {
        match self {
            Some(val) => val,
            None => panic!("called `Option::unwrap()` on a `None` value"),
        }
    }
}




/*
In this code, the same thing happens as in the match in Listing 19-26: Rust sees that val has the type T and panic! has the type !, so the result of the overall match expression is T. This code works because panic! doesn’t produce a value; it ends the program. In the None case, we won’t be returning a value from unwrap, so this code is valid.

One final expression that has the type ! is a loop:
*/
    print!("forever ");

    loop {
        print!("and ever ");
    }







// Dynamically Sized Types and the Sized Trait

/*
Let’s dig into the details of a dynamically sized type called str, which we’ve been using throughout the book. That’s right, not &str, but str on its own, is a DST. We can’t know how long the string is until runtime, meaning we can’t create a variable of type str, nor can we take an argument of type str. Consider the following code, which does not work:
*/
    let s1: str = "Hello there!";
    let s2: str = "How's it going?";





/*
To work with DSTs, Rust provides the Sized trait to determine whether or not a type’s size is known at compile time. This trait is automatically implemented for everything whose size is known at compile time. In addition, Rust implicitly adds a bound on Sized to every generic function. That is, a generic function definition like this:
*/
fn generic<T>(t: T) {
    // --snip--
}




/*
is actually treated as though we had written this:
*/
fn generic<T: Sized>(t: T) {
    // --snip--
}




/*
By default, generic functions will work only on types that have a known size at compile time. However, you can use the following special syntax to relax this restriction:
*/
fn generic<T: ?Sized>(t: &T) {
    // --snip--
}






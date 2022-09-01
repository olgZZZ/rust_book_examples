// Advanced Traits
// https://doc.rust-lang.org/stable/book/ch19-03-advanced-traits.html




// Specifying Placeholder Types in Trait Definitions with Associated Types

/*
One example of a trait with an associated type is the Iterator trait that the standard library provides. The associated type is named Item and stands in for the type of the values the type implementing the Iterator trait is iterating over. The definition of the Iterator trait is as shown in Listing 19-12.

Listing 19-12: The definition of the Iterator trait that has an associated type Item
*/
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}




/*
Associated types might seem like a similar concept to generics, in that the latter allow us to define a function without specifying what types it can handle. To examine the difference between the two concepts, we’ll look at an implementation of the Iterator trait on a type named Counter that specifies the Item type is u32:

Filename: src/lib.rs
*/
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // --snip--




/*
This syntax seems comparable to that of generics. So why not just define the Iterator trait with generics, as shown in Listing 19-13?

Listing 19-13: A hypothetical definition of the Iterator trait using generics
*/
pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}








// Default Generic Type Parameters and Operator Overloading

/*
Rust doesn’t allow you to create your own operators or overload arbitrary operators. But you can overload the operations and corresponding traits listed in std::ops by implementing the traits associated with the operator. For example, in Listing 19-14 we overload the + operator to add two Point instances together. We do this by implementing the Add trait on a Point struct:

Listing 19-14: Implementing the Add trait to overload the + operator for Point instances
Filename: src/main.rs
*/
use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
}





/*
The default generic type in this code is within the Add trait. Here is its definition:
*/
trait Add<Rhs=Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}





/*
We have two structs, Millimeters and Meters, holding values in different units. This thin wrapping of an existing type in another struct is known as the newtype pattern, which we describe in more detail in the “Using the Newtype Pattern to Implement External Traits on External Types” section. We want to add values in millimeters to values in meters and have the implementation of Add do the conversion correctly. We can implement Add for Millimeters with Meters as the Rhs, as shown in Listing 19-15.

Listing 19-15: Implementing the Add trait on Millimeters to add Millimeters to Meters
Filename: src/lib.rs
*/
use std::ops::Add;

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}


/*
To add Millimeters and Meters, we specify impl Add<Meters> to set the value of the Rhs type parameter instead of using the default of Self.

You’ll use default type parameters in two main ways:

    To extend a type without breaking existing code
    To allow customization in specific cases most users won’t need

*/








// Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name

/*
When calling methods with the same name, you’ll need to tell Rust which one you want to use. Consider the code in Listing 19-16 where we’ve defined two traits, Pilot and Wizard, that both have a method called fly. We then implement both traits on a type Human that already has a method named fly implemented on it. Each fly method does something different.

Listing 19-16: Two traits are defined to have a fly method and are implemented on the Human type, and a fly method is implemented on Human directly
Filename: src/main.rs
*/
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}








/*
When we call fly on an instance of Human, the compiler defaults to calling the method that is directly implemented on the type, as shown in Listing 19-17.

Listing 19-17: Calling fly on an instance of Human
Filename: src/main.rs
*/
fn main() {
    let person = Human;
    person.fly();
}




/*
Running this code will print *waving arms furiously*, showing that Rust called the fly method implemented on Human directly.

To call the fly methods from either the Pilot trait or the Wizard trait, we need to use more explicit syntax to specify which fly method we mean. Listing 19-18 demonstrates this syntax.

Listing 19-18: Specifying which trait’s fly method we want to call
Filename: src/main.rs
*/
fn main() {
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();
}


/*
Running this code prints the following:

$ cargo run
   Compiling traits-example v0.1.0 (file:///projects/traits-example)
    Finished dev [unoptimized + debuginfo] target(s) in 0.46s
     Running `target/debug/traits-example`
This is your captain speaking.
Up!
*waving arms furiously*
*/






/*
However, associated functions that are not methods don’t have a self parameter. When there are multiple types or traits that define non-method functions with the same function name, Rust doesn't always know which type you mean unless you use fully qualified syntax. For example, in Listing 19-19 we create a trait for an animal shelter that wants to name all baby dogs Spot. We make an Animal trait with an associated non-method function baby_name. The Animal trait is implemented for the struct Dog, on which we also provide an associated non-method function baby_name directly.

Listing 19-19: A trait with an associated function and a type with an associated function of the same name that also implements the trait
Filename: src/main.rs
*/
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn main() {
    println!("A baby dog is called a {}", Dog::baby_name());
}



/*
In main, we call the Dog::baby_name function, which calls the associated function defined on Dog directly. This code prints the following:

$ cargo run
   Compiling traits-example v0.1.0 (file:///projects/traits-example)
    Finished dev [unoptimized + debuginfo] target(s) in 0.54s
     Running `target/debug/traits-example`
A baby dog is called a Spot
*/



/*
This output isn’t what we wanted. We want to call the baby_name function that is part of the Animal trait that we implemented on Dog so the code prints A baby dog is called a puppy. The technique of specifying the trait name that we used in Listing 19-18 doesn’t help here; if we change main to the code in Listing 19-20, we’ll get a compilation error.

Listing 19-20: Attempting to call the baby_name function from the Animal trait, but Rust doesn’t know which implementation to use
Filename: src/main.rs
*/
fn main() {
    println!("A baby dog is called a {}", Animal::baby_name());
}



/*
Because Animal::baby_name doesn’t have a self parameter, and there could be other types that implement the Animal trait, Rust can’t figure out which implementation of Animal::baby_name we want. We’ll get this compiler error:

$ cargo run
   Compiling traits-example v0.1.0 (file:///projects/traits-example)
error[E0283]: type annotations needed
  --> src/main.rs:20:43
   |
20 |     println!("A baby dog is called a {}", Animal::baby_name());
   |                                           ^^^^^^^^^^^^^^^^^ cannot infer type
   |
   = note: cannot satisfy `_: Animal`

For more information about this error, try `rustc --explain E0283`.
error: could not compile `traits-example` due to previous error
*/




/*
To disambiguate and tell Rust that we want to use the implementation of Animal for Dog as opposed to the implementation of Animal for some other type, we need to use fully qualified syntax. Listing 19-21 demonstrates how to use fully qualified syntax.

Listing 19-21: Using fully qualified syntax to specify that we want to call the baby_name function from the Animal trait as implemented on Dog
Filename: src/main.rs
*/
fn main() {
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}


/*
We’re providing Rust with a type annotation within the angle brackets, which indicates we want to call the baby_name method from the Animal trait as implemented on Dog by saying that we want to treat the Dog type as an Animal for this function call. This code will now print what we want:

$ cargo run
   Compiling traits-example v0.1.0 (file:///projects/traits-example)
    Finished dev [unoptimized + debuginfo] target(s) in 0.48s
     Running `target/debug/traits-example`
A baby dog is called a puppy
*/


/*
In general, fully qualified syntax is defined as follows:

<Type as Trait>::function(receiver_if_method, next_arg, ...);
*/






// Using Supertraits to Require One Trait’s Functionality Within Another Trait

/*
For example, let’s say we want to make an OutlinePrint trait with an outline_print method that will print a given value formatted so that it's framed in asterisks. That is, given a Point struct that implements the standard library trait Display to result in (x, y), when we call outline_print on a Point instance that has 1 for x and 3 for y, it should print the following:

**********
*        *
* (1, 3) *
*        *
**********
*/



/*
In the implementation of the outline_print method, we want to use the Display trait’s functionality. Therefore, we need to specify that the OutlinePrint trait will work only for types that also implement Display and provide the functionality that OutlinePrint needs. We can do that in the trait definition by specifying OutlinePrint: Display. This technique is similar to adding a trait bound to the trait. Listing 19-22 shows an implementation of the OutlinePrint trait.

Listing 19-22: Implementing the OutlinePrint trait that requires the functionality from Display
Filename: src/main.rs
*/
use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}




/*
Let’s see what happens when we try to implement OutlinePrint on a type that doesn’t implement Display, such as the Point struct:

Filename: src/main.rs
*/
 [This code does not compile!] 
struct Point {
    x: i32,
    y: i32,
}

impl OutlinePrint for Point {}



/*
We get an error saying that Display is required but not implemented:

$ cargo run
   Compiling traits-example v0.1.0 (file:///projects/traits-example)
error[E0277]: `Point` doesn't implement `std::fmt::Display`
  --> src/main.rs:20:6
   |
20 | impl OutlinePrint for Point {}
   |      ^^^^^^^^^^^^ `Point` cannot be formatted with the default formatter
   |
   = help: the trait `std::fmt::Display` is not implemented for `Point`
   = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
note: required by a bound in `OutlinePrint`
  --> src/main.rs:3:21
   |
3  | trait OutlinePrint: fmt::Display {
   |                     ^^^^^^^^^^^^ required by this bound in `OutlinePrint`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `traits-example` due to previous error
*/





/*
To fix this, we implement Display on Point and satisfy the constraint that OutlinePrint requires, like so:

Filename: src/main.rs
*/
use std::fmt;

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}








// Using the Newtype Pattern to Implement External Traits on External Types

/*
As an example, let’s say we want to implement Display on Vec<T>, which the orphan rule prevents us from doing directly because the Display trait and the Vec<T> type are defined outside our crate. We can make a Wrapper struct that holds an instance of Vec<T>; then we can implement Display on Wrapper and use the Vec<T> value, as shown in Listing 19-23.

Listing 19-23: Creating a Wrapper type around Vec<String> to implement Display
Filename: src/main.rs
*/
use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}







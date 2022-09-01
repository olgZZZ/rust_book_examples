// Using Trait Objects That Allow for Values of Different Types
// https://doc.rust-lang.org/stable/book/ch17-02-trait-objects.html





// Defining a Trait for Common Behavior

/*
We’ve mentioned that, in Rust, we refrain from calling structs and enums “objects” to distinguish them from other languages’ objects. In a struct or enum, the data in the struct fields and the behavior in impl blocks are separated, whereas in other languages, the data and behavior combined into one concept is often labeled an object. However, trait objects are more like objects in other languages in the sense that they combine data and behavior. But trait objects differ from traditional objects in that we can’t add data to a trait object. Trait objects aren’t as generally useful as objects in other languages: their specific purpose is to allow abstraction across common behavior.

Listing 17-3 shows how to define a trait named Draw with one method named draw:

Listing 17-3: Definition of the Draw trait
Filename: src/lib.rs
*/
pub trait Draw {
    fn draw(&self);
}




/*
This syntax should look familiar from our discussions on how to define traits in Chapter 10. Next comes some new syntax: Listing 17-4 defines a struct named Screen that holds a vector named components. This vector is of type Box<dyn Draw>, which is a trait object; it’s a stand-in for any type inside a Box that implements the Draw trait.

Listing 17-4: Definition of the Screen struct with a components field holding a vector of trait objects that implement the Draw trait
Filename: src/lib.rs
*/
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}




/*
On the Screen struct, we’ll define a method named run that will call the draw method on each of its components, as shown in Listing 17-5:

Listing 17-5: A run method on Screen that calls the draw method on each component
Filename: src/lib.rs
*/
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}





/*
This works differently from defining a struct that uses a generic type parameter with trait bounds. A generic type parameter can only be substituted with one concrete type at a time, whereas trait objects allow for multiple concrete types to fill in for the trait object at runtime. For example, we could have defined the Screen struct using a generic type and a trait bound as in Listing 17-6:

Listing 17-6: An alternate implementation of the Screen struct and its run method using generics and trait bounds
Filename: src/lib.rs
*/
pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen<T>
where
    T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}







// Implementing the Trait

/*
Now we’ll add some types that implement the Draw trait. We’ll provide the Button type. Again, actually implementing a GUI library is beyond the scope of this book, so the draw method won’t have any useful implementation in its body. To imagine what the implementation might look like, a Button struct might have fields for width, height, and label, as shown in Listing 17-7:

Listing 17-7: A Button struct that implements the Draw trait
Filename: src/lib.rs
*/
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}





/*
If someone using our library decides to implement a SelectBox struct that has width, height, and options fields, they implement the Draw trait on the SelectBox type as well, as shown in Listing 17-8:

Listing 17-8: Another crate using gui and implementing the Draw trait on a SelectBox struct
Filename: src/main.rs
*/
use gui::Draw;

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}








/*
Our library’s user can now write their main function to create a Screen instance. To the Screen instance, they can add a SelectBox and a Button by putting each in a Box<T> to become a trait object. They can then call the run method on the Screen instance, which will call draw on each of the components. Listing 17-9 shows this implementation:

Listing 17-9: Using trait objects to store values of different types that implement the same trait
Filename: src/main.rs
*/
use gui::{Button, Screen};

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}






/*
For example, Listing 17-10 shows what happens if we try to create a Screen with a String as a component:

Listing 17-10: Attempting to use a type that doesn’t implement the trait object’s trait
Filename: src/main.rs
*/
 [This code does not compile!] 
use gui::Screen;

fn main() {
    let screen = Screen {
        components: vec![Box::new(String::from("Hi"))],
    };

    screen.run();
}



/*
We’ll get this error because String doesn’t implement the Draw trait:

$ cargo run
   Compiling gui v0.1.0 (file:///projects/gui)
error[E0277]: the trait bound `String: Draw` is not satisfied
 --> src/main.rs:5:26
  |
5 |         components: vec![Box::new(String::from("Hi"))],
  |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Draw` is not implemented for `String`
  |
  = note: required for the cast to the object type `dyn Draw`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `gui` due to previous error
*/



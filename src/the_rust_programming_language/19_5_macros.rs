// Macros
// https://doc.rust-lang.org/stable/book/ch19-06-macros.html





// Declarative Macros with macro_rules! for General Metaprogramming

/*
To define a macro, you use the macro_rules! construct. Let’s explore how to use macro_rules! by looking at how the vec! macro is defined. Chapter 8 covered how we can use the vec! macro to create a new vector with particular values. For example, the following macro creates a new vector containing three integers:
*/
let v: Vec<u32> = vec![1, 2, 3];



/*
We could also use the vec! macro to make a vector of two integers or a vector of five string slices. We wouldn’t be able to use a function to do the same because we wouldn’t know the number or type of values up front.

Listing 19-28 shows a slightly simplified definition of the vec! macro.

Listing 19-28: A simplified version of the vec! macro definition
Filename: src/lib.rs
*/
#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}






/*
Now let’s look at the pattern in the body of the code associated with this arm: temp_vec.push() within $()* is generated for each part that matches $() in the pattern zero or more times depending on how many times the pattern matches. The $x is replaced with each expression matched. When we call this macro with vec![1, 2, 3];, the code generated that replaces this macro call will be the following:
*/
{
    let mut temp_vec = Vec::new();
    temp_vec.push(1);
    temp_vec.push(2);
    temp_vec.push(3);
    temp_vec
}








// Procedural Macros for Generating Code from Attributes

/*
When creating procedural macros, the definitions must reside in their own crate with a special crate type. This is for complex technical reasons that we hope to eliminate in the future. In Listing 19-29, we show how to define a procedural macro, where some_attribute is a placeholder for using a specific macro variety.

Listing 19-29: An example of defining a procedural macro
Filename: src/lib.rs
*/
use proc_macro;

#[some_attribute]
pub fn some_name(input: TokenStream) -> TokenStream {
}







// How to Write a Custom derive Macro

/*
Let’s create a crate named hello_macro that defines a trait named HelloMacro with one associated function named hello_macro. Rather than making our users implement the HelloMacro trait for each of their types, we’ll provide a procedural macro so users can annotate their type with #[derive(HelloMacro)] to get a default implementation of the hello_macro function. The default implementation will print Hello, Macro! My name is TypeName! where TypeName is the name of the type on which this trait has been defined. In other words, we’ll write a crate that enables another programmer to write code like Listing 19-30 using our crate.

Listing 19-30: The code a user of our crate will be able to write when using our procedural macro
Filename: src/main.rs
*/
 [This code does not compile!] 
use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    Pancakes::hello_macro();
}


/*
This code will print Hello, Macro! My name is Pancakes! when we’re done. The first step is to make a new library crate, like this:

$ cargo new hello_macro --lib
*/




/*
Next, we’ll define the HelloMacro trait and its associated function:

Filename: src/lib.rs
*/
pub trait HelloMacro {
    fn hello_macro();
}



/*
We have a trait and its function. At this point, our crate user could implement the trait to achieve the desired functionality, like so:
*/
use hello_macro::HelloMacro;

struct Pancakes;

impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello, Macro! My name is Pancakes!");
    }
}

fn main() {
    Pancakes::hello_macro();
}





/*
The next step is to define the procedural macro. At the time of this writing, procedural macros need to be in their own crate. Eventually, this restriction might be lifted. The convention for structuring crates and macro crates is as follows: for a crate named foo, a custom derive procedural macro crate is called foo_derive. Let’s start a new crate called hello_macro_derive inside our hello_macro project:

$ cargo new hello_macro_derive --lib
*/





/*
We need to declare the hello_macro_derive crate as a procedural macro crate. We’ll also need functionality from the syn and quote crates, as you’ll see in a moment, so we need to add them as dependencies. Add the following to the Cargo.toml file for hello_macro_derive:

Filename: hello_macro_derive/Cargo.toml

[lib]
proc-macro = true

[dependencies]
syn = "1.0"
quote = "1.0"
*/





/*
To start defining the procedural macro, place the code in Listing 19-31 into your src/lib.rs file for the hello_macro_derive crate. Note that this code won’t compile until we add a definition for the impl_hello_macro function.

Listing 19-31: Code that most procedural macro crates will require in order to process Rust code
Filename: hello_macro_derive/src/lib.rs
*/
 [This code does not compile!] 
use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
}





/*
The hello_macro_derive function first converts the input from a TokenStream to a data structure that we can then interpret and perform operations on. This is where syn comes into play. The parse function in syn takes a TokenStream and returns a DeriveInput struct representing the parsed Rust code. Listing 19-32 shows the relevant parts of the DeriveInput struct we get from parsing the struct Pancakes; string:

Listing 19-32: The DeriveInput instance we get when parsing the code that has the macro’s attribute in Listing 19-30

DeriveInput {
    // --snip--

    ident: Ident {
        ident: "Pancakes",
        span: #0 bytes(95..103)
    },
    data: Struct(
        DataStruct {
            struct_token: Struct,
            fields: Unit,
            semi_token: Some(
                Semi
            )
        }
    )
}
*/






/*

Now that we have the code to turn the annotated Rust code from a TokenStream into a DeriveInput instance, let’s generate the code that implements the HelloMacro trait on the annotated type, as shown in Listing 19-33.

Listing 19-33: Implementing the HelloMacro trait using the parsed Rust code
Filename: hello_macro_derive/src/lib.rs

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}
*/





/*
At this point, cargo build should complete successfully in both hello_macro and hello_macro_derive. Let’s hook up these crates to the code in Listing 19-30 to see the procedural macro in action! Create a new binary project in your projects directory using cargo new pancakes. We need to add hello_macro and hello_macro_derive as dependencies in the pancakes crate’s Cargo.toml. If you’re publishing your versions of hello_macro and hello_macro_derive to crates.io, they would be regular dependencies; if not, you can specify them as path dependencies as follows:

hello_macro = { path = "../hello_macro" }
hello_macro_derive = { path = "../hello_macro/hello_macro_derive" }
*/







// Attribute-like macros

/*
Attribute-like macros are similar to custom derive macros, but instead of generating code for the derive attribute, they allow you to create new attributes. They’re also more flexible: derive only works for structs and enums; attributes can be applied to other items as well, such as functions. Here’s an example of using an attribute-like macro: say you have an attribute named route that annotates functions when using a web application framework:

#[route(GET, "/")]
fn index() {
*/




/*
This #[route] attribute would be defined by the framework as a procedural macro. The signature of the macro definition function would look like this:

#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
*/






// Function-like macros

/*
Function-like macros define macros that look like function calls. Similarly to macro_rules! macros, they’re more flexible than functions; for example, they can take an unknown number of arguments. However, macro_rules! macros can be defined only using the match-like syntax we discussed in the section “Declarative Macros with macro_rules! for General Metaprogramming” earlier. Function-like macros take a TokenStream parameter and their definition manipulates that TokenStream using Rust code as the other two types of procedural macros do. An example of a function-like macro is an sql! macro that might be called like so:

let sql = sql!(SELECT * FROM posts WHERE id=1);
*/


/*
This macro would parse the SQL statement inside it and check that it’s syntactically correct, which is much more complex processing than a macro_rules! macro can do. The sql! macro would be defined like this:

#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {

This definition is similar to the custom derive macro’s signature: we receive the tokens that are inside the parentheses and return the code we wanted to generate.
*/







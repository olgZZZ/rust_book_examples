// Storing Lists of Values with Vectors


// Creating a New Vector

// Listing 8-1: Creating a new, empty vector to hold values of type i32
fn main() {
    let v: Vec<i32> = Vec::new();
}



// Listing 8-2: Creating a new vector containing values
fn main() {
    let v = vec![1, 2, 3];
}




// Updating a Vector

// Listing 8-3: Using the push method to add values to a vector
fn main() {
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
}





// Reading Elements of Vectors

// Listing 8-4: Using indexing syntax or the get method to access an item in a vector
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
}




// Listing 8-5: Attempting to access the element at index 100 in a vector containing five elements
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let does_not_exist = &v[100];
    let does_not_exist = v.get(100);
}



// Listing 8-6: Attempting to add an element to a vector while holding a reference to an item
fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    v.push(6);

    println!("The first element is: {}", first);
}

// Compiling this code will result in this error:

/*
$ cargo run
   Compiling collections v0.1.0 (file:///projects/collections)
error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
 --> src/main.rs:6:5
  |
4 |     let first = &v[0];
  |                  - immutable borrow occurs here
5 | 
6 |     v.push(6);
  |     ^^^^^^^^^ mutable borrow occurs here
7 | 
8 |     println!("The first element is: {}", first);
  |                                          ----- immutable borrow later used here

For more information about this error, try `rustc --explain E0502`.
error: could not compile `collections` due to previous error

*/





// Iterating over the Values in a Vector

// Listing 8-7: Printing each element in a vector by iterating over the elements using a for loop
fn main() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
}


// Listing 8-8: Iterating over mutable references to elements in a vector
fn main() {
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
}



// Using an Enum to Store Multiple Types

// Listing 8-9: Defining an enum to store values of different types in one vector
fn main() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}



// Dropping a Vector Drops Its Elements

// Listing 8-10: Showing where the vector and its elements are dropped
fn main() {
    {
        let v = vec![1, 2, 3, 4];

        // do stuff with v
    } // <- v goes out of scope and is freed here
}







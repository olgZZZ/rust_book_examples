// Storing Keys with Associated Values in Hash Maps
// https://doc.rust-lang.org/stable/book/ch08-03-hash-maps.html



// Creating a New Hash Map

// Listing 8-20: Creating a new hash map and inserting some keys and values
fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
}




// Accessing Values in a Hash Map

// Listing 8-21: Accessing the score for the Blue team stored in the hash map
fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
}




// We can iterate over each key/value pair in a hash map in a similar manner as we 
// do with vectors, using a for loop:
fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}


/*
    Yellow: 50
    Blue: 10
*/




// Hash Maps and Ownership

// Listing 8-22: Showing that keys and values are owned by the hash map once they’re inserted
fn main() {
    use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
}




// Overwriting a Value

// Listing 8-23: Replacing a value stored with a particular key
fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);
}



// Adding a Key and Value Only If a Key Isn’t Present

// Listing 8-24: Using the entry method to only insert if the key does not already have a value
fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
}





// Updating a Value Based on the Old Value

// Listing 8-25: Counting occurrences of words using a hash map that stores words and counts
fn main() {
    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}






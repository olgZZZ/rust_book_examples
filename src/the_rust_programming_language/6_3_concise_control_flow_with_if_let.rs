// Concise Control Flow with if let
// https://doc.rust-lang.org/stable/book/ch06-03-if-let.html


// Listing 6-6: A match that only cares about executing code when the value is Some
fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
}



// Instead, we could write this in a shorter way using if let. The following code 
// behaves the same as the match in Listing 6-6:
fn main() {
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}



// Recall the Coin enum definition in Listing 6-4, where the Quarter variant also held 
// a UsState value. If we wanted to count all non-quarter coins we see while also announcing 
// the state of the quarters, we could do that with a match expression like this:
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let coin = Coin::Penny;
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
}





// Or we could use an if let and else expression like this:
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let coin = Coin::Penny;
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}























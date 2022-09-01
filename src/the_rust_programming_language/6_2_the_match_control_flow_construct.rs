// The match Control Flow Construct
// https://doc.rust-lang.org/stable/book/ch06-02-match.html





// Listing 6-3: An enum and a match expression that has the variants of the enum as its patterns
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {}






// For example, the following code prints “Lucky penny!” every time the method 
// is called with a Coin::Penny, but still returns the last value of the block, 1:
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {}







// Patterns that Bind to Values

// Listing 6-4: A Coin enum in which the Quarter variant also holds a UsState value
#[derive(Debug)] // so we can inspect the state in a minute
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

fn main() {}





// When a Coin::Quarter matches, the state variable will bind to the value of that 
// quarter’s state. Then we can use state in the code for that arm, like so:
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

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main() {
    value_in_cents(Coin::Quarter(UsState::Alaska));
}






// Matching with Option<T>

// Listing 6-5: A function that uses a match expression on an Option<i32>
fn main() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}







// Matches Are Exhaustive

// Consider this version of our plus_one function, which has a bug and won’t compile:
fn main() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}


/* $ cargo run
   Compiling enums v0.1.0 (file:///projects/enums)
error[E0004]: non-exhaustive patterns: `None` not covered
   --> src/main.rs:3:15
    |
3   |         match x {
    |               ^ pattern `None` not covered
    |
note: `Option<i32>` defined here
    = note: the matched value is of type `Option<i32>`
help: ensure that all possible cases are being handled by adding a match arm with a 
wildcard pattern or an explicit pattern as shown
    |
4   ~             Some(i) => Some(i + 1),
5   ~             None => todo!(),
    |

For more information about this error, try `rustc --explain E0004`.
error: could not compile `enums` due to previous error
*/






// Catch-all Patterns and the _ Placeholder


fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}
}



// Let’s change the rules of the game: now, if you roll anything other than a 3 or a 7, 
// you must roll again. We no longer need to use the catch-all value, so we can change 
// our code to use _ instead of the variable named other:
fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn reroll() {}
}





// Finally, we’ll change the rules of the game one more time, so that nothing else happens 
// on your turn if you roll anything other than a 3 or a 7. We can express that by using the 
// unit value (the empty tuple type we mentioned in “The Tuple Type” section) as the code that 
// goes with the _ arm:
fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
}

















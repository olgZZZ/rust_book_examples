use rand::Rng;
use std::cmp::Ordering;
use std::io;

#[derive(Debug)]
pub enum Err {
    ErrValue(i32),
}

#[derive(Debug)]
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Result<Guess, Err> {
        if value < 1 || value > 100 {
            println!("Guess value must be between 1 and 100, obtained {}.", value);
            return Err(Err::ErrValue(value));
        }
        Ok (Guess { value })
    }
    
    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..100);

    // println!("The secret number is: {}", secret_number);

    loop {
        println!("Please imput your quess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        let guess = match Guess::new(guess) {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You quessed: {}", guess.value);

        match guess.value.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
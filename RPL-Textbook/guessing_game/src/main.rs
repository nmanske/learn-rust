use std::io;
use std::cmp::Ordering;

use rand::Rng;

extern crate rand;

pub struct Guess {
    value: isize,
}

impl Guess {
    pub fn new(value: isize) -> Result<Guess, &'static str> {
        if value < 1 || value > 100 {
            return Err("guess must be between 0 and 100!");
        }
        Ok(Guess { value })
    }

    pub fn value(&self) -> isize {
        self.value
    }
}

fn main() {
    println!("\nGuess a number betwene 1 and 100!");

    let mut secret_number: isize = rand::thread_rng().gen_range(1,101);

    loop {
        println!("\nPlease input your guess: ");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Read line failed for stdin");

        // String to int
        let guess: isize = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("Parsing error: {}", err);
                continue;
            },
        };

        // Check range before assigning guess value
        let guess = match Guess::new(guess) {
            Ok(val) => val,
            Err(err) => {
                println!("Range error: {}", err);
                continue;
            },
        };

        println!("You guessed the number {}", guess.value());

        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("You win! Congrats!\nWould you like to continue? (Y/N)");

                let mut answer = String::new();
                io::stdin().read_line(&mut answer).expect("Read line failed for stdin");

                let answer = answer.to_uppercase().chars().next().unwrap();

                if answer == 'N' {
                    break;
                }

                secret_number = rand::thread_rng().gen_range(1,101);
            },
            Ordering::Greater => println!("Too large"),
        };
    }
}

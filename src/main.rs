use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    //print the two messages below to the console/terminal
    //Note the `println!() is called macros in rust and are somewhat different from functions
    println!("Welcome to the Guessing Game");

    loop {
        println!("Enter your random number!");

        //Here, we define a mutable variable and initialize it as a string.
        //In Rust, variables are immutable by default and hence their values can't be changed.
        //To make a variable mutable, use the mut keyword
        let mut guess = String::new();

        //Read the user input from the terminal using the standard io library
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");

        // let guess: u32 = guess.trim().parse().expect("Please enter a umber");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guess was {guess}"); // or println!("Your guess was {}", guess);

        let secret_number = rand::thread_rng().gen_range(1..=100);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Your Guess is lower than the secret value!"),
            Ordering::Greater => println!("Your guess is greater than the secret value!"),
            Ordering::Equal => {
                println!("Bingo, You win!");
                break;
            }
        }

        // println!("The secret number is: {secret_number}");
    }
}

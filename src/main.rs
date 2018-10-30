// By default rust extern carte std in the "prelude"
// this means we can "use" std and its sub modules i.e std::io

extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;



fn main() {
    println!("Welcome to Guess the number!"); 

    let secret_number = rand::thread_rng().gen_range(1, 101); // immutable value 
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line");
    
        // Shadow the previous guess with a new guess variable ( to reuse the name )
        // use a inline match statement to switch on the Result type instead of using .expect if an error is returned 
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => { 
                println!("You win!");
                break;
            }
        } 
    }
}

use std::{cmp::Ordering::{self, Equal}, io};
use rand::{Rng, RngExt}; // Added the missing semicolon here

fn main() {
    let mut rng = rand::rng();
    let random_number = rng.random_range(1..=100);
    println!("{random_number}");

    loop {
        println!("Guess one number :: ");

    let mut guess: String = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to take input");

    let guess_number:i32 = guess.trim().parse().expect("Failed");
    match guess_number.cmp(&random_number) {
      Ordering::Less=>{
        println!("Less number");
      },
        Ordering::Greater=>{
            println!("Greater number");
        },
        Ordering::Equal=>{
            println!("Equall..");
            break;
        }
        
    }

    println!("Your guess is {guess}");
    }
    

}

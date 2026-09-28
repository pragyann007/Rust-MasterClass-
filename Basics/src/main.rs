use std::{cmp::Ordering::{self, Equal}, io};
use rand::{Rng, RngExt}; // Added the missing semicolon here

fn main() {
    
    let mut  rng = rand::rng();
    let secret_number = rng.random_range(1..=100);

    loop {

    println!("Guess the Number ");
    let mut guess : String = String::new();
     io::stdin().read_line(&mut guess).expect("Failed to read");

     let guess_number:i32 = guess.trim().parse().expect("Cant convert to integer");

    match guess_number.cmp(&secret_number){
        Ordering::Less=>println!("Number is less than secret_number"),
        Ordering::Greater=>println!("Number is greater than secret_number"),
        Ordering::Equal=>{
            println!("Equal number , Congrats You won");
            break;
        }


     }
        
    }
}

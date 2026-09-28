/*Basic rust program for license.. */
use std::{cmp::Ordering::{self, Greater}, io} ;

fn main() {
    let standard_Age = 18 ; 

    let mut users_Age:String = String::new();


    println!("Input you age");
    io::stdin().read_line(&mut users_Age).expect("Failed");

    let users_Age:i32 = users_Age.trim().parse().expect("Parsing failed");

    match users_Age.cmp(&standard_Age){


        Ordering::Equal=>{
            println!("You are eligible");
        }
        Ordering::Less=>{
            println!("You are not eligible");
        }
        Ordering::Greater=>{
            println!("You are eligible");
        }
    }



    



    
    
}

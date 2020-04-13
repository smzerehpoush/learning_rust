use std::cmp::Ordering;
use std::io;

use rand::Rng;

//The first part of the guessing game program will ask for user input, process that input, and check that the input is in the expected form.
//To start, we’ll allow the player to input a guess. Enter the code
fn main() {
    println!("# guessing game #");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    // println!("secret number is : {}", secret_number);
    loop {
        println!("input : ");
        //defining guess variable
        let mut guess = String::new();
        //initializing guess with user input
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        // this is shadowing guess variable
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // note that _ is a catch all value
            Err(_) => {
                println!("bad input,");
                continue;
            }
        };
        // with this annotation let foo : type, we define var types for rust
        //trim removes whitespaces and also new lines
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
            Ordering::Greater => println!("Too big")
        }
    }
}

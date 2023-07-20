use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Welcome to my guessing game!");
    let secret_number = rand::thread_rng().gen_rng(1..=400)
    loop {
        println!("Please guess a number");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That's not a number or its out of range");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Too big!"),
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You won!!!");
                break;
            }
        }
    }
}

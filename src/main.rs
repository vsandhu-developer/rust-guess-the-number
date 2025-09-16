use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number game");

    let secret_number = rand::rng().random_range(1..=100);

    println!("Enter your guess");

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to get user input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Error");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Small Number"),
            Ordering::Equal => {
                println!("You won");
                break;
            }
            Ordering::Greater => println!("Big Number"),
        }

        println!("Your Guess {guess}");
    }
}

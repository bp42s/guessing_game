use rand::Rng;
/*
use std::cmp::Ordering;
use std::io;
*/

fn main() {
    std::println!("Guess the number!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);
    std::println!("The secret number is: {secret_number}");

    loop {
        std::println!("Please enter your guess.");

        let mut guess: String = String::new();

        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed {guess}");

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => std::println!("Too small!"),
            std::cmp::Ordering::Greater => std::println!("Too big!"),
            std::cmp::Ordering::Equal => {
                std::println!("You win!");
                break;
            }
        }
    }
}

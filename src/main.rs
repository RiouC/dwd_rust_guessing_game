// use core::num;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100); // 1(include), 100(exclude), =100(include)

    println!("Guess the number!");

    loop {
        println!("What is your guess ?");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(gu_n) => gu_n,
            // 1st option
            // Err(_) => continue,
            Err(_) => {
                println!("Your guess {} is not a number, please enter a positive number between 1 and 100",
                         guess
                );
                continue;
            }
        };

        println!("You guessed : {}", guess);

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

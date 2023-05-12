use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    //println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess: String = String::new();

        /*we want to convert the String the program reads as input
        into a real number type so we can compare it numerically
        to the secret number. We do so by adding this line to
        the main function body:
        */

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to real line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guedssed: {guess}");

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

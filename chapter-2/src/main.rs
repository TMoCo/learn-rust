use rand::Rng; //Rng trait defines methods that random number generators implement
use std::cmp::Ordering; // Ordering type from cmp library brought into scope
use std::io; // bring io library from std into scope

fn main() {
    println!("Guess the number!");

    // range expression from 1 to 100, also as [1,100]
    let secret_number = rand::thread_rng().gen_range(1..=100); // immutable variable

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // mutable variable

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // variable shadowing by redeclaring with same name
        let guess: u32 = match guess.trim().parse() { 
            // match expression like in Scala
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

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
use rand::Rng;
use std::io;

fn main() {
    let index = rand::thread_rng().gen_range(1..=10);

    println!("Guess a number from 1 to 10!");

    'outer: loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");
        let parsed_guess = guess.trim().parse::<i32>();
        match parsed_guess {
            Ok(n) => {
                if index == n {
                    println!("You win!");
                    break 'outer;
                } else {
                    println!("Wrong. Try again.");
                }
            }
            Err(err) => {
                println!("{}", &err);
            }
        };
    }
}

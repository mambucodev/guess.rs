use std::cmp::Ordering;
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
                match n.cmp(&index) {
                    Ordering::Less => println!("Go higher..."),
                    Ordering::Greater => println!("Go lower..."),
                    Ordering::Equal => {
                        println!("You won! It was {}.", n);
                        break 'outer;
                    }
                }
            }
            Err(err) => {
                println!("{}", &err);
            }
        };
    }
}

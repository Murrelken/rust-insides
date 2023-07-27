use std::io;
use rand::Rng;
use std::cmp::Ordering;

pub fn guessing_game() {
    let secret_number = rand::thread_rng().gen_range(10..=100);
    println!("Hint: { } + 7", secret_number - 7);

    println!("Guess the number!");

    let mut count: u32 = 0;

    let mut guess = String::new();

    loop {
        println!("Please input your guess.");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {guess}");

        let guess: u32 = match guess
            .trim()
            .parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        count += 1;

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You won!");
                println!("It took you {count} attempts...");
                break;
            }
        }
    }
}

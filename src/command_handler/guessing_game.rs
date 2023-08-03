use rand::Rng;
use std::cmp::Ordering;
use crate::command_handler::input_output_handler::{InputReceiver, Printer};

pub fn guessing_game<IO>() -> Result<(), String> where
    IO: Printer + InputReceiver<u32> {
    let secret_number = rand::thread_rng().gen_range(10..=100);
    IO::print(&format!("Hint: { } + 7", secret_number - 7));

    IO::print("Guess the number!");

    let mut count: u32 = 0;

    loop {
        IO::print("Please input your guess.");

        let guess = IO::try_get_input()?;

        IO::print(&format!("You guessed: {guess}"));

        count += 1;

        match guess.cmp(&secret_number) {
            Ordering::Less => IO::print("Too small!"),
            Ordering::Greater => IO::print("Too big!"),
            Ordering::Equal => {
                IO::print("You won!");
                IO::print(&format!("It took you {count} attempts..."));
                break;
            }
        }
    }

    Ok(())
}

mod guessing_game;
mod fibonacci_calculator;
mod degrees_convertor;
mod abstraction_test_site;

use std::io;
use guessing_game::guessing_game;
use fibonacci_calculator::ith_fibonacci;
use degrees_convertor::celsius_to_fahrenheit;
use degrees_convertor::fahrenheit_to_celsius;

fn main() {
    run_command_selection();
}

fn run_command_selection() {
    const FUNCTIONS: [&str; 4] = [
        "guessing_game()",
        "ith_fibonacci()",
        "celsius_to_fahrenheit()",
        "fahrenheit_to_celsius()",
    ];

    let commands = FUNCTIONS.map(|s| format!("Running {s}..."));

    println!("Pick one of available commands.");
    for (i, str) in FUNCTIONS.iter().enumerate() {
        println!("{ }: {str}", i + 1);
    }

    let mut command = String::new();
    io::stdin()
        .read_line(&mut command)
        .expect("Failed to read line");

    command = String::from("0");

    let command: u32 = match command
        .trim()
        .parse() {
        Ok(num) => num,
        Err(_) => { return; }
    };

    match command {
        1 => {
            println!("{ }", commands[0]);
            guessing_game();
        }
        2 => 'fibonacci_match: {
            println!("Type in your i to get ith Fibonacci number.");

            let mut i = String::new();
            io::stdin()
                .read_line(&mut i)
                .expect("Failed to read line");

            let i: u32 = match i
                .trim()
                .parse() {
                Ok(num) => num,
                Err(_) => break 'fibonacci_match
            };

            println!("{ }", commands[1]);
            let ith_fib = ith_fibonacci(i);
            println!("{ith_fib}");
        }
        3 => 'celsius_to_fahrenheit: {
            println!("Type in your celsius value.");

            let mut i = String::new();
            io::stdin()
                .read_line(&mut i)
                .expect("Failed to read line");

            let i: f64 = match i
                .trim()
                .parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Can't parse the number");
                    break 'celsius_to_fahrenheit;
                }
            };

            println!("{ }", commands[2]);
            let fahrenheit_val = celsius_to_fahrenheit(i);
            println!("{fahrenheit_val}");
        }
        4 => 'fahrenheit_to_celsius: {
            println!("Type in your fahrenheit value.");

            let mut i = String::new();
            io::stdin()
                .read_line(&mut i)
                .expect("Failed to read line");

            let i: f64 = match i
                .trim()
                .parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Can't parse the number");
                    break 'fahrenheit_to_celsius;
                }
            };

            println!("{ }", commands[3]);
            let celsius_val = fahrenheit_to_celsius(i);
            println!("{celsius_val}");
        }
        _ => println!("Command not recognized...")
    }

    println!("After-party");
}

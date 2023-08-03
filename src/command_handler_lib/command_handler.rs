use crate::command_handler_lib::guessing_game::*;
use crate::command_handler_lib::fibonacci_calculator::ith_fibonacci;
use crate::command_handler_lib::degrees_convertor::*;
use crate::CommandEnum;

pub fn run_command_selection(command: CommandEnum) {
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

use super::command_enum::CommandEnum;
use super::degrees_convertor::*;
use super::fibonacci_calculator::ith_fibonacci;
use super::guessing_game::*;
use crate::command_handler::input_output_handler::*;

pub fn run<IO>(command: CommandEnum) -> Result<(), String>
where
    IO: Printer + InputReceiver<u32> + InputReceiver<f64>,
{
    IO::print("Pick one of available commands.");
    for (i, str) in FUNCTIONS.iter().enumerate() {
        IO::print(&format!("{ }: {str}", i + 1));
    }

    let commands = FUNCTIONS.map(|s| format!("Running {s}..."));
    match command {
        CommandEnum::GuessingGame => {
            IO::print(&commands[0]);
            guessing_game::<IO>()?;
        }
        CommandEnum::FibonacciGetter => {
            IO::print("Type in your i to get ith Fibonacci number.");

            let i = IO::try_get_input()?;

            IO::print(&commands[1]);
            let ith_fib = ith_fibonacci(i);
            IO::print(&ith_fib.to_string());
        }
        CommandEnum::CelsiusToFahrenheit => {
            IO::print("Type in your celsius value.");

            let i = IO::try_get_input()?;

            IO::print(&commands[2]);
            let fahrenheit_val = celsius_to_fahrenheit(i);
            IO::print(&fahrenheit_val.to_string());
        }
        CommandEnum::FahrenheitToCelsius => {
            IO::print("Type in your fahrenheit value.");

            let i = IO::try_get_input()?;

            IO::print(&commands[3]);
            let celsius_val = fahrenheit_to_celsius(i);
            IO::print(&celsius_val.to_string());
        }
    }

    IO::print("After-party");

    Ok(())
}

const FUNCTIONS: [&str; 4] = [
    "guessing_game()",
    "ith_fibonacci()",
    "celsius_to_fahrenheit()",
    "fahrenheit_to_celsius()",
];

mod console_command;
mod console_input_output_handler;

use std::io;
use std::error::Error;
use rust_insides::run_command_selection;
use console_command::ConsoleCommand;
use console_input_output_handler::ConsoleInputOutputHandler;

fn main() -> Result<(), Box<dyn Error>> {
    default_run()?;

    Ok(())
}

fn default_run() -> Result<(), Box<dyn Error>> {
    let mut command = String::new();
    io::stdin().read_line(&mut command)?;

    let command = ConsoleCommand { value: command.trim().parse()? };

    run_command_selection::<ConsoleCommand, ConsoleInputOutputHandler>(command)?;

    Ok(())
}

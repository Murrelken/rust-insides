mod command_handler_lib;

use std::io;
use command_handler_lib:: {command_handler::run_command_selection, command_enum::CommandEnum};
use std::convert::TryFrom;

fn main() {
    let mut command = String::new();
    io::stdin()
        .read_line(&mut command)
        .expect("Failed to read line");

    command = String::from("0");

    let command: i32 = match command
        .trim()
        .parse() {
        Ok(num) => num,
        Err(_) => { return; }
    };

    let command = match CommandEnum::try_from(command) {
        Ok(r) => r,
        Err(e) => panic!("{e}")
    };

    run_command_selection(command);
}


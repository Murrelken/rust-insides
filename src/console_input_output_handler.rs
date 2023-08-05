use std::io;
use std::str::FromStr;
use rust_web_server::command_handler::input_output_handler::*;

pub struct ConsoleInputOutputHandler {}

impl ConsoleInputOutputHandler {
    fn try_get_input<T>() -> Result<T, String> where
        T: FromStr {
        let mut i = String::new();
        match io::stdin().read_line(&mut i) {
            Ok(_) => {}
            Err(_) => return Err(String::from("Couldn't read the line"))
        };

        match i.trim().parse() {
            Ok(x) => Ok(x),
            Err(_) => Err(String::from("Couldn't parse the line"))
        }
    }
}

impl Printer for ConsoleInputOutputHandler {
    fn print(message: &str) {
        println!("{}", message);
    }
}

impl InputReceiver<u32> for ConsoleInputOutputHandler {
    fn try_get_input() -> Result<u32, String> {
        Self::try_get_input()
    }
}

impl InputReceiver<f64> for ConsoleInputOutputHandler {
    fn try_get_input() -> Result<f64, String> {
        Self::try_get_input()
    }
}

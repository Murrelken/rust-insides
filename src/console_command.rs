use rust_insides::command_handler::command_enum::CommandEnum;

pub struct ConsoleCommand {
    pub value: i32,
}

impl TryInto<CommandEnum> for ConsoleCommand {
    type Error = &'static str;

    fn try_into(self) -> Result<CommandEnum, Self::Error> {
        match self.value {
            1 => Ok(CommandEnum::GuessingGame),
            2 => Ok(CommandEnum::FibonacciGetter),
            3 => Ok(CommandEnum::CelsiusToFahrenheit),
            4 => Ok(CommandEnum::FahrenheitToCelsius),
            _ => Err("Command not recognized...")
        }
    }
}

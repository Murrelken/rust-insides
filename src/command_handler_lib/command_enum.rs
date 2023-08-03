use std::convert::TryFrom;

#[derive(Debug)]
pub enum CommandEnum {
    GuessingGame,
    FibonacciGetter,
    CelsiusToFahrenheit,
    FahrenheitToCelsius
}

impl TryFrom<i32> for CommandEnum {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => {
                Ok(CommandEnum::GuessingGame)
            }
            2 => 'fibonacci_match: {
                Ok(CommandEnum::FibonacciGetter)
            }
            3 => 'celsius_to_fahrenheit: {
                Ok(CommandEnum::CelsiusToFahrenheit)
            }
            4 => 'fahrenheit_to_celsius: {
                Ok(CommandEnum::FahrenheitToCelsius)
            }
            _ => Err("Command not recognized...")
        }
    }
}
pub fn celsius_to_fahrenheit(celsius_value:f64) -> f64 {
    celsius_value * 9f64 / 5f64 + 32f64
}

pub fn fahrenheit_to_celsius(fahrenheit_value:f64) -> f64 {
    (fahrenheit_value - 32f64) * 5f64 / 9f64
}
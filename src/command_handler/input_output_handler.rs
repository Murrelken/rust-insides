pub trait Printer {
    fn print(message: &str);
}

pub trait InputReceiver<T> {
    fn try_get_input() -> Result<T, String>;
}

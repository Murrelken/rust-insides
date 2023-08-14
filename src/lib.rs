mod abstraction_test_site;
mod closures_and_iterators_test_site;
pub mod command_handler;
mod smart_pointers_test_site;

pub use closures_and_iterators_test_site::pixel::*;
pub use closures_and_iterators_test_site::tests_presets::*;

use crate::command_handler::command_enum::CommandEnum;
use crate::command_handler::command_runner::run;
use crate::command_handler::input_output_handler::*;
use std::convert::TryInto;

pub fn run_command_selection<T, IO>(command: T) -> Result<(), String>
where
    T: TryInto<CommandEnum>,
    String: From<<T as TryInto<CommandEnum>>::Error>,
    IO: Printer + InputReceiver<u32> + InputReceiver<f64>,
{
    let command = command.try_into()?;
    run::<IO>(command)?;
    Ok(())
}

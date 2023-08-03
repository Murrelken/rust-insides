pub mod command_handler;

use std::convert::TryInto;
use crate::command_handler::command_runner::run;
use crate::command_handler::command_enum::CommandEnum;
use crate::command_handler::input_output_handler::*;

pub fn run_command_selection<T, IO>(command: T) -> Result<(), String> where
    T: TryInto<CommandEnum>,
    String: From<<T as TryInto<CommandEnum>>::Error>,
    IO: Printer + InputReceiver<u32> + InputReceiver<f64> {
    let command_enum = command.try_into()?;
    run::<IO>(command_enum)?;
    Ok(())
}

use crate::commands::*;
use crate::error::AppError;

pub mod commands;
pub mod error;
pub mod tests;

pub fn bin_selector(mut args: impl Iterator<Item = String>) -> Result<String, AppError> {
    match args.next() {
        Some(arg) if arg == "echo" => Ok(run_echo(args).join(" ")),
        Some(arg) if arg == "cat" => run_cat(args),
        Some(arg) if arg == "ls" => Ok(run_ls(args)?.join(" ")),
        Some(arg) if arg == "grep" => Ok(run_grep(args)?.join(" ")),
        Some(_) => Err(AppError::InvalidCommand),
        None => Err(AppError::NoArgumentsReceived),
    }
}

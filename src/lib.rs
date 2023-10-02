use std::error::Error;
use std::fmt;
use std::fs;

pub mod tests;

#[derive(Debug)]
pub enum AppError {
    MissingFilePath,
    FileReadError,
    InvalidCommand,
    NoArgumentsReceived,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::MissingFilePath => write!(f, "Didn't get a filepath"),
            AppError::FileReadError => write!(f, "File does not exist"),
            AppError::InvalidCommand => write!(f, "Invalid Command"),
            AppError::NoArgumentsReceived => write!(f, "No arguments received"),
        }
    }
}

impl Error for AppError {}

pub fn run_echo(mut args: impl Iterator<Item = String>) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();

    while let Some(arg) = args.next() {
        output.push(arg);
    }

    return output;
}

pub fn run_cat(mut args: impl Iterator<Item = String>) -> Result<String, AppError> {
    let file_path = match args.next() {
        Some(arg) => arg,
        None => return Err(AppError::MissingFilePath),
    };

    if let Ok(content) = fs::read_to_string(file_path) {
        return Ok(content);
    } else {
        return Err(AppError::FileReadError);
    };
}

pub fn bin_selector(mut args: impl Iterator<Item = String>) -> Result<String, AppError> {
    match args.next() {
        Some(arg) if arg == "echo" => Ok(run_echo(args).join(" ")),
        Some(arg) if arg == "cat" => run_cat(args),
        Some(_) => Err(AppError::InvalidCommand),
        None => Err(AppError::NoArgumentsReceived),
    }
}


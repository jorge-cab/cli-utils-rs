use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    FileReadError,
    InvalidCommand,
    NoArgumentsReceived,
    ReadDirectoryError,
    MissingArgument,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::FileReadError => write!(f, "File does not exist"),
            AppError::InvalidCommand => write!(f, "Invalid Command"),
            AppError::NoArgumentsReceived => write!(f, "No arguments received"),
            AppError::ReadDirectoryError => write!(f, "Error reading specified directory"),
            AppError::MissingArgument => write!(f, "1 or more arguments missing"),
        }
    }
}

impl Error for AppError {}

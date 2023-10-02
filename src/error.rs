use std::error::Error;
use std::fmt;

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

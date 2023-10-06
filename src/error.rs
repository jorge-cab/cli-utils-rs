use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    MissingPath,
    FileReadError,
    InvalidCommand,
    NoArgumentsReceived,
    ReadDirectoryError,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::MissingPath => write!(f, "Didn't get a filepath"),
            AppError::FileReadError => write!(f, "File does not exist"),
            AppError::InvalidCommand => write!(f, "Invalid Command"),
            AppError::NoArgumentsReceived => write!(f, "No arguments received"),
            AppError::ReadDirectoryError => write!(f, "Error reading specified directory"),
        }
    }
}

impl Error for AppError {}

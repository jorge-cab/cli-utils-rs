use crate::error::AppError;
use std::fs;

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

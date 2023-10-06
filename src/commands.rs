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
        None => return Err(AppError::MissingPath),
    };

    if let Ok(content) = fs::read_to_string(file_path) {
        return Ok(content);
    } else {
        return Err(AppError::FileReadError);
    };
}

pub fn run_ls(mut args: impl Iterator<Item = String>) -> Result<Vec<String>, AppError> {

    let dir_path = match args.next() {
        Some(arg) => arg,
        None => String::from("."),
    };

    let mut output = Vec::new();

    match fs::read_dir(dir_path) {
        Ok(contents) => {
            for elm in contents {
                match elm {
                    Ok(elm) => output.push(elm.file_name().to_str().unwrap().to_string()),
                    Err(_) => return Err(AppError::ReadDirectoryError),
                }
            }
        },
        Err(_) => return Err(AppError::ReadDirectoryError),
    };

    return Ok(output)
}


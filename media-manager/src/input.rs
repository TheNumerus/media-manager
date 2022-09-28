use crate::AppError;
use std::io::{stdin, ErrorKind};

pub fn ask_confirmation() -> Result<bool, AppError> {
    let mut buf = String::with_capacity(4);

    stdin()
        .read_line(&mut buf)
        .map_err(|e| AppError::Input("Error reading input".into(), e))?;

    match buf.trim() {
        "y" => Ok(true),
        "n" => Ok(false),
        _ => Err(AppError::Input(
            "Invalid input, write y/n".into(),
            ErrorKind::InvalidInput.into(),
        )),
    }
}

pub fn ask_confirmation_looped() -> Result<bool, AppError> {
    let mut buf = String::with_capacity(4);

    loop {
        stdin()
            .read_line(&mut buf)
            .map_err(|e| AppError::Input("Error reading input".into(), e))?;

        match buf.trim() {
            "y" => return Ok(true),
            "n" => return Ok(false),
            _ => {
                println!("Invalid input, write y/n");
                continue;
            }
        }
    }
}

pub fn read_line() -> Result<String, AppError> {
    let mut buf = String::new();
    stdin()
        .read_line(&mut buf)
        .map_err(|e| AppError::Input("Failed to read input".into(), e))?;

    // trim right in-place
    buf.truncate(buf.trim_end().len());

    Ok(buf)
}

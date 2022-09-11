use crate::args::RunMode;
use crate::command::Command;
use crate::error::AppError;
use directories::ProjectDirs;
use std::io::{stdin, ErrorKind};
use std::path::PathBuf;
use std::process::ExitCode;

mod args;
mod command;
mod config;
mod error;

const DEFAULT_DB_NAME: &str = "data.db";

fn main() -> ExitCode {
    if let Err(e) = run() {
        eprintln!("{e}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

fn run() -> Result<(), AppError> {
    let mode = RunMode::try_from(std::env::args())?;

    let db_path = get_db_path()?;
    std::fs::create_dir_all(&db_path.parent().unwrap())
        .map_err(|e| AppError::Input("Could not create folder for data".to_owned(), e))?;
    let db = libmm::db::Database::open(db_path)?;

    match mode {
        RunMode::SingleCommand(command) => command.execute(&db),
        RunMode::Interactive => loop {
            let command = match get_command() {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("{e}");
                    if let AppError::ArgsParse(_) = e {
                        continue;
                    } else {
                        break Ok(());
                    }
                }
            };

            if let Command::Exit = command {
                return Ok(());
            }
            command.execute(&db)?;
        },
    }
}

fn get_command() -> Result<Command, AppError> {
    let mut buf = String::new();
    let res = stdin().read_line(&mut buf);

    if let Err(e) = res {
        return Err(AppError::Input(
            "Failed to read command, use valid UTF-8".to_owned(),
            e,
        ));
    }

    let args = buf.split_whitespace().collect::<Vec<_>>();

    Command::try_from_arr(&args)
}

fn get_db_path() -> Result<PathBuf, AppError> {
    match ProjectDirs::from("", "", "media-manager") {
        None => Err(AppError::Input(
            "No home directory found".to_owned(),
            ErrorKind::Other.into(),
        )),
        Some(dirs) => {
            let mut root = dirs.data_dir().to_owned();
            root.push(DEFAULT_DB_NAME);
            Ok(root)
        }
    }
}

use crate::args::RunMode;
use crate::command::Command;
use crate::config::Config;
use crate::error::AppError;
use crate::paths::Paths;
use std::io::stdin;
use std::process::ExitCode;

mod args;
mod command;
mod config;
mod error;
mod paths;

fn main() -> ExitCode {
    if let Err(e) = run() {
        eprintln!("{e}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

fn run() -> Result<(), AppError> {
    let mode = RunMode::try_from(std::env::args())?;

    let paths = Paths::new()?;
    paths.make_dirs()?;

    let db = libmm::db::Database::open(paths.db_path)?;
    let config = Config::init(paths.config_path)?;

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

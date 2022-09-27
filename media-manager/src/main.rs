use crate::command::Command;
use crate::config::Config;
use crate::error::AppError;
use crate::paths::Paths;

use clap::Parser;
use std::process::ExitCode;

mod command;
mod config;
mod error;
mod input;
mod paths;

fn main() -> ExitCode {
    if let Err(e) = run() {
        eprintln!("{e}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

fn run() -> Result<(), AppError> {
    let args = <Args as Parser>::parse();

    let paths = Paths::new()?;
    paths.make_dirs()?;

    let db = libmm::db::Database::open(paths.db_path)?;
    let config = Config::init(paths.config_path)?;

    args.command.execute(&db, &config)?;

    Ok(())
}

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

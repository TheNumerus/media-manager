use crate::command::list_movies::ListMoviesCommand;
use crate::{AppError, Command};
use std::path::PathBuf;

pub struct CommandParser;

impl CommandParser {
    pub fn try_from_arr(args: &[impl AsRef<str>]) -> Result<Command, AppError> {
        let mut args = args.iter();

        let first = args
            .next()
            .ok_or(AppError::ArgsParse("Empty argument list".into()))?
            .as_ref();

        match first {
            "list-movies" => {
                let with_metadata = args
                    .next()
                    .map(|a| a.as_ref() == "--with-metadata")
                    .unwrap_or(false);
                Ok(Command::ListMovies(ListMoviesCommand { with_metadata }))
            }
            "add-movie" => {
                let path = args
                    .next()
                    .ok_or(AppError::ArgsParse("No path to movie provided".into()))?;
                Ok(Command::AddMovie(PathBuf::from(path.as_ref())))
            }
            "help" => Ok(Command::PrintHelp),
            "exit" => Ok(Command::Exit),
            _ => Err(AppError::ArgsParse("Unknown command".into())),
        }
    }
}

use crate::{AppError, Config};
use clap::Subcommand;
use libmm::db::Database;

mod add_movie;
mod list_movies;

use add_movie::AddMovieCommand;
use list_movies::ListMoviesCommand;

#[derive(Debug, Eq, PartialEq, Subcommand)]
pub enum Command {
    ListMovies(ListMoviesCommand),
    AddMovie(AddMovieCommand),
}

impl Command {
    pub fn execute(self, db: &Database, config: &Config) -> Result<(), AppError> {
        match self {
            Self::ListMovies(command) => command.execute(db),
            Self::AddMovie(command) => command.execute(db, config),
        }
    }
}

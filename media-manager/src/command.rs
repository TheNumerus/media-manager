use crate::{input, AppError, Config};
use libmm::api::TmdbClient;
use libmm::db::{Database, Insertable};
use libmm::media::{NameParser, ParsedName};
use std::io::ErrorKind;
use std::path::PathBuf;

mod list_movies;
mod parser;

use list_movies::ListMoviesCommand;
pub use parser::CommandParser;

#[derive(Debug, Eq, PartialEq)]
pub enum Command {
    PrintHelp,
    ListMovies(ListMoviesCommand),
    AddMovie(PathBuf),
    Exit,
}

impl Command {
    pub fn execute(self, db: &Database, config: &Config) -> Result<(), AppError> {
        match self {
            Self::PrintHelp => Self::print_help(),
            Self::ListMovies(command) => command.execute(db),
            Self::AddMovie(path) => Self::add_movie(db, config, path),
            Self::Exit => Ok(()),
        }
    }

    fn print_help() -> Result<(), AppError> {
        println!("help");
        Ok(())
    }

    fn add_movie(db: &Database, config: &Config, path: PathBuf) -> Result<(), AppError> {
        let ParsedName { title, year } = Self::name_from_path(path.clone())?;

        let title = if Self::ask_if_correct(&title, year) {
            title
        } else {
            input::read_line()?
        };

        let client = TmdbClient::new(config.tmdb_token.clone());

        let results = client.search_movies_by_title(title, year)?;

        for (i, movie) in results.iter().enumerate() {
            println!("[{}] {} ({})", i + 1, movie.title, movie.release_year);
        }

        println!("Please select movie by its index, or 0 if none is correct");

        let index = input::read_line()?
            .parse()
            .map_err(|_| AppError::invalid_input("Input is not a number"))?;

        match index {
            0 => println!("No movie was added."),
            i if i <= (results.len()) => {
                let res_index = i - 1;
                let detail = client
                    .get_movie_detail(results[res_index].tmdb_id)?
                    .ok_or(AppError::invalid_input("No movie was found"))?;
                let title = detail.title.clone();

                let movie = detail.complete(path);
                db.insert(movie)?;
                println!("Movie {} was added to db", title);
            }
            _ => println!("Invalid index given, no movie was added."),
        };

        Ok(())
    }

    fn ask_if_correct(title: &str, year: Option<usize>) -> bool {
        match year {
            Some(year) => {
                println!("Is this name correct? y/n: \"{title}\", release year: {year}");
            }
            None => {
                println!("Is this name correct? y/n: \"{title}\"");
            }
        }

        loop {
            let confirmation = input::ask_confirmation();

            match confirmation {
                Ok(c) => return c,
                Err(e) => {
                    println!("{e}");
                    continue;
                }
            }
        }
    }

    fn name_from_path(path: PathBuf) -> Result<ParsedName, AppError> {
        if !path.is_file() {
            // change to `ErrorKind::IsADirectory` after https://github.com/rust-lang/rust/issues/86442 stabilises
            return Err(AppError::Input(
                "Provided path is a not a file".into(),
                ErrorKind::InvalidInput.into(),
            ));
        }
        let filename = path
            .file_stem()
            .ok_or(AppError::invalid_input("Invalid filename"))?;

        Ok(NameParser::parse(filename.to_string_lossy()))
    }
}

use crate::AppError;
use libmm::db::movie::Movie;
use libmm::db::{Database, Selectable};
use std::io::ErrorKind;
use std::path::PathBuf;

#[derive(Debug, Eq, PartialEq)]
pub enum Command {
    PrintHelp,
    ListMovies,
    AddMovie(PathBuf),
    Exit,
}

impl Command {
    pub fn try_from_arr(args: &[impl AsRef<str>]) -> Result<Self, AppError> {
        let mut args = args.iter();

        let first = args
            .next()
            .ok_or(AppError::ArgsParse("Empty argument list".into()))?
            .as_ref();

        match first {
            "list-movies" => Ok(Self::ListMovies),
            "add-movie" => {
                let path = args
                    .next()
                    .ok_or(AppError::ArgsParse("No path to movie provided".into()))?;
                Ok(Self::AddMovie(PathBuf::from(path.as_ref())))
            }
            "help" => Ok(Self::PrintHelp),
            "exit" => Ok(Self::Exit),
            _ => Err(AppError::ArgsParse("Unknown command".into())),
        }
    }

    pub fn execute(self, db: &Database) -> Result<(), AppError> {
        match self {
            Self::PrintHelp => Self::print_help(),
            Self::ListMovies => Self::list_movies(db),
            Self::AddMovie(path) => Self::add_movie(db, path),
            Self::Exit => Ok(()),
        }
    }

    fn print_help() -> Result<(), AppError> {
        println!("help");
        Ok(())
    }

    fn list_movies(db: &Database) -> Result<(), AppError> {
        let movies = db.list_all()?;

        for movie in movies {
            let Movie {
                title,
                release_year,
                id,
                tmdb_id,
                ..
            } = movie;
            println!("[{id}/tmdb:{tmdb_id}] {title} ({release_year})");
        }

        Ok(())
    }

    fn add_movie(db: &Database, path: PathBuf) -> Result<(), AppError> {
        if !path.is_file() {
            // change to `ErrorKind::IsADirectory` after https://github.com/rust-lang/rust/issues/86442 stabilises
            return Err(AppError::Input(
                "Provided path is a not a file".into(),
                ErrorKind::InvalidInput.into(),
            ));
        }
        let filename = path.file_name().ok_or(AppError::Input(
            "Invalid filename".into(),
            ErrorKind::InvalidInput.into(),
        ))?;
        dbg!(filename);
        todo!()
    }
}

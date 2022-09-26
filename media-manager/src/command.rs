use crate::{input, AppError, Config};
use libmm::api::TmdbClient;
use libmm::db::movie::{LoadedMovie, Movie, NewMovie};
use libmm::db::{Database, Insertable, Selectable};
use libmm::media::{NameParser, ParsedName};
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

    pub fn execute(self, db: &Database, config: &Config) -> Result<(), AppError> {
        match self {
            Self::PrintHelp => Self::print_help(),
            Self::ListMovies => Self::list_movies(db),
            Self::AddMovie(path) => Self::add_movie(db, config, path),
            Self::Exit => Ok(()),
        }
    }

    fn print_help() -> Result<(), AppError> {
        println!("help");
        Ok(())
    }

    fn list_movies(db: &Database) -> Result<(), AppError> {
        let movies: Vec<LoadedMovie> = db.list_all()?;

        for movie in movies {
            let id = movie.id();
            let Movie {
                ref title,
                ref release_year,
                ref tmdb_id,
                ..
            } = movie;
            println!("[{id}/tmdb:{tmdb_id}] {title} ({release_year})");
        }

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

        let results = client.search_movies_by_title(title)?;

        for (i, movie) in results.iter().enumerate() {
            println!("[{}] {} ({})", i + 1, movie.title, movie.release_date);
            if let Some(overview) = &movie.overview {
                println!("\t{overview}");
            }
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
                    .get_movie_detail(results[res_index].id)?
                    .ok_or(AppError::invalid_input("No movie was found"))?;
                let title = detail.title.clone();

                let movie = NewMovie { path };
                db.insert(detail)?;
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

        let name_correct = loop {
            let confirmation = input::ask_confirmation();

            match confirmation {
                Ok(c) => break c,
                Err(e) => {
                    println!("{e}");
                    continue;
                }
            }
        };
        name_correct
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

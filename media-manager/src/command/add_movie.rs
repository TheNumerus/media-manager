use crate::{AppError, Config};
use clap::Args;
use libmm::api::TmdbClient;
use libmm::db::movie::CompleteMovie;
use libmm::db::{Database, Insertable};
use libmm::media::{MediaMetadata, NameParser, ParsedName};
use std::fs::File;
use std::io::ErrorKind;
use std::path::PathBuf;

/// Add movie to database
#[derive(Debug, Eq, PartialEq, Args)]
pub struct AddMovieCommand {
    /// Path to movie file
    path: PathBuf,
}

impl AddMovieCommand {
    pub fn execute(&self, db: &Database, config: &Config) -> Result<(), AppError> {
        let ParsedName { title, year } = name_from_path(self.path.clone())?;

        let title = if ask_if_correct(&title, year) {
            title
        } else {
            crate::input::read_line()?
        };

        let client = TmdbClient::new(config.tmdb_token.clone());

        let results = client.search_movies_by_title(title, year)?;

        for (i, movie) in results.iter().enumerate() {
            println!("[{}] {} ({})", i + 1, movie.title, movie.release_year);
        }

        let index = match get_index(results.len())? {
            MovieIndex::None => {
                println!("No movie was added.");
                return Ok(());
            }
            MovieIndex::Invalid => {
                println!("Invalid index given, no movie was added.");
                return Ok(());
            }
            MovieIndex::Valid(i) => i,
        };

        let detail = client
            .get_movie_detail(results[index].tmdb_id)?
            .ok_or(AppError::invalid_input("No movie was found"))?;

        let mut movie = detail.complete(self.path.clone());

        handle_alternate_cut(&mut movie, &self.path)?;

        let title = movie.title.clone();
        db.insert(movie)?;
        println!("Movie {} was added to db", title);

        Ok(())
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
        let confirmation = crate::input::ask_confirmation();

        match confirmation {
            Ok(c) => return c,
            Err(e) => {
                println!("{e}");
                continue;
            }
        }
    }
}

enum MovieIndex {
    None,
    Valid(usize),
    Invalid,
}

fn get_index(max: usize) -> Result<MovieIndex, AppError> {
    println!("Please select movie by its index, or 0 if none is correct");

    let index = crate::input::read_line()?
        .parse()
        .map_err(|_| AppError::invalid_input("Input is not a number"))?;

    Ok(match index {
        0 => MovieIndex::None,
        i if i <= max => MovieIndex::Valid(i - 1),
        _ => MovieIndex::Invalid,
    })
}

fn handle_alternate_cut(movie: &mut CompleteMovie, path: &PathBuf) -> Result<(), AppError> {
    let file = File::open(&path)
        .map_err(|e| AppError::Input("Could not open file for metadata info".into(), e))?;
    let metadata = MediaMetadata::from_file(file)?;
    let minutes = metadata.duration.as_secs() / 60;

    if minutes != *movie.original_runtime() as u64 {
        println!("File runtime ({} min) is different from TMDB runtime ({} min). It's possible that you have special cut of the movie. Is that correct y/n?", minutes, movie.original_runtime());
        if crate::input::ask_confirmation_looped()? {
            println!("Enter the name of alternate cut:");
            let cut_name = crate::input::read_line()?;

            *movie.cut_mut() = Some(cut_name);
        }
    }

    Ok(())
}

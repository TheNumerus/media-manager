use clap::Args;
use libmm::db::movie::LoadedMovie;
use libmm::db::{Database, Selectable};
use libmm::media::MediaMetadata;
use std::fs::File;

use crate::AppError;

#[derive(Debug, Eq, PartialEq, Args)]
/// List all movies in database
pub struct ListMoviesCommand {
    #[arg(long)]
    /// Print additional info about file
    pub with_metadata: bool,
}

impl ListMoviesCommand {
    pub fn execute(self, db: &Database) -> Result<(), AppError> {
        let movies: Vec<LoadedMovie> = db.list_all()?;

        for movie in movies {
            let id = movie.id();
            let path = movie.path();
            let LoadedMovie {
                ref title,
                ref release_year,
                ref tmdb_id,
                ..
            } = movie;
            println!("[{id}/tmdb:{tmdb_id}] {title} ({release_year})");

            if self.with_metadata {
                let file = File::open(path).map_err(|e| {
                    AppError::Input(
                        format!(
                            "Failed to opne file '{}' for metadata reading",
                            path.to_string_lossy()
                        ),
                        e,
                    )
                })?;
                let metadata = MediaMetadata::from_file(file)?;

                println!("    {:?}", metadata);
            }
        }

        Ok(())
    }
}

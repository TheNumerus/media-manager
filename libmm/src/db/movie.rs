use crate::db::{Creatable, Database, Insertable, Selectable};
use crate::error::Error;
use crate::{Complete, EntityState, Loaded};
use rusqlite::{params, OptionalExtension, Row};
use std::path::PathBuf;

#[derive(Debug)]
pub struct NewMovie {
    pub tmdb_id: usize,
    pub title: String,
    pub path: PathBuf,
    pub original_runtime: u32,
    pub release_year: u32,
}

#[derive(Debug)]
pub struct Movie<T: EntityState> {
    // are everywhere
    pub tmdb_id: usize,
    pub title: String,
    pub path: PathBuf,
    pub release_year: u32,
    // only on loaded
    id: Option<usize>,
    // on loaded + complete
    original_runtime: Option<u32>,
    _marker: std::marker::PhantomData<T>,
}

pub type CompleteMovie = Movie<Complete>;
pub type LoadedMovie = Movie<Loaded>;

impl Movie<Loaded> {
    pub fn id(&self) -> &usize {
        self.id.as_ref().unwrap()
    }
}

impl<T: EntityState> Creatable<Movie<T>> for Database {
    fn create_table_sql() -> &'static str {
        "CREATE TABLE `movie` (
            `id` INTEGER PRIMARY KEY,
            `tmdb_id` INTEGER,
            `title` TEXT,
            `path` TEXT,
            `original_runtime` INTEGER,
            `release_year` INTEGER
        );"
    }
}

impl Insertable<NewMovie> for Database {
    fn insert(&self, object: NewMovie) -> Result<usize, Error> {
        let NewMovie {
            tmdb_id,
            title,
            path,
            original_runtime,
            release_year,
            ..
        } = object;

        let mut stmt = self.conn.prepare(
            "INSERT INTO `movie` (tmdb_id, title, path, original_runtime, release_year) VALUES (?, ?, ?, ?, ?)",
        )?;

        stmt.execute(params![
            tmdb_id,
            title.as_str(),
            path.to_string_lossy(),
            original_runtime,
            release_year
        ])?;
        Ok(Database::last_insert_id(self)?)
    }
}

impl Selectable<LoadedMovie> for Database {
    fn select_by_id(&self, id: usize) -> Result<Option<LoadedMovie>, Error> {
        let mut stmt = self.conn.prepare("SELECT * FROM `movie` WHERE `id` = ?")?;

        Ok(stmt.query_row([id], movie_mapper).optional()?)
    }

    fn list_all(&self) -> Result<Vec<LoadedMovie>, Error> {
        let mut stmt = self.conn.prepare("SELECT * FROM `movie`")?;

        let mapped = stmt.query_map([], movie_mapper)?;

        let mut vec = Vec::new();
        for row in mapped {
            vec.push(row?);
        }

        Ok(vec)
    }
}

fn movie_mapper(row: &Row) -> Result<LoadedMovie, rusqlite::Error> {
    Ok(Movie {
        id: row.get(0)?,
        tmdb_id: row.get(1)?,
        title: row.get(2)?,
        path: PathBuf::from(row.get::<usize, String>(3)?),
        original_runtime: row.get(4)?,
        release_year: row.get(5)?,
        _marker: std::marker::PhantomData::default(),
    })
}

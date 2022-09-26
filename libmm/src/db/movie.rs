use crate::db::{Creatable, Database, Insertable, Selectable};
use crate::error::Error;
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
pub struct Movie {
    pub id: usize,
    pub tmdb_id: usize,
    pub title: String,
    pub path: PathBuf,
    pub original_runtime: u32,
    pub release_year: u32,
}

impl Creatable<Movie> for Database {
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

impl Selectable<Movie> for Database {
    fn select_by_id(&self, id: usize) -> Result<Option<Movie>, Error> {
        let mut stmt = self.conn.prepare("SELECT * FROM `movie` WHERE `id` = ?")?;

        Ok(stmt.query_row([id], movie_mapper).optional()?)
    }

    fn list_all(&self) -> Result<Vec<Movie>, Error> {
        let mut stmt = self.conn.prepare("SELECT * FROM `movie`")?;

        let mapped = stmt.query_map([], movie_mapper)?;

        let mut vec = Vec::new();
        for row in mapped {
            vec.push(row?);
        }

        Ok(vec)
    }
}

fn movie_mapper(row: &Row) -> Result<Movie, rusqlite::Error> {
    Ok(Movie {
        id: row.get(0)?,
        tmdb_id: row.get(1)?,
        title: row.get(2)?,
        path: PathBuf::from(row.get::<usize, String>(3)?),
        original_runtime: row.get(4)?,
        release_year: row.get(5)?,
    })
}

use crate::db::{Creatable, Database, Insertable, Selectable};
use crate::error::Error;
use crate::{Complete, EntityState, Incomplete, Loaded};
use rusqlite::{params, OptionalExtension, Row};
use std::fmt::{Debug, Formatter};
use std::path::PathBuf;

pub struct Movie<T: EntityState> {
    // are everywhere
    pub tmdb_id: usize,
    pub title: String,
    pub release_year: u32,
    // on loaded + complete
    path: Option<PathBuf>,
    original_runtime: Option<u32>, // might be on incomplete
    cut: Option<String>,
    // only on loaded
    id: Option<usize>,

    _marker: std::marker::PhantomData<T>,
}

pub type IncompleteMovie = Movie<Incomplete>;
pub type CompleteMovie = Movie<Complete>;
pub type LoadedMovie = Movie<Loaded>;

impl<T: EntityState> Debug for Movie<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Movie")
            .field("id", &self.id)
            .field("tmdb_id", &self.tmdb_id)
            .field("title", &self.title)
            .field("release_year", &self.title)
            .field("original_runtime", &self.original_runtime)
            .field("cut", &self.cut)
            .field("path", &self.path)
            .finish()
    }
}

impl Movie<Incomplete> {
    pub fn new(tmdb_id: usize, title: String, release_year: u32) -> Self {
        Self {
            tmdb_id,
            title,
            release_year,
            cut: None,
            id: None,
            path: None,
            original_runtime: None,
            _marker: std::marker::PhantomData::default(),
        }
    }

    pub fn set_runtime(&mut self, runtime: Option<u32>) -> &mut Self {
        self.original_runtime = runtime;
        self
    }

    pub fn complete(self, path: PathBuf) -> Movie<Complete> {
        // change after https://github.com/rust-lang/rust/issues/86555 stabilises
        Movie {
            path: Some(path),
            tmdb_id: self.tmdb_id,
            title: self.title,
            release_year: self.release_year,
            original_runtime: self.original_runtime,
            cut: self.cut,
            id: None,
            _marker: std::marker::PhantomData::default(),
        }
    }
}

impl Movie<Complete> {
    pub fn path(&self) -> &PathBuf {
        self.path.as_ref().unwrap()
    }

    pub fn original_runtime(&self) -> &u32 {
        self.original_runtime.as_ref().unwrap()
    }

    pub fn cut(&self) -> &Option<String> {
        &self.cut
    }

    pub fn cut_mut(&mut self) -> &mut Option<String> {
        &mut self.cut
    }
}

impl Movie<Loaded> {
    pub fn id(&self) -> &usize {
        self.id.as_ref().unwrap()
    }

    pub fn path(&self) -> &PathBuf {
        self.path.as_ref().unwrap()
    }

    pub fn original_runtime(&self) -> &u32 {
        self.original_runtime.as_ref().unwrap()
    }

    pub fn cut(&self) -> &Option<String> {
        &self.cut
    }
}

impl<T: EntityState> Creatable<Movie<T>> for Database {
    fn create_table_sql() -> &'static str {
        "CREATE TABLE IF NOT EXISTS `movie` (
            `id` INTEGER PRIMARY KEY,
            `tmdb_id` INTEGER,
            `title` TEXT,
            `cut` TEXT,
            `path` TEXT,
            `original_runtime` INTEGER,
            `release_year` INTEGER
        );"
    }
}

impl Insertable<CompleteMovie> for Database {
    fn insert(&self, object: CompleteMovie) -> Result<usize, Error> {
        let Movie {
            tmdb_id,
            title,
            cut,
            path,
            original_runtime,
            release_year,
            ..
        } = object;

        let mut stmt = self.conn.prepare(
            "INSERT INTO `movie` (tmdb_id, title, cut, path, original_runtime, release_year) VALUES (?, ?, ?, ?, ?, ?)",
        )?;

        stmt.execute(params![
            tmdb_id,
            title.as_str(),
            cut,
            path.unwrap().to_string_lossy(),
            original_runtime,
            release_year
        ])?;

        Database::last_insert_id(self)
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
        cut: row.get(3)?,
        path: Some(PathBuf::from(row.get::<usize, String>(4)?)),
        original_runtime: row.get(5)?,
        release_year: row.get(6)?,
        _marker: std::marker::PhantomData::default(),
    })
}

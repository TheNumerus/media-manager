use crate::db::{Creatable, Database, Insertable, Selectable};
use crate::error::Error;
use crate::{Complete, EntityState, Incomplete, Loaded};
use rusqlite::{params, OptionalExtension, Row};
use std::fmt::{Debug, Formatter};

pub struct TvShow<T: EntityState> {
    // are everywhere
    pub tvmaze_id: usize,
    pub title: String,
    // only on loaded
    id: Option<usize>,

    _marker: std::marker::PhantomData<T>,
}

pub type IncompleteTvShow = TvShow<Incomplete>;
pub type CompleteTvShow = TvShow<Complete>;
pub type LoadedTvShow = TvShow<Loaded>;

impl TvShow<Incomplete> {
    pub fn new(tvmaze_id: usize, title: String) -> Self {
        Self {
            tvmaze_id,
            title,
            id: None,
            _marker: std::marker::PhantomData::default(),
        }
    }

    pub fn complete(self) -> TvShow<Complete> {
        // change after https://github.com/rust-lang/rust/issues/86555 stabilises
        TvShow {
            tvmaze_id: self.tvmaze_id,
            title: self.title,
            id: None,
            _marker: std::marker::PhantomData::default(),
        }
    }
}

impl<T: EntityState> Debug for TvShow<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TvShow")
            .field("id", &self.id)
            .field("tvmaze_id", &self.tvmaze_id)
            .field("title", &self.title)
            .finish()
    }
}

impl<T: EntityState> Creatable<TvShow<T>> for Database {
    fn create_table_sql() -> &'static str {
        "CREATE TABLE IF NOT EXISTS `tvshow` (
            `id` INTEGER PRIMARY KEY,
            `tvmaze_id` INTEGER,
            `title` TEXT
        );"
    }
}

impl Insertable<TvShow<Complete>> for Database {
    fn insert(&self, object: TvShow<Complete>) -> Result<usize, Error> {
        let TvShow {
            tvmaze_id, title, ..
        } = object;

        let mut stmt = self
            .conn
            .prepare("INSERT INTO `tvshow` (tvmaze_id, title) VALUES (?, ?)")?;

        stmt.execute(params![tvmaze_id, title.as_str(),])?;

        Database::last_insert_id(self)
    }
}

impl Selectable<TvShow<Loaded>> for Database {
    fn select_by_id(&self, id: usize) -> Result<Option<TvShow<Loaded>>, Error> {
        let mut stmt = self.conn.prepare("SELECT * FROM `tvshow` WHERE `id` = ?")?;

        Ok(stmt.query_row([id], tvshow_mapper).optional()?)
    }

    fn list_all(&self) -> Result<Vec<TvShow<Loaded>>, Error> {
        let mut stmt = self.conn.prepare("SELECT * FROM `tvshow`")?;

        let mapped = stmt.query_map([], tvshow_mapper)?;

        let mut vec = Vec::new();
        for row in mapped {
            vec.push(row?);
        }

        Ok(vec)
    }
}

fn tvshow_mapper(row: &Row) -> Result<LoadedTvShow, rusqlite::Error> {
    Ok(TvShow {
        id: row.get(0)?,
        tvmaze_id: row.get(1)?,
        title: row.get(2)?,
        _marker: std::marker::PhantomData::default(),
    })
}

use crate::db::movie::Movie;
use crate::error::{DbError, Error};
use rusqlite::{Connection, Row};
use std::path::Path;

pub mod movie;
pub mod tvshow;

#[derive(Debug)]
pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let conn = Connection::open(path)?;

        if Self::is_conn_empty(&conn)? {
            Self::init(&conn)?;
        }

        Ok(Self { conn })
    }

    pub(crate) fn last_insert_id(&self) -> Result<usize, Error> {
        self.conn
            .query_row("SELECT last_insert_rowid();", [], Self::get_first_row)
            .map_err(|e| e.into())
    }

    fn init(conn: &Connection) -> Result<(), Error> {
        conn.execute_batch(<Database as Creatable<Movie>>::create_table_sql())?;

        Ok(())
    }

    fn is_conn_empty(conn: &Connection) -> Result<bool, DbError> {
        let mut stmt = conn
            .prepare("SELECT COUNT(name) FROM sqlite_schema WHERE type='table' ORDER BY name;")?;

        Ok(stmt.query_row([], Self::get_first_row)? == 0)
    }

    fn get_first_row(row: &Row) -> Result<usize, rusqlite::Error> {
        row.get(0)
    }
}

pub trait Creatable<T> {
    fn create_table_sql() -> &'static str;
}

pub trait Insertable<T> {
    fn insert(&self, object: T) -> Result<usize, Error>;
}

pub trait Selectable<T> {
    fn select_by_id(&self, id: usize) -> Result<Option<T>, Error>;
    fn list_all(&self) -> Result<Vec<T>, Error>;
}

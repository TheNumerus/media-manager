use crate::error::{DbError, Error};
use rusqlite::Connection;
use std::path::Path;

pub mod entity;

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

    fn init(conn: &Connection) -> Result<(), Error> {
        let mut stmt = conn.execute_batch("CREATE TABLE;");
        todo!()
    }

    fn is_conn_empty(conn: &Connection) -> Result<bool, DbError> {
        let mut stmt =
            conn.prepare("SELECT name FROM sqlite_schema WHERE type='table' ORDER BY name;")?;

        Ok(stmt.execute([])? != 0)
    }
}

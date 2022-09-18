use crate::db::{Creatable, Database, Insertable, Selectable};
use crate::error::Error;

pub struct TvShow {}

impl Creatable<TvShow> for Database {
    fn create_table_sql() -> &'static str {
        ""
    }
}

impl Selectable<TvShow> for Database {
    fn select_by_id(&self, id: usize) -> Result<Option<TvShow>, Error> {
        todo!()
    }

    fn list_all(&self) -> Result<Vec<TvShow>, Error> {
        todo!()
    }
}

impl Insertable<TvShow> for Database {
    fn insert(&self, object: TvShow) -> Result<usize, Error> {
        todo!()
    }
}

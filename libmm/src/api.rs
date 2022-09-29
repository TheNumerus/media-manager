use std::fmt::Display;

mod tmdb;
mod tvmaze;

pub use tmdb::TmdbClient;
pub use tvmaze::TvMazeClient;

struct QueryBuilder<'a> {
    keys: Vec<String>,
    values: Vec<Box<dyn Display + 'a>>,
}

impl<'a> QueryBuilder<'a> {
    pub fn new() -> Self {
        Self {
            keys: Vec::new(),
            values: Vec::new(),
        }
    }

    pub fn add(mut self, k: impl Into<String>, v: impl Display + 'a) -> Self {
        self.keys.push(k.into());
        self.values.push(Box::new(v));

        self
    }

    pub fn build(self) -> String {
        if self.keys.is_empty() {
            String::new()
        } else {
            let mut query = String::from("?");

            let pairs = self
                .keys
                .into_iter()
                .zip(self.values)
                .map(|(k, v)| format!("{k}={v}"))
                .collect::<Vec<_>>()
                .join("&");
            query.push_str(&pairs);

            query
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_query() {
        let query = QueryBuilder::new()
            .add(String::from("test"), 1)
            .add(String::from("test2"), "aaaaa")
            .build();

        assert_eq!(String::from("?test=1&test2=aaaaa"), query);
    }
}

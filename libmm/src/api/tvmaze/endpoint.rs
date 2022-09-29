use crate::api::QueryBuilder;
use std::fmt::Display;

const BASE_URL: &str = "https://api.tvmaze.com";

pub enum TvMazeEndpoint<'a> {
    SearchTvShow { query: &'a str },
}

impl<'a> TvMazeEndpoint<'a> {
    pub fn url(&'a self) -> String {
        match self {
            TvMazeEndpoint::SearchTvShow { query } => {
                build_url("/search/shows", QueryBuilder::new().add("q", query).build())
            }
        }
    }
}

fn build_url(path: impl Display, query: String) -> String {
    format!("{BASE_URL}{path}{query}")
}

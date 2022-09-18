use crate::db::movie::NewMovie;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct MovieDetail {
    pub id: usize,
    pub title: String,
    pub original_title: String,
    pub original_language: String,
    pub overview: Option<String>,
    pub release_date: String,
    pub runtime: u32,
}

impl From<MovieDetail> for NewMovie {
    fn from(md: MovieDetail) -> Self {
        Self {
            tmdb_id: md.id,
            title: md.title,
            original_runtime: md.runtime,
            release_year: convert_year(md.release_date),
        }
    }
}

fn convert_year(date: String) -> u32 {
    date.split('-')
        .nth(0)
        .expect("Invalid date format")
        .parse()
        .expect("Invalid number")
}

#[derive(Deserialize, Default)]
pub(crate) struct SearchMovieResponse {
    #[allow(dead_code)]
    pub page: usize,
    pub results: Vec<SearchedMovie>,
    #[allow(dead_code)]
    pub total_pages: usize,
    #[allow(dead_code)]
    pub total_results: usize,
}

#[derive(Deserialize, Debug)]
pub struct SearchedMovie {
    pub id: usize,
    pub title: String,
    pub original_title: String,
    pub original_language: String,
    pub overview: Option<String>,
    pub release_date: String,
}

#[derive(Deserialize, Debug)]
pub struct ErrorInfo {
    pub status_code: usize,
    pub status_message: String,
}

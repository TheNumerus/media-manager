use crate::db::movie::IncompleteMovie;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub(crate) struct MovieDetail {
    pub id: usize,
    pub title: String,
    #[allow(dead_code)]
    pub original_title: String,
    #[allow(dead_code)]
    pub original_language: String,
    #[allow(dead_code)]
    pub overview: Option<String>,
    pub release_date: String,
    pub runtime: u32,
}

impl From<MovieDetail> for IncompleteMovie {
    fn from(md: MovieDetail) -> Self {
        let mut movie = IncompleteMovie::new(md.id, md.title, convert_year(md.release_date));

        if md.runtime != 0 {
            movie.set_runtime(Some(md.runtime));
        }

        movie
    }
}

/// Convert from yyyy-mm-dd to year integer
fn convert_year(date: String) -> u32 {
    date.split('-')
        .next()
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
pub(crate) struct SearchedMovie {
    pub id: usize,
    pub title: String,
    #[allow(dead_code)]
    pub original_title: String,
    #[allow(dead_code)]
    pub original_language: String,
    #[allow(dead_code)]
    pub overview: Option<String>,
    pub release_date: String,
}

impl From<SearchedMovie> for IncompleteMovie {
    fn from(sm: SearchedMovie) -> Self {
        IncompleteMovie::new(sm.id, sm.title, convert_year(sm.release_date))
    }
}

#[derive(Deserialize, Debug)]
pub(crate) struct ErrorInfo {
    #[allow(dead_code)]
    pub status_code: usize,
    pub status_message: String,
}

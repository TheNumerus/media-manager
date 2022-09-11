use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct MovieDetail {
    pub id: usize,
    pub title: String,
    pub original_title: String,
    pub original_language: String,
    pub overview: Option<String>,
    pub release_date: String,
    pub runtime: usize,
}

#[derive(Deserialize, Debug)]
pub struct ErrorInfo {
    pub status_code: usize,
    pub status_message: String,
}

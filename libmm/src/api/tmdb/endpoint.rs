use std::fmt::Display;

const BASE_URL: &str = "https://api.themoviedb.org/3";

pub enum TmdbEndpoint<'a> {
    GetMovieDetail { movie_id: usize },
    SearchMovies { query: &'a str, year: Option<usize> },
}

impl<'a> TmdbEndpoint<'a> {
    pub fn url(&'a self, api_key: &'a str) -> String {
        match self {
            Self::GetMovieDetail { movie_id } => Self::build_url(
                format!("/movie/{movie_id}"),
                Self::build_query(&[("api_key", api_key)]),
            ),
            Self::SearchMovies { query, year } => {
                let query = match year {
                    Some(y) => Self::build_query(&[
                        ("api_key", api_key),
                        ("query", query),
                        ("primary_release_year", &y.to_string()),
                    ]),
                    None => Self::build_query(&[("api_key", api_key), ("query", query)]),
                };
                Self::build_url("/search/movie", query)
            }
        }
    }

    fn build_url(path: impl AsRef<str> + Display, query: String) -> String {
        format!("{BASE_URL}{path}?{query}")
    }

    fn build_query(params: &[(&'static str, impl AsRef<str> + Display)]) -> String {
        params
            .iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect::<Vec<_>>()
            .join("&")
    }
}

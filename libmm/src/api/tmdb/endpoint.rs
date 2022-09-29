use crate::api::QueryBuilder;
use std::fmt::Display;

const BASE_URL: &str = "https://api.themoviedb.org/3";

pub enum TmdbEndpoint<'a> {
    GetMovieDetail { movie_id: usize },
    SearchMovies { query: &'a str, year: Option<usize> },
}

impl<'a> TmdbEndpoint<'a> {
    pub fn url(&'a self, api_key: &'a str) -> String {
        match self {
            Self::GetMovieDetail { movie_id } => build_url(
                format!("/movie/{movie_id}"),
                QueryBuilder::new().add("api_key", api_key).build(),
            ),
            Self::SearchMovies { query, year } => {
                let mut builder = QueryBuilder::new()
                    .add("api_key", api_key)
                    .add("query", query);

                if let Some(year) = year {
                    builder = builder.add("primary_release_year", year);
                }

                let query = builder.build();
                build_url("/search/movie", query)
            }
        }
    }
}

fn build_url(path: impl Display, query: String) -> String {
    format!("{BASE_URL}{path}{query}")
}

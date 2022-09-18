const BASE_URL: &str = "https://api.themoviedb.org/3";

pub enum TmdbEndpoint<'a> {
    GetMovieDetail { movie_id: usize },
    SearchMovies { query: &'a str },
}

impl<'a> TmdbEndpoint<'a> {
    pub fn url(&'a self, api_key: &'a str) -> String {
        match self {
            Self::GetMovieDetail { movie_id } => {
                format!("{BASE_URL}/movie/{movie_id}?api_key={api_key}")
            },
            Self::SearchMovies { query} => {
                format!("{BASE_URL}/search/movie?api_key={api_key}&query={query}")
            }
        }
    }
}

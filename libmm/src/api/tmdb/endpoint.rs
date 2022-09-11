const BASE_URL: &str = "https://api.themoviedb.org/3";

pub enum TmdbEndpoint {
    GetMovieDetail { movie_id: usize },
}

impl TmdbEndpoint {
    pub fn url(&self, api_key: &str) -> String {
        match self {
            TmdbEndpoint::GetMovieDetail { movie_id } => {
                format!("{BASE_URL}/movie/{movie_id}?api_key={api_key}")
            }
        }
    }
}

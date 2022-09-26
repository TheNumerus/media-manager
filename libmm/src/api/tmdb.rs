use crate::api::tmdb::endpoint::TmdbEndpoint;
use crate::api::tmdb::response::{ErrorInfo, MovieDetail, SearchMovieResponse};
use crate::error::{ApiError, Error};

use crate::db::movie::IncompleteMovie;
use ureq::Agent;

mod endpoint;
mod response;

/// Client for The Movie Database (TMDB) API
pub struct TmdbClient {
    api_key: String,
    agent: Agent,
}

impl TmdbClient {
    pub fn new(api_key: String) -> Self {
        let agent = ureq::agent();
        Self { api_key, agent }
    }

    pub fn get_movie_detail(&self, movie_id: usize) -> Result<Option<IncompleteMovie>, Error> {
        let url = TmdbEndpoint::GetMovieDetail { movie_id }.url(&self.api_key);

        let res = self.agent.get(&url).call();

        match res {
            Ok(res) => {
                let res = res.into_json::<MovieDetail>();
                match res {
                    Ok(md) => Ok(Some(md.into())),
                    Err(_e) => Err(ApiError::InvalidFormat.into()),
                }
            }
            Err(ureq::Error::Status(401, _)) => Err(ApiError::ApiKey.into()),
            Err(ureq::Error::Status(404, _)) => Ok(None),
            Err(ureq::Error::Status(_status, res)) => {
                let res = res.into_json::<ErrorInfo>();
                match res {
                    Ok(e) => Err(ApiError::Unknown(e.status_message).into()),
                    Err(_e) => Err(ApiError::InvalidFormat.into()),
                }
            }
            Err(ureq::Error::Transport(t)) => Err(ApiError::Transport(t).into()),
        }
    }

    /// Returns list of movies.
    ///
    /// ## Remarks
    /// Movies are without runtime length.
    pub fn search_movies_by_title(
        &self,
        title: impl AsRef<str>,
    ) -> Result<Vec<IncompleteMovie>, Error> {
        let url = TmdbEndpoint::SearchMovies {
            query: title.as_ref(),
        }
        .url(&self.api_key);

        let res = self.agent.get(&url).call();

        match res {
            Ok(res) => {
                let res = res.into_json::<SearchMovieResponse>();
                match res {
                    Ok(response) => Ok(response.results.into_iter().map(|m| m.into()).collect()),
                    Err(_e) => Err(ApiError::InvalidFormat.into()),
                }
            }
            Err(ureq::Error::Status(401, _)) => Err(ApiError::ApiKey.into()),
            Err(ureq::Error::Status(404, _)) => Ok(Vec::new()),
            Err(ureq::Error::Status(_status, res)) => {
                let res = res.into_json::<ErrorInfo>();
                match res {
                    Ok(e) => Err(ApiError::Unknown(e.status_message).into()),
                    Err(_e) => Err(ApiError::InvalidFormat.into()),
                }
            }
            Err(ureq::Error::Transport(t)) => Err(ApiError::Transport(t).into()),
        }
    }
}

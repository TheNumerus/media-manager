use crate::api::tmdb::endpoint::TmdbEndpoint;
use crate::api::tmdb::response::{ErrorInfo, MovieDetail};
use crate::error::{ApiError, Error};

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

    pub fn get_movie_detail(&self, movie_id: usize) -> Result<Option<MovieDetail>, Error> {
        let url = TmdbEndpoint::GetMovieDetail { movie_id }.url(&self.api_key);

        let res = self.agent.get(&url).call();

        match res {
            Ok(res) => {
                let res = res.into_json::<MovieDetail>();
                match res {
                    Ok(md) => Ok(Some(md)),
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
}

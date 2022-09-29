use crate::api::tvmaze::endpoint::TvMazeEndpoint;
use crate::api::tvmaze::response::SearchTvShowResponse;
use crate::db::tvshow::IncompleteTvShow;
use crate::error::{ApiError, Error};

use ureq::Agent;

mod endpoint;
mod response;

pub struct TvMazeClient {
    agent: Agent,
}

impl TvMazeClient {
    pub fn new() -> Self {
        Self {
            agent: Agent::new(),
        }
    }

    pub fn search_tvshows_by_title(
        &self,
        title: impl AsRef<str>,
    ) -> Result<Vec<IncompleteTvShow>, Error> {
        let url = TvMazeEndpoint::SearchTvShow {
            query: title.as_ref(),
        }
        .url();

        let res = self.agent.get(&url).call();

        match res {
            Ok(res) => {
                let vec = res.into_json::<Vec<SearchTvShowResponse>>();
                match vec {
                    Ok(vec) => Ok(vec.into_iter().map(|s| s.show.into()).collect()),
                    Err(_e) => Err(ApiError::InvalidFormat.into()),
                }
            }
            Err(ureq::Error::Status(_status, res)) => {
                let res = res.into_string();
                match res {
                    Ok(e) => Err(ApiError::Unknown(e).into()),
                    Err(_e) => Err(ApiError::InvalidFormat.into()),
                }
            }
            Err(ureq::Error::Transport(t)) => Err(ApiError::Transport(t).into()),
        }
    }
}

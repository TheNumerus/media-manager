use crate::db::tvshow::IncompleteTvShow;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub(crate) struct SearchTvShowResponse {
    pub(crate) show: SearchTvShowPartial,
}

#[derive(Deserialize, Debug)]
pub(crate) struct SearchTvShowPartial {
    id: usize,
    name: String,
    premiered: String,
    ended: Option<String>,
}

impl From<SearchTvShowPartial> for IncompleteTvShow {
    fn from(tvshow: SearchTvShowPartial) -> Self {
        IncompleteTvShow::new(tvshow.id, tvshow.name)
    }
}

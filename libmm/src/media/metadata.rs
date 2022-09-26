use crate::error::{Error, MediaError};
use matroska::Matroska;
use std::fs::File;
use std::time::Duration;

pub struct MediaMetadata {
    pub duration: Option<Duration>,
}

impl MediaMetadata {
    pub fn from_file(file: File) -> Result<Self, Error> {
        let matroska = Matroska::open(file).map_err(|e| MediaError::Matroska(e))?;

        Ok(Self {
            duration: matroska.info.duration,
        })
    }
}

use crate::error::{Error, MediaError};
use matroska::{Matroska, Track};
use std::fs::File;
use std::time::Duration;

#[derive(Debug)]
pub struct MediaMetadata {
    pub duration: Option<Duration>,
    pub video_codec: String,
    pub audio_tracks: Vec<String>,
}

impl MediaMetadata {
    pub fn from_file(file: File) -> Result<Self, Error> {
        let matroska = Matroska::open(file).map_err(|e| MediaError::Matroska(e))?;

        let duration = matroska.info.duration;

        let video_codec = match matroska.video_tracks().nth(0) {
            Some(track) => Self::parse_video_codec(track),
            None => return Err(MediaError::NoVideoTrack.into()),
        };

        let audio_tracks = matroska.audio_tracks().map(Self::map_audio_track).collect();

        Ok(Self {
            duration,
            video_codec,
            audio_tracks,
        })
    }

    fn parse_video_codec(track: &Track) -> String {
        if track.codec_id.contains("HEVC") {
            String::from("HEVC / h265")
        } else if track.codec_id.contains("AVC") {
            String::from("AVC / h264")
        } else {
            track.codec_id.clone()
        }
    }

    fn map_audio_track(item: &Track) -> String {
        let name = item.name.as_ref();
        let lang = item.language.as_ref().map(lang_to_string);

        match (name, lang) {
            (Some(name), Some(lang)) => {
                format!("{name} ({lang})")
            }
            (None, Some(lang)) => format!("Audio track ({lang})"),
            (Some(name), None) => format!("{name}"),
            (None, None) => format!("Audio track"),
        }
    }
}

fn lang_to_string(lang: &matroska::Language) -> &String {
    match lang {
        matroska::Language::ISO639(s) => s,
        matroska::Language::IETF(s) => s,
    }
}

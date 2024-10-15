use std::collections::HashMap;

use super::{MetadataValue, TrackID};
use crate::MprisDuration;

#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Metadata {
    pub album_artists: Option<Vec<String>>,
    pub album_name: Option<String>,
    pub art_url: Option<String>,
    pub artists: Option<Vec<String>>,
    pub audio_bpm: Option<u64>,
    pub auto_rating: Option<f64>,
    pub comments: Option<Vec<String>>,
    pub composers: Option<Vec<String>>,
    pub content_created: Option<String>,
    pub disc_number: Option<u64>,
    pub first_used: Option<String>,
    pub genres: Option<Vec<String>>,
    pub last_used: Option<String>,
    pub length: Option<MprisDuration>,
    pub lyricists: Option<Vec<String>>,
    pub lyrics: Option<String>,
    pub title: Option<String>,
    pub track_id: Option<TrackID>,
    pub track_number: Option<u64>,
    pub url: Option<String>,
    pub use_count: Option<u64>,
    pub user_rating: Option<f64>,
}

impl Metadata {
    pub fn is_empty(&self) -> bool {
        self.album_artists.is_none()
            && self.album_name.is_none()
            && self.art_url.is_none()
            && self.artists.is_none()
            && self.audio_bpm.is_none()
            && self.auto_rating.is_none()
            && self.comments.is_none()
            && self.composers.is_none()
            && self.content_created.is_none()
            && self.disc_number.is_none()
            && self.first_used.is_none()
            && self.genres.is_none()
            && self.last_used.is_none()
            && self.length.is_none()
            && self.lyricists.is_none()
            && self.lyrics.is_none()
            && self.title.is_none()
            && self.track_id.is_none()
            && self.track_number.is_none()
            && self.url.is_none()
            && self.url.is_none()
            && self.user_rating.is_none()
    }
}

macro_rules! extract {
    ($hash:ident, $key:expr, $f:expr) => {
        extract(&mut $hash, $key, $f)
    };
}

fn extract<T, F>(raw: &mut HashMap<String, MetadataValue>, key: &str, f: F) -> Option<T>
where
    F: FnOnce(MetadataValue) -> Option<T>,
{
    raw.remove(key).and_then(f)
}

impl From<HashMap<String, MetadataValue>> for Metadata {
    fn from(mut raw: HashMap<String, MetadataValue>) -> Self {
        Metadata {
            album_artists: extract!(raw, "xesam:albumArtist", MetadataValue::into_strings),
            album_name: extract!(raw, "xesam:album", MetadataValue::into_nonempty_string),
            art_url: extract!(raw, "mpris:artUrl", MetadataValue::into_nonempty_string),
            artists: extract!(raw, "xesam:artist", MetadataValue::into_strings),
            audio_bpm: extract!(raw, "xesam:audioBPM", MetadataValue::into_u64),
            auto_rating: extract!(raw, "xesam:autoRating", MetadataValue::into_float),
            comments: extract!(raw, "xesam:comment", MetadataValue::into_strings),
            composers: extract!(raw, "xesam:composer", MetadataValue::into_strings),
            content_created: extract!(raw, "xesam:contentCreated", MetadataValue::into_string),
            disc_number: extract!(raw, "xesam:discNumber", MetadataValue::into_u64),
            first_used: extract!(raw, "xesam:firstUsed", MetadataValue::into_string),
            genres: extract!(raw, "xesam:genre", MetadataValue::into_strings),
            last_used: extract!(raw, "xesam:lastUsed", MetadataValue::into_string),
            length: extract!(raw, "mpris:length", |v| MprisDuration::try_from(v).ok()),
            lyricists: extract!(raw, "xesam:lyricist", MetadataValue::into_strings),
            lyrics: extract!(raw, "xesam:asText", MetadataValue::into_string),
            title: extract!(raw, "xesam:title", MetadataValue::into_nonempty_string),
            track_id: extract!(raw, "mpris:trackid", |v| TrackID::try_from(v).ok()),
            track_number: extract!(raw, "xesam:trackNumber", MetadataValue::into_u64),
            url: extract!(raw, "xesam:url", MetadataValue::into_nonempty_string),
            use_count: extract!(raw, "xesam:useCount", MetadataValue::into_u64),
            user_rating: extract!(raw, "xesam:userRating", MetadataValue::into_float),
        }
    }
}

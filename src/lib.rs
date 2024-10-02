use std::fmt::Display;

use zbus::Connection;

mod metadata;
mod player;
mod proxies;

use metadata::InvalidTrackID;

pub use metadata::{Metadata, TrackID};
pub use player::Player;

pub struct Mpris {
    connection: Connection,
}

impl Mpris {
    pub async fn new() -> Result<Self, MprisError> {
        let connection = Connection::session().await?;
        Ok(Self { connection })
    }

    pub async fn players(&self) -> Result<Vec<Player>, MprisError> {
        player::all(&self.connection).await
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
/// The [`Player`]'s playback status
///
/// See: [MPRIS2 specification about `PlaybackStatus`][playback_status]
///
/// [playback_status]: https://specifications.freedesktop.org/mpris-spec/latest/Player_Interface.html#Enum:Playback_Status
pub enum PlaybackStatus {
    /// A track is currently playing.
    Playing,
    /// A track is currently paused.
    Paused,
    /// There is no track currently playing.
    Stopped,
}

/// [`PlaybackStatus`] had an invalid string value.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InvalidPlaybackStatus(String);

impl ::std::str::FromStr for PlaybackStatus {
    type Err = InvalidPlaybackStatus;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string {
            "Playing" => Ok(Self::Playing),
            "Paused" => Ok(Self::Paused),
            "Stopped" => Ok(Self::Stopped),
            _ => Err(InvalidPlaybackStatus(string.to_owned())),
        }
    }
}

impl PlaybackStatus {
    pub fn as_str(&self) -> &str {
        match self {
            PlaybackStatus::Playing => "Playing",
            PlaybackStatus::Paused => "Paused",
            PlaybackStatus::Stopped => "Stopped",
        }
    }
}

impl Display for PlaybackStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
/// A [`Player`]'s looping status.
///
/// See: [MPRIS2 specification about `Loop_Status`][loop_status]
///
/// [loop_status]: https://specifications.freedesktop.org/mpris-spec/latest/Player_Interface.html#Enum:Loop_Status
pub enum LoopStatus {
    /// The playback will stop when there are no more tracks to play
    None,

    /// The current track will start again from the begining once it has finished playing
    Track,

    /// The playback loops through a list of tracks
    Playlist,
}

/// [`LoopStatus`] had an invalid string value.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InvalidLoopStatus(String);

impl ::std::str::FromStr for LoopStatus {
    type Err = InvalidLoopStatus;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string {
            "None" => Ok(LoopStatus::None),
            "Track" => Ok(LoopStatus::Track),
            "Playlist" => Ok(LoopStatus::Playlist),
            _ => Err(InvalidLoopStatus(string.to_owned())),
        }
    }
}

impl LoopStatus {
    pub fn as_str(&self) -> &str {
        match self {
            LoopStatus::None => "None",
            LoopStatus::Track => "Track",
            LoopStatus::Playlist => "Playlist",
        }
    }
}

impl Display for LoopStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum MprisError {
    /// An error occurred while talking to the D-Bus.
    DbusError(zbus::Error),

    /// Failed to parse an enum from a string value received from the [`Player`]. This means that the
    /// [`Player`] replied with unexpected data.
    ParseError(String),

    /// Some other unexpected error occurred.
    Miscellaneous(String),
}

impl From<zbus::Error> for MprisError {
    fn from(value: zbus::Error) -> Self {
        MprisError::DbusError(value)
    }
}

impl From<InvalidPlaybackStatus> for MprisError {
    fn from(value: InvalidPlaybackStatus) -> Self {
        Self::ParseError(value.0)
    }
}

impl From<InvalidLoopStatus> for MprisError {
    fn from(value: InvalidLoopStatus) -> Self {
        Self::ParseError(value.0)
    }
}
impl From<InvalidTrackID> for MprisError {
    fn from(value: InvalidTrackID) -> Self {
        Self::ParseError(value.0)
    }
}

#[cfg(test)]
mod error_tests {
    use super::*;

    #[test]
    fn valid_playback_status() {
        assert_eq!("Playing".parse(), Ok(PlaybackStatus::Playing));
        assert_eq!("Paused".parse(), Ok(PlaybackStatus::Paused));
        assert_eq!("Stopped".parse(), Ok(PlaybackStatus::Stopped));
    }

    #[test]
    fn invalid_playback_status() {
        assert_eq!(
            "".parse::<PlaybackStatus>(),
            Err(InvalidPlaybackStatus("".into()))
        );
        assert_eq!(
            "playing".parse::<PlaybackStatus>(),
            Err(InvalidPlaybackStatus("playing".into()))
        );
        assert_eq!(
            "wrong".parse::<PlaybackStatus>(),
            Err(InvalidPlaybackStatus("wrong".into()))
        );
    }

    #[test]
    fn playback_status_as_str() {
        assert_eq!(PlaybackStatus::Playing.as_str(), "Playing");
        assert_eq!(PlaybackStatus::Paused.as_str(), "Paused");
        assert_eq!(PlaybackStatus::Stopped.as_str(), "Stopped");
    }

    #[test]
    fn valid_loop_status() {
        assert_eq!("None".parse(), Ok(LoopStatus::None));
        assert_eq!("Track".parse(), Ok(LoopStatus::Track));
        assert_eq!("Playlist".parse(), Ok(LoopStatus::Playlist));
    }

    #[test]
    fn invalid_loop_status() {
        assert_eq!("".parse::<LoopStatus>(), Err(InvalidLoopStatus("".into())));
        assert_eq!(
            "track".parse::<LoopStatus>(),
            Err(InvalidLoopStatus("track".into()))
        );
        assert_eq!(
            "wrong".parse::<LoopStatus>(),
            Err(InvalidLoopStatus("wrong".into()))
        );
    }

    #[test]
    fn loop_status_as_str() {
        assert_eq!(LoopStatus::None.as_str(), "None");
        assert_eq!(LoopStatus::Track.as_str(), "Track");
        assert_eq!(LoopStatus::Playlist.as_str(), "Playlist");
    }
}

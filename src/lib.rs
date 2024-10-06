use crate::proxies::DBusProxy;
use std::fmt::Display;

use zbus::{
    names::{BusName, WellKnownName},
    Connection,
};

mod metadata;
mod player;
mod proxies;

use metadata::InvalidTrackID;

pub use metadata::{Metadata, TrackID};
pub use player::Player;

pub(crate) const MPRIS2_PREFIX: &str = "org.mpris.MediaPlayer2.";

pub struct Mpris {
    connection: Connection,
}

impl Mpris {
    pub async fn new() -> Result<Self, MprisError> {
        let connection = Connection::session().await?;
        Ok(Self { connection })
    }

    pub fn new_from_connection(connection: Connection) -> Self {
        Self { connection }
    }

    pub async fn find_first(&self) -> Result<Option<Player>, MprisError> {
        match self.all_player_bus_names().await?.into_iter().next() {
            Some(bus) => Ok(Some(
                Player::new_from_connection(self.connection.clone(), bus).await?,
            )),
            None => Ok(None),
        }
    }

    pub async fn find_active(&self) -> Result<Option<Player>, MprisError> {
        let players = self.all_players().await?;
        if players.is_empty() {
            return Ok(None);
        }

        let mut first_paused: Option<Player> = None;
        let mut first_with_track: Option<Player> = None;
        let mut first_found: Option<Player> = None;

        for player in players {
            let player_status = player.playback_status().await?;

            if player_status == PlaybackStatus::Playing {
                return Ok(Some(player));
            }

            if first_paused.is_none() && player_status == PlaybackStatus::Paused {
                first_paused.replace(player);
            } else if first_with_track.is_none() && !player.metadata().await?.is_empty() {
                first_with_track.replace(player);
            } else if first_found.is_none() {
                first_found.replace(player);
            }
        }

        Ok(first_paused.or(first_with_track).or(first_found))
    }

    pub async fn find_by_name(&self, name: &str) -> Result<Option<Player>, MprisError> {
        let players = self.all_players().await?;
        if players.is_empty() {
            return Ok(None);
        }
        for player in players {
            if player.identity().await?.to_lowercase() == name {
                return Ok(Some(player));
            }
        }
        Ok(None)
    }

    pub async fn all_players(&self) -> Result<Vec<Player>, MprisError> {
        let bus_names = self.all_player_bus_names().await?;
        let mut players = Vec::with_capacity(bus_names.len());
        for player_name in bus_names {
            players.push(Player::new_from_connection(self.connection.clone(), player_name).await?);
        }
        Ok(players)
    }

    async fn all_player_bus_names(&self) -> Result<Vec<BusName<'static>>, MprisError> {
        let proxy = DBusProxy::new(&self.connection).await?;
        let mut names: Vec<BusName> = proxy
            .list_names()
            .await?
            .into_iter()
            .filter(|name| name.starts_with(MPRIS2_PREFIX))
            // We got the bus name from the D-Bus server so unchecked is fine
            .map(|name| BusName::from(WellKnownName::from_string_unchecked(name)))
            .collect();
        names.sort_unstable_by_key(|n| n.to_lowercase());
        Ok(names)
    }
}

impl From<Connection> for Mpris {
    fn from(value: Connection) -> Self {
        Self::new_from_connection(value)
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

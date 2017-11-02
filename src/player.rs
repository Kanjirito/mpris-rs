use super::PlaybackStatus;
use dbus::{Connection, ConnPath};
use generated::OrgMprisMediaPlayer2;
use generated::OrgMprisMediaPlayer2Player;
use metadata::Metadata;
use prelude::*;
use std::ops::Deref;

pub(crate) const MPRIS2_PREFIX: &str = "org.mpris.MediaPlayer2.";
pub(crate) const MPRIS2_PATH: &str = "/org/mpris/MediaPlayer2";

/// When D-Bus connection is managed for you, use this timeout while communicating with a Player.
pub const DEFAULT_TIMEOUT_MS: i32 = 500; // ms

/// A MPRIS-compatible player.
///
/// You can query this player about the currently playing media, or control it.
///
/// **See:** [MPRIS2 MediaPlayer2.Player Specification][spec]
/// [spec]: <https://specifications.freedesktop.org/mpris-spec/latest/Player_Interface.html>
pub struct Player<'conn, C: Deref<Target = Connection>> {
    connection_path: ConnPath<'conn, C>,
    identity: String,
}

impl<'conn, C: Deref<Target = Connection>> Player<'conn, C> {
    /// Create a new `Player` using a D-Bus connection path.
    ///
    /// If no player is running on this bus name an `Err` will be returned.
    pub fn new(connection_path: ConnPath<'conn, C>) -> Result<Player<'conn, C>> {
        let identity = connection_path.get_identity()?;

        Ok(Player {
            connection_path: connection_path,
            identity: identity,
        })
    }

    /// Returns the current D-Bus communication timeout (in milliseconds).
    ///
    /// When querying D-Bus the call should not block longer than this, and will instead fail the
    /// query if no response has been received in this time.
    ///
    /// You can change this using `set_dbus_timeout_ms`.
    pub fn dbus_timeout_ms(&self) -> i32 {
        self.connection_path.timeout
    }

    /// Change the D-Bus communication timeout.
    ///
    /// **See** `dbus_timeout_ms`
    pub fn set_dbus_timeout_ms(&mut self, timeout_ms: i32) {
        self.connection_path.timeout = timeout_ms;
    }

    /// Returns the player's D-Bus bus name.
    pub fn bus_name(&self) -> &str {
        &self.connection_path.dest
    }

    /// Returns the player's MPRIS `Identity`.
    ///
    /// This is usually the application's name, like `Spotify`.
    pub fn identity(&self) -> &str {
        &self.identity
    }

    /// Query the player for current metadata.
    ///
    /// See `Metadata` for more information about what is included here.
    pub fn get_metadata(&self) -> Result<Metadata> {
        self.connection_path
            .get_metadata()
            .map_err(|e| e.into())
            .and_then(Metadata::new_from_dbus)
    }

    /// Send a `PlayPause` signal to the player.
    ///
    /// See: [MPRIS2 specification about `PlayPause`](https://specifications.freedesktop.org/mpris-spec/latest/Player_Interface.html#Method:PlayPause)
    pub fn play_pause(&self) -> Result<()> {
        self.connection_path.play_pause().map_err(|e| e.into())
    }

    /// Send a `Play` signal to the player.
    ///
    /// See: [MPRIS2 specification about `Play`](https://specifications.freedesktop.org/mpris-spec/latest/Player_Interface.html#Method:Play)
    pub fn play(&self) -> Result<()> {
        self.connection_path.play().map_err(|e| e.into())
    }

    /// Send a `Pause` signal to the player.
    ///
    /// See: [MPRIS2 specification about `Pause`](https://specifications.freedesktop.org/mpris-spec/latest/Player_Interface.html#Method:Pause)
    pub fn pause(&self) -> Result<()> {
        self.connection_path.pause().map_err(|e| e.into())
    }

    /// Send a `Stop` signal to the player.
    ///
    /// See: [MPRIS2 specification about `Stop`](https://specifications.freedesktop.org/mpris-spec/latest/Player_Interface.html#Method:Stop)
    pub fn stop(&self) -> Result<()> {
        self.connection_path.stop().map_err(|e| e.into())
    }

    /// Send a `Next` signal to the player.
    ///
    /// See: [MPRIS2 specification about `Next`](https://specifications.freedesktop.org/mpris-spec/latest/Player_Interface.html#Method:Next)
    pub fn next(&self) -> Result<()> {
        self.connection_path.next().map_err(|e| e.into())
    }

    /// Send a `Previous` signal to the player.
    ///
    /// See: [MPRIS2 specification about `Previous`](https://specifications.freedesktop.org/mpris-spec/latest/Player_Interface.html#Method:Previous)
    pub fn previous(&self) -> Result<()> {
        self.connection_path.previous().map_err(|e| e.into())
    }

    /// Sends a `PlayPause` signal to the player, if the player indicates that it can pause.
    ///
    /// Returns a boolean to show if the signal was sent or not.
    ///
    /// See: [MPRIS2 specification about `PlayPause`](https://specifications.freedesktop.org/mpris-spec/latest/Player_Interface.html#Method:PlayPause)
    pub fn checked_play_pause(&self) -> Result<bool> {
        if self.can_pause()? {
            self.play_pause().map(|_| true)
        } else {
            Ok(false)
        }
    }

    /// Sends a `Play` signal to the player, if the player indicates that it can play.
    ///
    /// Returns a boolean to show if the signal was sent or not.
    ///
    /// See: [MPRIS2 specification about `Play`](https://specifications.freedesktop.org/mpris-spec/latest/Player_Interface.html#Method:Play)
    pub fn checked_play(&self) -> Result<bool> {
        if self.can_play()? {
            self.play().map(|_| true)
        } else {
            Ok(false)
        }
    }

    /// Sends a `Pause` signal to the player, if the player indicates that it can pause.
    ///
    /// Returns a boolean to show if the signal was sent or not.
    ///
    /// See: [MPRIS2 specification about `Pause`](https://specifications.freedesktop.org/mpris-spec/latest/Player_Interface.html#Method:Pause)
    pub fn checked_pause(&self) -> Result<bool> {
        if self.can_pause()? {
            self.pause().map(|_| true)
        } else {
            Ok(false)
        }
    }

    /// Sends a `Stop` signal to the player, if the player indicates that it can stop.
    ///
    /// Returns a boolean to show if the signal was sent or not.
    ///
    /// See: [MPRIS2 specification about `Stop`](https://specifications.freedesktop.org/mpris-spec/latest/Player_Interface.html#Method:Stop)
    pub fn checked_stop(&self) -> Result<bool> {
        if self.can_stop()? {
            self.stop().map(|_| true)
        } else {
            Ok(false)
        }
    }

    /// Sends a `Next` signal to the player, if the player indicates that it can go to the next
    /// media.
    ///
    /// Returns a boolean to show if the signal was sent or not.
    ///
    /// See: [MPRIS2 specification about `Next`](https://specifications.freedesktop.org/mpris-spec/latest/Player_Interface.html#Method:Next)
    pub fn checked_next(&self) -> Result<bool> {
        if self.can_go_next()? {
            self.next().map(|_| true)
        } else {
            Ok(false)
        }
    }

    /// Sends a `Previous` signal to the player, if the player indicates that it can go to a
    /// previous media.
    ///
    /// Returns a boolean to show if the signal was sent or not.
    ///
    /// See: [MPRIS2 specification about `Previous`](https://specifications.freedesktop.org/mpris-spec/latest/Player_Interface.html#Method:Previous)
    pub fn checked_previous(&self) -> Result<bool> {
        if self.can_go_previous()? {
            self.previous().map(|_| true)
        } else {
            Ok(false)
        }
    }

    /// Queries the player to see if it can be controlled or not.
    ///
    /// See: [MPRIS2 specification about `CanControl`](https://specifications.freedesktop.org/mpris-spec/latest/Player_Interface.html#Property:CanControl)
    pub fn can_control(&self) -> Result<bool> {
        self.connection_path.get_can_control().map_err(|e| e.into())
    }

    /// Queries the player to see if it can go to next or not.
    ///
    /// See: [MPRIS2 specification about `CanGoNext`](https://specifications.freedesktop.org/mpris-spec/latest/Player_Interface.html#Property:CanGoNext)
    pub fn can_go_next(&self) -> Result<bool> {
        self.connection_path.get_can_go_next().map_err(|e| e.into())
    }

    /// Queries the player to see if it can go to previous or not.
    ///
    /// See: [MPRIS2 specification about `CanGoPrevious`](https://specifications.freedesktop.org/mpris-spec/latest/Player_Interface.html#Property:CanGoPrevious)
    pub fn can_go_previous(&self) -> Result<bool> {
        self.connection_path.get_can_go_previous().map_err(
            |e| e.into(),
        )
    }

    /// Queries the player to see if it can pause.
    ///
    /// See: [MPRIS2 specification about `CanPause`](https://specifications.freedesktop.org/mpris-spec/latest/Player_Interface.html#Property:CanPause)
    pub fn can_pause(&self) -> Result<bool> {
        self.connection_path.get_can_pause().map_err(|e| e.into())
    }

    /// Queries the player to see if it can play.
    ///
    /// See: [MPRIS2 specification about `CanPlay`](https://specifications.freedesktop.org/mpris-spec/latest/Player_Interface.html#Property:CanPlay)
    pub fn can_play(&self) -> Result<bool> {
        self.connection_path.get_can_play().map_err(|e| e.into())
    }

    /// Queries the player to see if it can seek within the media.
    ///
    /// See: [MPRIS2 specification about `CanSeek`](https://specifications.freedesktop.org/mpris-spec/latest/Player_Interface.html#Property:CanSeek)
    pub fn can_seek(&self) -> Result<bool> {
        self.connection_path.get_can_seek().map_err(|e| e.into())
    }

    /// Queries the player to see if it can stop.
    ///
    /// MPRIS2 defines [the `Stop` message to only work when the player can be controlled][stop], so that
    /// is the property used for this method.
    ///
    /// See: [MPRIS2 specification about `CanControl`](https://specifications.freedesktop.org/mpris-spec/latest/Player_Interface.html#Property:CanControl)
    /// [stop]: https://specifications.freedesktop.org/mpris-spec/latest/Player_Interface.html#Method:Stop
    pub fn can_stop(&self) -> Result<bool> {
        self.can_control()
    }

    /// Query the player for current playback status.
    pub fn get_playback_status(&self) -> Result<PlaybackStatus> {
        self.connection_path.get_playback_status()?.parse()
    }
}
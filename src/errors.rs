/// [`PlaybackStatus`][crate::PlaybackStatus] had an invalid string value.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InvalidPlaybackStatus(pub(crate) String);

/// [`LoopStatus`][crate::LoopStatus] had an invalid string value.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InvalidLoopStatus(pub(crate) String);

/// [`TrackID`][crate::metadata::TrackID] had an invalid ObjectPath.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InvalidTrackID(pub(crate) String);

#[derive(Debug, PartialEq, Clone)]
pub enum MprisError {
    /// An error occurred while talking to the D-Bus.
    DbusError(zbus::Error),

    /// Failed to parse an enum from a string value received from the [`Player`][crate::Player].
    /// This means that the [`Player`][crate::Player] replied with unexpected data.
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

impl From<String> for MprisError {
    fn from(value: String) -> Self {
        Self::Miscellaneous(value)
    }
}

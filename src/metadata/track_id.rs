use std::ops::Deref;

use zbus::zvariant::{ObjectPath, OwnedObjectPath, OwnedValue, Value};

use super::MetadataValue;
use crate::errors::InvalidTrackID;

const NO_TRACK: &str = "/org/mpris/MediaPlayer2/TrackList/NoTrack";

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct TrackID(String);

impl TrackID {
    pub fn new(id: String) -> Result<Self, InvalidTrackID> {
        Self::try_from(id)
    }

    pub fn no_track() -> Self {
        // We know it's a valid path so it's safe to skip the check
        Self(NO_TRACK.into())
    }

    pub fn is_no_track(&self) -> bool {
        self.as_str() == NO_TRACK
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub(crate) fn get_object_path(&self) -> ObjectPath {
        // Safe because we checked the string at creation
        ObjectPath::from_str_unchecked(&self.0)
    }
}

impl TryFrom<&str> for TrackID {
    type Error = InvalidTrackID;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match ObjectPath::try_from(value) {
            Ok(_) => Ok(Self(value.to_string())),
            Err(e) => {
                if let zbus::zvariant::Error::Message(s) = e {
                    Err(InvalidTrackID(s))
                } else {
                    // ObjectValue only creates Serde errors which get converted into
                    // zbus::zvariant::Error::Message
                    unreachable!("ObjectPath should only return Message errors")
                }
            }
        }
    }
}

impl TryFrom<String> for TrackID {
    type Error = InvalidTrackID;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match ObjectPath::try_from(value.as_str()) {
            Ok(_) => Ok(Self(value)),
            Err(e) => {
                if let zbus::zvariant::Error::Message(s) = e {
                    Err(InvalidTrackID(s))
                } else {
                    // ObjectValue only creates Serde errors which get converted into
                    // zbus::zvariant::Error::Message
                    unreachable!("ObjectPath should only return Message errors")
                }
            }
        }
    }
}

impl TryFrom<MetadataValue> for TrackID {
    type Error = InvalidTrackID;

    fn try_from(value: MetadataValue) -> Result<Self, Self::Error> {
        match value {
            MetadataValue::String(s) => s.try_into(),
            _ => Err(InvalidTrackID(String::from("not a string"))),
        }
    }
}

impl TryFrom<OwnedValue> for TrackID {
    type Error = InvalidTrackID;

    fn try_from(value: OwnedValue) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

impl TryFrom<&OwnedValue> for TrackID {
    type Error = InvalidTrackID;

    fn try_from(value: &OwnedValue) -> Result<Self, Self::Error> {
        match value.deref() {
            Value::Str(s) => s.as_str().try_into(),
            Value::ObjectPath(path) => Ok(Self(path.to_string())),
            _ => Err(InvalidTrackID(String::from("not a String or ObjectPath"))),
        }
    }
}

impl<'a> TryFrom<Value<'a>> for TrackID {
    type Error = InvalidTrackID;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Str(s) => s.as_str().try_into(),
            Value::ObjectPath(path) => Ok(Self(path.to_string())),
            _ => Err(InvalidTrackID(String::from("not a String or ObjectPath"))),
        }
    }
}

impl From<OwnedObjectPath> for TrackID {
    fn from(value: OwnedObjectPath) -> Self {
        Self(value.to_string())
    }
}

impl From<&OwnedObjectPath> for TrackID {
    fn from(value: &OwnedObjectPath) -> Self {
        Self(value.to_string())
    }
}

impl From<ObjectPath<'_>> for TrackID {
    fn from(value: ObjectPath) -> Self {
        Self(value.to_string())
    }
}

impl From<&ObjectPath<'_>> for TrackID {
    fn from(value: &ObjectPath) -> Self {
        Self(value.to_string())
    }
}

impl Deref for TrackID {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<TrackID> for ObjectPath<'_> {
    fn from(val: TrackID) -> Self {
        // We used a ObjectPath when creating TrackID so it's safe to skip the check
        ObjectPath::from_string_unchecked(val.0)
    }
}

impl From<TrackID> for OwnedObjectPath {
    fn from(val: TrackID) -> Self {
        OwnedObjectPath::from(ObjectPath::from(val))
    }
}

impl From<TrackID> for String {
    fn from(value: TrackID) -> Self {
        value.0
    }
}

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use super::*;
    use serde_test::{assert_de_tokens, assert_ser_tokens, Token};

    #[test]
    fn test_serialization() {
        let track_id = TrackID::try_from("/foo/bar").unwrap();
        assert_ser_tokens(&track_id, &[Token::Str("/foo/bar")]);
    }

    #[test]
    fn test_deserialization() {
        let track_id = TrackID::try_from("/foo/bar").unwrap();
        assert_de_tokens(&track_id, &[Token::Str("/foo/bar")]);
    }
}

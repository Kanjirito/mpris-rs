use zbus::zvariant::Value;

use crate::errors::InvalidMetadataValue;

/*
* Subset of DBus data types that are commonly used in MPRIS metadata, and a boolean variant as it
* seems likely to be used in some custom metadata.
*
* See https://www.freedesktop.org/wiki/Specifications/mpris-spec/metadata/
*/
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MetadataValue {
    Boolean(bool),
    Float(f64),
    SignedInt(i64),
    UnsignedInt(u64),
    String(String),
    Strings(Vec<String>),
    Unsupported,
}

impl MetadataValue {
    pub fn into_nonempty_string(self) -> Option<String> {
        String::try_from(self)
            .ok()
            .and_then(|s| if s.is_empty() { None } else { Some(s) })
    }

    pub fn into_i64(self) -> Option<i64> {
        match self {
            MetadataValue::SignedInt(i) => Some(i),
            MetadataValue::UnsignedInt(i) => Some(i.clamp(0, i64::MAX as u64) as i64),
            _ => None,
        }
    }

    pub fn into_u64(self) -> Option<u64> {
        match self {
            MetadataValue::SignedInt(i) if i < 0 => Some(0),
            MetadataValue::SignedInt(i) => Some(i as u64),
            MetadataValue::UnsignedInt(i) => Some(i),
            _ => None,
        }
    }
}

impl<'a> From<Value<'a>> for MetadataValue {
    fn from(value: Value) -> Self {
        match value {
            Value::Bool(v) => MetadataValue::Boolean(v),
            Value::I16(v) => MetadataValue::SignedInt(v as i64),
            Value::I32(v) => MetadataValue::SignedInt(v as i64),
            Value::I64(v) => MetadataValue::SignedInt(v),
            Value::U16(v) => MetadataValue::UnsignedInt(v as u64),
            Value::U32(v) => MetadataValue::UnsignedInt(v as u64),
            Value::U64(v) => MetadataValue::UnsignedInt(v),
            Value::U8(v) => MetadataValue::UnsignedInt(v as u64),

            Value::F64(v) => MetadataValue::Float(v),

            Value::Str(v) => MetadataValue::String(v.to_string()),
            Value::Signature(v) => MetadataValue::String(v.to_string()),
            Value::ObjectPath(v) => MetadataValue::String(v.to_string()),

            Value::Array(a) if a.full_signature() == "as" => {
                let mut strings = Vec::with_capacity(a.len());
                for v in a.iter() {
                    if let Value::Str(s) = v {
                        strings.push(s.to_string());
                    }
                }
                MetadataValue::Strings(strings)
            }

            Value::Value(v) => MetadataValue::from(*v),

            Value::Array(_) => MetadataValue::Unsupported,
            Value::Dict(_) => MetadataValue::Unsupported,
            Value::Structure(_) => MetadataValue::Unsupported,
            Value::Fd(_) => MetadataValue::Unsupported,
        }
    }
}

impl From<bool> for MetadataValue {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}

impl From<f64> for MetadataValue {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}

impl From<i64> for MetadataValue {
    fn from(value: i64) -> Self {
        Self::SignedInt(value)
    }
}

impl From<u64> for MetadataValue {
    fn from(value: u64) -> Self {
        Self::UnsignedInt(value)
    }
}

impl From<String> for MetadataValue {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<Vec<String>> for MetadataValue {
    fn from(value: Vec<String>) -> Self {
        Self::Strings(value)
    }
}

impl From<super::TrackID> for MetadataValue {
    fn from(value: super::TrackID) -> Self {
        Self::String(value.into())
    }
}

impl From<crate::MprisDuration> for MetadataValue {
    fn from(value: crate::MprisDuration) -> Self {
        Self::SignedInt(value.into())
    }
}

impl TryFrom<MetadataValue> for bool {
    type Error = InvalidMetadataValue;

    fn try_from(value: MetadataValue) -> Result<Self, Self::Error> {
        match value {
            MetadataValue::Boolean(v) => Ok(v),
            _ => Err(InvalidMetadataValue::from(
                "expected MetadataValue::Boolean",
            )),
        }
    }
}

impl TryFrom<MetadataValue> for f64 {
    type Error = InvalidMetadataValue;

    fn try_from(value: MetadataValue) -> Result<Self, Self::Error> {
        match value {
            MetadataValue::Float(v) => Ok(v),
            _ => Err(InvalidMetadataValue::from("expected MetadataValue::Float")),
        }
    }
}

impl TryFrom<MetadataValue> for i64 {
    type Error = InvalidMetadataValue;

    fn try_from(value: MetadataValue) -> Result<Self, Self::Error> {
        match value {
            MetadataValue::SignedInt(v) => Ok(v),
            MetadataValue::UnsignedInt(v) => {
                if v <= i64::MAX as u64 {
                    Ok(v as i64)
                } else {
                    Err(InvalidMetadataValue::from("value too big for i64"))
                }
            }
            _ => Err(InvalidMetadataValue::from(
                "expected MetadataValue::SignedInt or MetadataValue::UnsignedInt",
            )),
        }
    }
}

impl TryFrom<MetadataValue> for u64 {
    type Error = InvalidMetadataValue;

    fn try_from(value: MetadataValue) -> Result<Self, Self::Error> {
        match value {
            MetadataValue::UnsignedInt(v) => Ok(v),
            MetadataValue::SignedInt(v) => {
                if v >= 0 {
                    Ok(v as u64)
                } else {
                    Err(InvalidMetadataValue::from("value is negative"))
                }
            }
            _ => Err(InvalidMetadataValue::from(
                "expected MetadataValue::SignedInt or MetadataValue::UnsignedInt",
            )),
        }
    }
}

impl TryFrom<MetadataValue> for String {
    type Error = InvalidMetadataValue;

    fn try_from(value: MetadataValue) -> Result<Self, Self::Error> {
        match value {
            MetadataValue::String(v) => Ok(v),
            MetadataValue::Strings(mut v) => {
                if v.len() == 1 {
                    Ok(v.pop().unwrap())
                } else {
                    Err(InvalidMetadataValue::from(
                        "MetadataValue::Strings contains more than 1 String",
                    ))
                }
            }
            _ => Err(InvalidMetadataValue::from(
                "expected MetadataValue::Strings or MetadataValue::String",
            )),
        }
    }
}

impl TryFrom<MetadataValue> for Vec<String> {
    type Error = InvalidMetadataValue;

    fn try_from(value: MetadataValue) -> Result<Self, Self::Error> {
        match value {
            MetadataValue::String(v) => Ok(vec![v]),
            MetadataValue::Strings(v) => Ok(v),
            _ => Err(InvalidMetadataValue::from(
                "expected MetadataValue::Strings or MetadataValue::String",
            )),
        }
    }
}

#[cfg(test)]
mod metadata_value_integer_tests {
    use super::*;

    #[test]
    fn test_signed_integer_casting() {
        assert_eq!(
            MetadataValue::SignedInt(i64::MIN).into_i64(),
            Some(i64::MIN)
        );
        assert_eq!(MetadataValue::SignedInt(0).into_i64(), Some(0_i64));
        assert_eq!(
            MetadataValue::SignedInt(i64::MAX).into_i64(),
            Some(i64::MAX)
        );
        assert_eq!(MetadataValue::UnsignedInt(0).into_i64(), Some(0_i64));
        assert_eq!(
            MetadataValue::UnsignedInt(u64::MAX).into_i64(),
            Some(i64::MAX)
        );

        assert_eq!(MetadataValue::SignedInt(i64::MIN).try_into(), Ok(i64::MIN));
        assert_eq!(MetadataValue::SignedInt(0_i64).try_into(), Ok(0_i64));
        assert_eq!(MetadataValue::SignedInt(i64::MAX).try_into(), Ok(i64::MAX));
        assert_eq!(MetadataValue::UnsignedInt(0).try_into(), Ok(0_i64));
        assert!(i64::try_from(MetadataValue::UnsignedInt(u64::MAX)).is_err());
    }

    #[test]
    fn test_unsigned_integer_casting() {
        assert_eq!(MetadataValue::SignedInt(i64::MIN).into_u64(), Some(0_u64));
        assert_eq!(MetadataValue::SignedInt(0).into_u64(), Some(0_u64));
        assert_eq!(
            MetadataValue::SignedInt(i64::MAX).into_u64(),
            Some(i64::MAX as u64)
        );
        assert_eq!(MetadataValue::UnsignedInt(0).into_u64(), Some(0_u64));
        assert_eq!(
            MetadataValue::UnsignedInt(u64::MAX).into_u64(),
            Some(u64::MAX)
        );

        assert!(u64::try_from(MetadataValue::SignedInt(i64::MIN)).is_err());
        assert_eq!(MetadataValue::SignedInt(0).try_into(), Ok(0_u64));
        assert_eq!(
            MetadataValue::SignedInt(i64::MAX).try_into(),
            Ok(i64::MAX as u64)
        );
        assert_eq!(MetadataValue::UnsignedInt(0).try_into(), Ok(0_u64));
        assert_eq!(
            MetadataValue::UnsignedInt(u64::MAX).try_into(),
            Ok(u64::MAX)
        );
    }
}

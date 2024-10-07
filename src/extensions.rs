use std::time::Duration;

use crate::MprisError;

pub(crate) trait DurationExt {
    /// Tries to convert the Duration as microseconds to a valid i64
    fn convert_to_micro(self) -> Result<i64, MprisError>;
}

impl DurationExt for Duration {
    fn convert_to_micro(self) -> Result<i64, MprisError> {
        i64::try_from(self.as_micros()).map_err(|_| {
            MprisError::Miscellaneous(
                "could not convert Duration into microseconds, Duration too big".to_string(),
            )
        })
    }
}

#[cfg(test)]
mod duration_ext_tests {
    use super::*;

    #[test]
    fn valid_convert() {
        assert_eq!(Duration::default().convert_to_micro(), Ok(0));
        assert_eq!(
            Duration::from_micros(i64::MAX as u64).convert_to_micro(),
            Ok(i64::MAX)
        )
    }

    #[test]
    fn invalid_convert() {
        assert!(Duration::from_micros(i64::MAX as u64 + 1)
            .convert_to_micro()
            .is_err());
    }
}

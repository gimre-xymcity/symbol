use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NetworkTimestamp(i64);

/// Represents a symbol network timestamp with millisecond resolution
///
impl NetworkTimestamp {
    /// Returns `true` if this is epochal timestamp.
    pub fn is_epochal(&self) -> bool {
        0 == self.0
    }

    /// Adds a specified number of milliseconds to timestamp.
    pub fn add_milliseconds(&self, count: i64) -> Self {
        Self(self.0 + count)
    }

    /// Adds a specified number of seconds to timestamp.
    pub fn add_seconds(&self, count: i64) -> Self {
        self.add_milliseconds(1000 * count)
    }

    /// Adds a specified number of minutes to timestamp.
    pub fn add_minutes(&self, count: i64) -> Self {
        self.add_seconds(60 * count)
    }

    /// Adds a specified number of hours to timestamp.
    pub fn add_hours(&self, count: i64) -> Self {
        self.add_minutes(60 * count)
    }
}

impl fmt::Display for NetworkTimestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{}", self.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_epochal_timestamp() {
        // Act:
        let timestamp = NetworkTimestamp(0);

        // Assert:
        assert!(timestamp.is_epochal());
        assert_eq!(0, timestamp.0);
    }

    #[test]
    fn can_create_non_epochal_timestamp() {
        // Act:
        let timestamp = NetworkTimestamp(123);

        // Assert:
        assert!(!timestamp.is_epochal());
        assert_eq!(123, timestamp.0);
    }

    #[test]
    fn can_add_milliseconds() {
        // Arrange:
        let original_timestamp = NetworkTimestamp(100);

        // Act:
        let timestamp = original_timestamp.add_milliseconds(50);

        // Assert:
        assert_eq!(100, original_timestamp.0);
        assert_eq!(150, timestamp.0);
    }

    #[test]
    fn can_add_seconds() {
        // Arrange:
        let original_timestamp = NetworkTimestamp(100);

        // Act:
        let timestamp = original_timestamp.add_seconds(50);

        // Assert:
        assert_eq!(100, original_timestamp.0);
        assert_eq!(100 + 50 * 1000, timestamp.0);
    }

    #[test]
    fn can_add_minutes() {
        // Arrange:
        let original_timestamp = NetworkTimestamp(100);

        // Act:
        let timestamp = original_timestamp.add_minutes(50);

        // Assert:
        assert_eq!(100, original_timestamp.0);
        assert_eq!(100 + 50 * 60 * 1000, timestamp.0);
    }

    #[test]
    fn can_add_hours() {
        // Arrange:
        let original_timestamp = NetworkTimestamp(100);

        // Act:
        let timestamp = original_timestamp.add_hours(50);

        // Assert:
        assert_eq!(100, original_timestamp.0);
        assert_eq!(100 + 50 * 60 * 60 * 1000, timestamp.0);
    }

    #[test]
    fn equality_is_supported() {
        // Arrange:
        let timestamp = NetworkTimestamp(12345);

        // Act + Assert:
        assert_eq!(timestamp, NetworkTimestamp(12345));
        assert_ne!(timestamp, NetworkTimestamp(12344));
        assert_ne!(timestamp, NetworkTimestamp(12346));
    }

    #[test]
    fn display_is_supported() {
        // Arrange:
        let timestamp = NetworkTimestamp(123).add_hours(10);

        // Act + Assert:
        assert_eq!("36000123", format!("{}", timestamp));
    }
}

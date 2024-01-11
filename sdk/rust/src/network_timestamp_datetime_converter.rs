use chrono::DateTime;
use chrono::Duration;
use chrono::Utc;

pub enum TimeUnits {
    Hours,
    Seconds,
    Milliseconds,
}

/// Provides utilities for converting between network timestamp and datetimes.
pub struct NetworkTimestampDatetimeConverter {
    pub epoch: DateTime<Utc>,
    pub time_units: TimeUnits,
}

impl NetworkTimestampDatetimeConverter {
    pub fn to_datetime(&self, raw_timestamp: i64) -> DateTime<Utc> {
        self.epoch
            + match self.time_units {
                TimeUnits::Hours => Duration::hours,
                TimeUnits::Seconds => Duration::seconds,
                TimeUnits::Milliseconds => Duration::milliseconds,
            }(raw_timestamp)
    }

    pub fn to_difference(&self, datetime: DateTime<Utc>) -> i64 {
        let duration = datetime - self.epoch;
        assert!(duration >= Duration::zero());
        match self.time_units {
            TimeUnits::Hours => duration.num_hours(),
            TimeUnits::Seconds => duration.num_seconds(),
            TimeUnits::Milliseconds => duration.num_milliseconds(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDate, TimeZone};

    fn create_datetime(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        min: u32,
        sec: u32,
    ) -> DateTime<Utc> {
        Utc.from_utc_datetime(
            &NaiveDate::from_ymd_opt(year, month, day)
                .unwrap()
                .and_hms_opt(hour, min, sec)
                .unwrap(),
        )
    }

    fn create_converter() -> NetworkTimestampDatetimeConverter {
        let epoch_time = create_datetime(2020, 1, 2, 3, 0, 0);

        NetworkTimestampDatetimeConverter {
            epoch: epoch_time,
            time_units: TimeUnits::Hours,
        }
    }

    #[test]
    fn can_convert_epochal_timestamp_to_datetime() {
        // Arrange:
        let converter = create_converter();

        // Act:
        let utc_datetime = converter.to_datetime(0);

        // Assert:
        assert_eq!(
            Utc.from_utc_datetime(
                &NaiveDate::from_ymd_opt(2020, 1, 2)
                    .unwrap()
                    .and_hms_opt(3, 0, 0)
                    .unwrap()
            ),
            utc_datetime
        )
    }

    #[test]
    fn can_convert_non_epochal_timestamp_to_datetime() {
        // Arrange:
        let converter = create_converter();

        // Act:
        let utc_datetime = converter.to_datetime(5);

        // Assert:
        assert_eq!(
            Utc.from_utc_datetime(
                &NaiveDate::from_ymd_opt(2020, 1, 2)
                    .unwrap()
                    .and_hms_opt(3 + 5, 0, 0)
                    .unwrap()
            ),
            utc_datetime
        )
    }

    #[test]
    #[should_panic]
    fn cannot_convert_datetime_before_epochal_timestamp() {
        // Arrange:
        let converter = create_converter();

        // Act:
        let raw_timestamp = converter.to_difference(create_datetime(2020, 1, 2, 2, 0, 0));

        // Assert:
        assert_eq!(0, raw_timestamp);
    }

    #[test]
    fn can_convert_datetime_to_epochal_timestamp() {
        // Arrange:
        let converter = create_converter();

        // Act:
        let raw_timestamp = converter.to_difference(create_datetime(2020, 1, 2, 3, 0, 0));

        // Assert:
        assert_eq!(0, raw_timestamp);
    }

    #[test]
    fn can_convert_datetime_to_non_epochal_timestamp() {
        // Arrange:
        let converter = create_converter();

        // Act:
        let raw_timestamp = converter.to_difference(create_datetime(2020, 1, 2, 3 + 5, 0, 0));

        // Assert:
        assert_eq!(5, raw_timestamp);
    }
}

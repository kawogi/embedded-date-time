use core::fmt;

use crate::DateTime;
use crate::Time;
use crate::Weekday;

/// A simple date in the Gregorian calendar without time zone information.
#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Clone)]
pub struct Date {
    /// The year as commonly used in a date.
    ///
    /// The range of 2000 to 2099 is guaranteed to be supported by all methods.
    /// For support beyond this range, consult the documentation of the respective implementation.
    pub year: u16,

    /// Month of the year [1-12].
    pub month: u8,

    /// Day of the month [1-31].
    pub day: u8,
}

impl Date {
    /// Create a new Date from year, month and day.
    ///
    /// No checks will be performed to validate the date.
    #[must_use]
    pub fn new(year: u16, month: u8, day: u8) -> Self {
        Self { year, month, day }
    }

    /// Create a new Date from year, month and day.
    ///
    /// Returns None if the year, month or day is invalid.
    ///
    /// The year must be between 2000 and 2199.
    /// This range might be extended in the future if the implementation supports it.
    #[must_use]
    pub fn new_checked(year: u16, month: u8, day: u8) -> Option<Self> {
        let result = Self::new(year, month, day);
        result.is_valid().then_some(result)
    }

    /// Determine whether the date is a valid combination of year, month and day.
    ///
    /// This is guaranteed to work for the years between 2000 and 2199, inclusive.
    ///
    /// Years outside of this range will return arbitrary results or may panic.
    #[must_use]
    pub fn is_valid(self) -> bool {
        if !(2000..=2199).contains(&self.year) {
            return false;
        }

        let days_in_month = match self.month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => 28 + u8::from(self.is_leap_year()),
            _ => return false,
        };

        (1..=days_in_month).contains(&self.day)
    }

    /// Helper function to determine whether a given year is a leap year.
    ///
    /// This is guaranteed to work for the years between 2000 and 2199.
    ///
    /// Years outside of this range will return arbitrary results or may panic.
    #[must_use]
    pub fn is_leap_year(self) -> bool {
        let year2000 = self.year2000();
        // check if the year is divisible by 4 but make an exception for 2100
        year2000.trailing_zeros() >= 2 && year2000 != 100
    }

    /// Create a new `DateTime` from a `Date` and a `Time`.
    ///
    /// Returns None if the hours, minutes or seconds is invalid.
    #[must_use]
    pub fn with_time(self, time: Time) -> DateTime {
        DateTime { date: self, time }
    }

    /// Returns the year relative to 2000.
    ///
    /// This is guaranteed to work for the years between 2000 and 2255, inclusive.
    ///
    /// The purpose of this method is to obtain a year with reduced width for cheaper calculations.
    /// Subtracting 2000 preserves the leap year pattern.
    ///
    /// Years outside of this range will return arbitrary results or may panic.
    #[must_use]
    #[expect(
        clippy::cast_possible_truncation,
        reason = "the limitation is documented"
    )]
    pub fn year2000(self) -> u8 {
        (self.year - 2000) as u8
    }

    /// Returns the weekday of the date.
    ///
    /// This is guaranteed to work for all valid dates within the years 2000 to 2171.
    ///
    /// Invalid dates will return arbitrary results or may panic.
    #[must_use]
    pub fn weekday(self) -> Weekday {
        // number of week-day shifts for each month of a non-leap year
        // To keep this number low, we use modulo 7 at compile time which won't affect the result.
        let months = [
            0_u8, // enables 1-based indexing of months
            0,    // January
            31 % 7,
            59 % 7,
            90 % 7,
            120 % 7,
            151 % 7,
            181 % 7,
            212 % 7,
            243 % 7,
            (273 % 7) as u8,
            (304 % 7) as u8,
            (334 % 7) as u8, // December
        ];

        // number of week-day shifts for the year
        // one extra-shift for each leap year (with a correction for the skipped year 2100)
        let year2000 = self.year2000();
        let year_ordinal = year2000 + year2000.div_ceil(4) - u8::from(year2000 > 100);

        // insert Feb. 29 for leap years
        let leap_ordinal = u8::from(self.month > 2 && self.is_leap_year());

        // number of week-day shifts for the month
        let month_ordinal = *months.get(usize::from(self.month)).unwrap_or(&0);

        // number of week-day shifts for the day
        let day_ordinal = self.day;

        // total number of week-day shifts
        // This will overflow past the year 2171; extend to u16 if this becomes a real limitation.
        let ordinal_date = year_ordinal + leap_ordinal + month_ordinal + day_ordinal;

        // this is the only real division (modulo 7)
        // the offset has been chosen to make the result match the ordinal value of the Weekday-enum
        Weekday::from((ordinal_date + 5) % 7)
    }
}

impl fmt::Debug for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for Date {
    fn format(&self, fmt: defmt::Formatter<'_>) {
        defmt::write!(fmt, "{:04}-{:02}-{:02}", self.year, self.month, self.day);
    }
}

#[cfg(test)]
mod tests {
    #![expect(clippy::panic, reason = "this is a test")]

    #[test]
    fn test_validation() {
        // iterate through all valid (and some invalid) dates and test whether `chrono` and
        // `embedded-date-time` agree on the validation.
        for year in 2000_u16..=2199 {
            for month in 0_u8..=13 {
                for day in 0_u8..=32 {
                    let embedded_date = crate::Date::new_checked(year, month, day);
                    let chrono_date = chrono::NaiveDate::from_ymd_opt(
                        i32::from(year),
                        u32::from(month),
                        u32::from(day),
                    );

                    match (embedded_date, chrono_date) {
                        (None, Some(date)) => {
                            panic!("chrono validated but embedded didn't: {date}")
                        }
                        (Some(date), None) => {
                            panic!("embedded validated but chrono didn't: {date:?}")
                        }
                        (None, None) | (Some(_), Some(_)) => {}
                    }
                }
            }
        }
    }
}

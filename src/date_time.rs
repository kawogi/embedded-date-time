use core::fmt;

use crate::Date;
use crate::Time;
use crate::Weekday;

/// Combines a simple date and time.
#[expect(
    missing_copy_implementations,
    reason = "This type isn't `Copy` on purpose as embedded systems might not handle 64-bit operations efficiently."
)]
#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Clone)]
pub struct DateTime {
    /// The date part of this date and time.
    pub date: Date,

    /// The time part of this date and time.
    pub time: Time,
}

impl DateTime {
    /// Return the year of the date.
    #[must_use]
    pub fn year(self) -> u16 {
        self.date.year
    }

    /// Return the month of the date [1-12].
    #[must_use]
    pub fn month(self) -> u8 {
        self.date.month
    }

    /// Return the day of the month for this date [1-31].
    #[must_use]
    pub fn day(self) -> u8 {
        self.date.day
    }

    /// Read the hour of the time [0-23].
    #[must_use]
    pub fn hour(self) -> u8 {
        self.time.hour
    }

    /// Read the minute of the time [0-59].
    #[must_use]
    pub fn minute(self) -> u8 {
        self.time.minute
    }

    /// Read the second of the time [0-60].
    ///
    /// A value of 60 might occur on a leap second.
    #[must_use]
    pub fn second(self) -> u8 {
        self.time.second
    }

    /// Compute the weekday of the date.
    ///
    /// This is guaranteed to work for all valid dates within the years 2000 to 2171.
    ///
    /// Invalid dates will return arbitrary results or may panic.
    #[must_use]
    pub fn weekday(self) -> Weekday {
        self.date.weekday()
    }
}

impl fmt::Debug for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}

impl fmt::Display for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.date, self.time)
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for DateTime {
    fn format(&self, fmt: defmt::Formatter<'_>) {
        defmt::write!(fmt, "{} {}", self.date, self.time);
    }
}

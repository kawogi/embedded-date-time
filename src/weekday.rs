use core::fmt;

/// Day of the week starting from Monday = 1 (ISO 8601).
///
/// The ordinal value of 0 is not being used by common RTC chips.
/// This place can be used to re-map Sunday to 0 in order to obtain a week starting from Sunday.
///
/// `PartialOrd` isn't implemented on purpose because weekdays are usually used in a cyclic manner.
#[derive(PartialEq, Eq, Hash, Copy, Clone)]
#[repr(u8)]
pub enum Weekday {
    /// Monday.
    Mon = Self::MON_NUM,
    /// Tuesday.
    Tue = Self::TUE_NUM,
    /// Wednesday.
    Wed = Self::WED_NUM,
    /// Thursday.
    Thu = Self::THU_NUM,
    /// Friday.
    Fri = Self::FRI_NUM,
    /// Saturday.
    Sat = Self::SAT_NUM,
    /// Sunday.
    Sun = Self::SUN_NUM,
}

impl Weekday {
    /// the numeric code being used for Monday.
    pub const MON_NUM: u8 = 1;
    /// the numeric code being used for Tuesday.
    pub const TUE_NUM: u8 = 2;
    /// the numeric code being used for Wednesday.
    pub const WED_NUM: u8 = 3;
    /// the numeric code being used for Thursday.
    pub const THU_NUM: u8 = 4;
    /// the numeric code being used for Friday.
    pub const FRI_NUM: u8 = 5;
    /// the numeric code being used for Saturday.
    pub const SAT_NUM: u8 = 6;
    /// the numeric code being used for Sunday.
    pub const SUN_NUM: u8 = 7;

    /// the short name of Monday in English.
    pub const MON: &str = "Mon";
    /// the short name of Tuesday in English.
    pub const TUE: &str = "Tue";
    /// the short name of Wednesday in English.
    pub const WED: &str = "Wed";
    /// the short name of Thursday in English.
    pub const THU: &str = "Thu";
    /// the short name of Friday in English.
    pub const FRI: &str = "Fri";
    /// the short name of Saturday in English.
    pub const SAT: &str = "Sat";
    /// the short name of Sunday in English.
    pub const SUN: &str = "Sun";

    /// Returns a day-of-week number starting from Monday = 0.
    #[must_use]
    pub fn num_days_from_monday(self) -> u8 {
        self as u8 - 1
    }

    /// Returns a day-of-week number starting from Monday = 1.
    #[must_use]
    pub fn number_from_monday(self) -> u8 {
        self as u8
    }

    /// Returns a day-of-week number starting from Sunday = 0.
    #[must_use]
    pub fn num_days_from_sunday(self) -> u8 {
        // avoid modulo operation
        if self == Self::Sun { 0 } else { self as u8 }
    }

    /// Returns a day-of-week number starting from Sunday = 1.
    #[must_use]
    pub fn number_from_sunday(self) -> u8 {
        // avoid modulo operation
        if self == Self::Sun { 1 } else { self as u8 + 1 }
    }

    /// Return the weekday's short name in English.
    ///
    /// The returned string is guaranteed to be 3 characters long.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Mon => Self::MON,
            Self::Tue => Self::TUE,
            Self::Wed => Self::WED,
            Self::Thu => Self::THU,
            Self::Fri => Self::FRI,
            Self::Sat => Self::SAT,
            Self::Sun => Self::SUN,
        }
    }
}

impl From<u8> for Weekday {
    fn from(value: u8) -> Self {
        // cheaply maps the value to the corresponding weekday
        // Sunday can be obtained from 0 or 7
        match value {
            Self::MON_NUM => Self::Mon,
            Self::TUE_NUM => Self::Tue,
            Self::WED_NUM => Self::Wed,
            Self::THU_NUM => Self::Thu,
            Self::FRI_NUM => Self::Fri,
            Self::SAT_NUM => Self::Sat,
            _ => Self::Sun,
        }
    }
}

impl AsRef<str> for Weekday {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Debug for Weekday {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}

impl fmt::Display for Weekday {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for Weekday {
    fn format(&self, fmt: defmt::Formatter<'_>) {
        defmt::write!(fmt, "{}", self.as_str());
    }
}

#[cfg(test)]
mod tests {
    #![expect(clippy::panic, clippy::unwrap_used, reason = "this is a test")]

    use chrono::Datelike as _;

    #[test]
    fn test_weekday() {
        // iterate through all valid dates and test whether `chrono` and `embedded-date-time` agree
        // on the weekday calculation.
        for year in 2000_u16..=2171 {
            'next_month: for month in 1_u8..=12 {
                for day in 1_u8..=31 {
                    let Some(embedded_date) = crate::Date::new_checked(year, month, day) else {
                        continue 'next_month;
                    };
                    let chrono_date = chrono::NaiveDate::from_ymd_opt(
                        i32::from(year),
                        u32::from(month),
                        u32::from(day),
                    )
                    .unwrap();

                    let chrono_weekday = chrono_date.weekday();
                    let embedded_weekday = embedded_date.weekday();

                    match (chrono_weekday, embedded_weekday) {
                        (chrono::Weekday::Mon, super::Weekday::Mon)
                        | (chrono::Weekday::Tue, super::Weekday::Tue)
                        | (chrono::Weekday::Wed, super::Weekday::Wed)
                        | (chrono::Weekday::Thu, super::Weekday::Thu)
                        | (chrono::Weekday::Fri, super::Weekday::Fri)
                        | (chrono::Weekday::Sat, super::Weekday::Sat)
                        | (chrono::Weekday::Sun, super::Weekday::Sun) => (),
                        _ => panic!(
                            "chrono and embedded weekdays do not match: {chrono_weekday:?} != {embedded_weekday:?} for {chrono_date}"
                        ),
                    }

                    assert_eq!(
                        chrono_weekday.num_days_from_monday(),
                        u32::from(embedded_weekday.num_days_from_monday()),
                        "num_days_from_monday: {chrono_date}",
                    );

                    assert_eq!(
                        chrono_weekday.num_days_from_sunday(),
                        u32::from(embedded_weekday.num_days_from_sunday()),
                        "num_days_from_sunday: {chrono_date}",
                    );

                    assert_eq!(
                        chrono_weekday.number_from_monday(),
                        u32::from(embedded_weekday.number_from_monday()),
                        "number_from_monday: {chrono_date}",
                    );

                    assert_eq!(
                        chrono_weekday.number_from_sunday(),
                        u32::from(embedded_weekday.number_from_sunday()),
                        "number_from_sunday: {chrono_date}",
                    );
                }
            }
        }
    }
}

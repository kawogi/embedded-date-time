use core::fmt;

/// A simple time of a day.
#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Clone)]
pub struct Time {
    /// Hour of the day [0-23].
    pub hour: u8,

    /// Minute of the hour [0-59].
    pub minute: u8,

    /// Second of the minute [0-60].
    ///
    /// A value of 60 might occur on a leap second.
    pub second: u8,
}

impl Time {
    /// Create a new Time from hours, minutes and seconds.
    ///
    /// No checks will be performed to validate the time.
    #[must_use]
    pub fn new(hour: u8, minute: u8, second: u8) -> Self {
        Self {
            hour,
            minute,
            second,
        }
    }

    /// Create a new Time from hours, minutes and seconds.
    ///
    /// Returns None if the hour, minute or second is invalid.
    ///
    /// Seconds are permitted to be 60 on a leap second.
    #[must_use]
    pub fn new_checked(hour: u8, minute: u8, second: u8) -> Option<Self> {
        let result = Self::new(hour, minute, second);
        result.is_valid().then_some(result)
    }

    /// Determine whether the time is a valid combination of hours, minutes and seconds.
    ///
    /// Seconds are permitted to be 60 on a leap second.
    #[must_use]
    pub fn is_valid(self) -> bool {
        self.hour <= 23 && self.minute <= 59 && self.second <= 60
    }
}

impl fmt::Debug for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}:{:02}", self.hour, self.minute, self.second)
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for Time {
    fn format(&self, fmt: defmt::Formatter<'_>) {
        defmt::write!(
            fmt,
            "{:02}:{:02}:{:02}",
            self.hour,
            self.minute,
            self.second
        );
    }
}

#[cfg(feature = "ufmt")]
impl ufmt::uDebug for Time {
    fn fmt<W>(&self, fmt: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt::uWrite + ?Sized,
    {
        ufmt::uDisplay::fmt(&self, fmt)
    }
}

#[cfg(feature = "ufmt")]
impl ufmt::uDisplay for Time {
    fn fmt<W>(&self, fmt: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt::uWrite + ?Sized,
    {
        use ufmt::uwrite;

        let Self {
            hour,
            minute,
            second,
        } = *self;

        if hour < 10 {
            uwrite!(fmt, "0{}:", hour)?;
        } else {
            uwrite!(fmt, "{}:", hour)?;
        }

        if minute < 10 {
            uwrite!(fmt, "0{}:", minute)?;
        } else {
            uwrite!(fmt, "{}:", minute)?;
        }

        if second < 10 {
            uwrite!(fmt, "0{}", second)?;
        } else {
            uwrite!(fmt, "{}", second)?;
        }

        Ok(())
    }
}

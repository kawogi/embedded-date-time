//! Fundamental primitives to communicate date and time information.
//!
//! The focus of this implementation is to be lightweight and easy to use
//! so that it can be used on resource-constrained embedded systems.
//! Only basic checks will be performed for the reasonable use-cases.

#![no_std]

mod date;
mod date_time;
mod time;
mod weekday;

pub use date::Date;
pub use date_time::DateTime;
pub use time::Time;
pub use weekday::Weekday;

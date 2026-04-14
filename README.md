# embedded-date-time

The purpose of this rust crate is to provide a basic set of data types to store and communicate date and time information on embedded devices with very limited capacity.

It is meant to be a replacement for the [`chrono`](https://github.com/chronotope/chrono)-crate which is rather heavyweight even when all default features are being disabled.
The implementation avoids unnecessarily large data types, checks for exotic exceptions and computationally intensive algorithms.

This comes with a few limitations:

- no sub-second time resolution
- limited support for years beyond 2000-2171 (see individual method documentation for details)
- no time zones and no daylight-saving times
- unspecified behavior for bogus dates and time
- some formatting decisions might be considered opinionated

## Features

This crate has support for [`defmt`](https://github.com/knurling-rs/defmt) and [`ufmt`](https://github.com/japaric/ufmt) which can be enabled by the features with the respective name.

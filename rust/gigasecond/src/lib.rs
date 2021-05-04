//! Given a moment, determine the moment that would be after a gigasecond has passed.
//! 
//! A gigasecond is 10^9 (1,000,000,000) seconds.
//! 
//! If you're unsure what operations you can perform on DateTime<Utc> take a look at the chrono crate which is listed as a dependency in the Cargo.toml file for this exercise.

use chrono::{DateTime, Utc, Duration};

/// Computes a new Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    start + Duration::seconds(1_000_000_000)
}

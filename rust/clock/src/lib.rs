//! Implement a clock that handles times without dates.
//! 
//! You should be able to add and subtract minutes to it.
//! 
//! Two clocks that represent the same time should be equal to each other.
//! 
//! # Traits
//! Rust Traits for .to_string()
//! Did you implement .to_string() for the Clock struct?
//! 
//! If so, try implementing the Display trait for Clock instead.
//! 
//! Traits allow for a common way to implement functionality for various types.
//! 
//! For additional learning, consider how you might implement String::from for the Clock type. You don't have to actually implement this—it's redundant with Display, which is generally the better choice when the destination type is String—but it's useful to have a few type-conversion traits in your toolkit.

use std::fmt;

/// contains a representation of the time of the day
#[derive(Debug, PartialEq)]
pub struct Clock {
    minutes: i32,
}

impl Clock {
    /// constructor for the Clock struct
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock{minutes: ((60*hours+minutes)%1440+1440)%1440}
    }

    /// add time in minutes
    pub fn add_minutes(&self, minutes: i32) -> Self {
        println!("{}",self);
        Clock{minutes: ((self.minutes+minutes)%1440+1440)%1440}
    }

    /// a stringer method for the Clock struct
    pub fn to_string(&self) -> String {
        let mut hours = format!("{}", self.minutes / 60);
        if hours.len() < 2 {
            hours = format!("0{}", hours);
        }
        let mut minutes = format!("{}", self.minutes % 60);
        if minutes.len() < 2 {
            minutes = format!("0{}", minutes);
        }
        format!("{}:{}", hours, minutes)
    }
}

impl fmt::Display for Clock {
    /// implementation of the display trait
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut hours = format!("{}", self.minutes / 60);
        if hours.len() < 2 {
            hours = format!("0{}", hours);
        }
        let mut minutes = format!("{}", self.minutes % 60);
        if minutes.len() < 2 {
            minutes = format!("0{}", minutes);
        }
        write!(f, "{}:{}", hours, minutes)
    }
}
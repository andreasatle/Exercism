use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock{minutes: ((60*hours+minutes)%1440+1440)%1440}
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        println!("{}",self);
        Clock{minutes: ((self.minutes+minutes)%1440+1440)%1440}
    }

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
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock{
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes = hours * 60 + minutes;
        let total_minutes = ((total_minutes % (24 * 60)) + (24 * 60)) % (24 * 60);
        let hours = total_minutes / 60;
        let minutes = total_minutes % 60;
        Self { hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let total = self.hours * 60 + self.minutes + minutes;
        Clock::new(0, total)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:02}:{:02}",
            self.hours,
            self.minutes
        )
    }
}

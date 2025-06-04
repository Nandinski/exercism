use std::fmt;

const MINUTES_IN_HOUR: i32 = 60;
const HOURS_IN_DAY: i32 = 24;
const MINUTES_IN_DAY: i32 = MINUTES_IN_HOUR * HOURS_IN_DAY;

#[derive(PartialEq, Debug)]
pub struct Clock {
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes = ((hours * MINUTES_IN_HOUR) + minutes).rem_euclid(MINUTES_IN_DAY);

        Self {
            minutes: total_minutes,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(0, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:02}:{:02}",
            self.minutes / MINUTES_IN_HOUR,
            self.minutes % MINUTES_IN_HOUR
        )
    }
}

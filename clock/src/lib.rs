use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    hour: u8,
    minute: u8,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let (hours, minutes) = Self::normalize(hours, minutes);
        Clock {
            hour: hours,
            minute: minutes,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let (hours, minutes) = Self::normalize(self.hour as i32, minutes + self.minute as i32);
        Clock {
            hour: hours,
            minute: minutes,
        }
    }

    fn normalize(hours: i32, minutes: i32) -> (u8, u8) {
        let mut minute = minutes % 60;
        let mut hour_adjustment = minutes / 60;
        if minute < 0 {
            hour_adjustment -= 1;
            minute += 60;
        }

        let mut hour = (hours + hour_adjustment) % 24;
        if hour < 0 {
            hour += 24;
        }

        (hour as u8, minute as u8)
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hour, self.minute)
    }
}

use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: u8,
    minutes: u8,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut hrs = hours + minutes / 60;

        if hrs < 0 {
            hrs = 24 + (hrs % 24);
        }

        let mut min = minutes % 60;
        if min < 0 {
            if hrs == 0 {
                hrs = 24;
            }
            hrs -= 1;
            min += 60;
        }

        hrs %= 24;

        Self {
            hours: hrs as u8,
            minutes: min as u8,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let minutes_sum = self.minutes as i32 + minutes;
        Clock::new(self.hours as i32, minutes_sum)
    }

    fn pad_start(num: u8) -> String {
        if num < 10 {
            format!("0{}", num)
        } else {
            num.to_string()
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{}",
            Self::pad_start(self.hours),
            Self::pad_start(self.minutes),
        )
    }
}

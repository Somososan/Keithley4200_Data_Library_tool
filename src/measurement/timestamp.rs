use crate::calamine_helper::MyRange;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Hash)]
pub struct TimeStamp {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

impl std::fmt::Display for TimeStamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{} {}:{}:{}",
            self.year, self.month, self.day, self.hour, self.minute, self.second
        )
    }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Date {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

const OFFSETS: (usize, usize) = (9, 1);

impl std::fmt::Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", self.year, self.month, self.day)
    }
}

impl From<TimeStamp> for Date {
    fn from(timestamp: TimeStamp) -> Self {
        Date {
            year: timestamp.year,
            month: timestamp.month,
            day: timestamp.day,
        }
    }
}

impl crate::Extract for TimeStamp {
    fn extract(sheet: &MyRange) -> Option<TimeStamp> {
        let raw_string: &str = sheet
            .it
            .get(OFFSETS)?
            .get_string()
            .expect("Time stamp extraction failure");
        let mut split_string = raw_string.split_ascii_whitespace();
        let mut date = split_string.next().unwrap().split('/');
        //keep this order for the Yankees
        let month: u8 = date
            .next()
            .unwrap_or("0")
            .parse()
            .expect("error parsing time stamp");
        let day: u8 = date
            .next()
            .unwrap_or("0")
            .parse()
            .expect("error parsing time stamp");
        let year: u16 = date
            .next()
            .unwrap_or("0")
            .parse()
            .expect("error parsing time stamp");

        let mut time = split_string.next().unwrap().split(':');
        let hour: u8 = time
            .next()
            .unwrap_or("0")
            .parse()
            .expect("error parsing time stamp");
        let minute: u8 = time
            .next()
            .unwrap_or("0")
            .parse()
            .expect("error parsing time stamp");
        let second: u8 = time
            .next()
            .unwrap_or("0")
            .parse()
            .expect("error parsing time stamp");
        Some(TimeStamp {
            year,
            month,
            day,
            hour,
            minute,
            second,
        })
    }
}

use chrono::{Datelike as _, Timelike as _, Utc};

use crate::maps::Maps;

#[derive(Debug)]
pub struct SystemTime {
    year: u16,
    month: u16,
    day_of_week: u16,
    day: u16,
    hour: u16,
    minute: u16,
    second: u16,
    millis: u16,
}

impl SystemTime {
    pub fn now() -> SystemTime {
        let now = Utc::now();
        

        SystemTime {
            year: now.year() as u16,
            month: now.month() as u16,
            day_of_week: now.weekday() as u16,
            day: now.day() as u16,
            hour: now.hour() as u16,
            minute: now.minute() as u16,
            second: now.second() as u16,
            millis: now.timestamp_millis() as u16,
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_word(addr, self.year);
        maps.write_word(addr + 2, self.month);
        maps.write_word(addr + 4, self.day_of_week);
        maps.write_word(addr + 6, self.day);
        maps.write_word(addr + 8, self.hour);
        maps.write_word(addr + 10, self.minute);
        maps.write_word(addr + 12, self.second);
        maps.write_word(addr + 14, self.millis);
    }
}

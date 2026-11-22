use chrono::{DateTime, Datelike, NaiveDate, NaiveDateTime, Timelike, Utc};

use crate::maps::Maps;
use crate::windows::constants::time::*;

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

fn file_time_epoch() -> Option<NaiveDateTime> {
    NaiveDate::from_ymd_opt(FILETIME_YEAR_START, 1, 1)?.and_hms_opt(0, 0, 0)
}

impl SystemTime {
    pub fn now() -> Self {
        Self::from_datetime(Utc::now().naive_utc())
    }

    pub(crate) fn from_datetime(date: NaiveDateTime) -> Self {
        Self {
            year: date.year() as u16,
            month: date.month() as u16,
            day_of_week: date.weekday().num_days_from_sunday() as u16,
            day: date.day() as u16,
            hour: date.hour() as u16,
            minute: date.minute() as u16,
            second: date.second() as u16,
            millis: (date.nanosecond() / NANOSECONDS_PER_MILLISECOND) as u16,
        }
    }

    pub(crate) fn from_bytes(bytes: [u8; SYSTEMTIME_BYTES]) -> Self {
        let fields: [u16; SYSTEMTIME_FIELDS] = std::array::from_fn(|index| {
            let offset = index * size_of::<u16>();
            u16::from_le_bytes([bytes[offset], bytes[offset + 1]])
        });
        let [year, month, day_of_week, day, hour, minute, second, millis] = fields;
        Self {
            year,
            month,
            day_of_week,
            day,
            hour,
            minute,
            second,
            millis,
        }
    }

    pub(crate) fn to_bytes(&self) -> [u8; SYSTEMTIME_BYTES] {
        let fields = [
            self.year,
            self.month,
            self.day_of_week,
            self.day,
            self.hour,
            self.minute,
            self.second,
            self.millis,
        ];
        let mut bytes = [0; SYSTEMTIME_BYTES];
        for (output, value) in bytes.chunks_exact_mut(size_of::<u16>()).zip(fields) {
            output.copy_from_slice(&value.to_le_bytes());
        }
        bytes
    }

    pub(crate) fn datetime(&self) -> Option<NaiveDateTime> {
        if self.millis >= MILLISECONDS_PER_SECOND {
            return None;
        }
        NaiveDate::from_ymd_opt(self.year as i32, self.month as u32, self.day as u32)?
            .and_hms_milli_opt(
                self.hour as u32,
                self.minute as u32,
                self.second as u32,
                self.millis as u32,
            )
    }

    pub(crate) fn from_file_time(value: u64) -> Option<Self> {
        if value > i64::MAX as u64 {
            return None;
        }
        let seconds = (value / FILETIME_TICKS_PER_SECOND) as i64;
        let nanos = (value % FILETIME_TICKS_PER_SECOND) as u32 * NANOSECONDS_PER_FILETIME_TICK;
        let epoch = file_time_epoch()?.and_utc().timestamp();
        let date = DateTime::from_timestamp(epoch.checked_add(seconds)?, nanos)?.naive_utc();
        Some(Self::from_datetime(date))
    }

    pub(crate) fn file_time(&self) -> Option<u64> {
        // SYSTEMTIME input is capped below the final partial FILETIME year.
        if !(FILETIME_YEAR_START..=SYSTEMTIME_YEAR_END).contains(&(self.year as i32)) {
            return None;
        }
        let duration = self.datetime()?.signed_duration_since(file_time_epoch()?);
        (duration.num_seconds() as u64)
            .checked_mul(FILETIME_TICKS_PER_SECOND)?
            .checked_add(self.millis as u64 * FILETIME_TICKS_PER_MILLISECOND)
    }

    pub fn save(&self, address: u64, maps: &mut Maps) {
        maps.write_bytes(address, &self.to_bytes());
    }
}

use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

pub struct WIMCInput {}

pub struct WIMCOutput {}

pub struct Date {
    day: u8,
    month: u8,
    year: u16,
    hour: u8,
    minute: u8,
    second: u8,
}

impl Date {}

impl Display for Date {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        //"2004-06-14T23:34:30"
        write!(f, "{}-{}-{}T{}:{}:{}", self.year, self.month, self.day, self.hour, self.minute, self.second)
    }
}

impl Debug for Date {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

#[cfg(test)]
mod tests {
    use std::time::SystemTime;

    use crate::models::get_date_time;

    #[test]
    pub fn date_now() {
        let dur = SystemTime::UNIX_EPOCH.elapsed().map(|va| va.as_secs()).unwrap();
        println!("{:?}", get_date_time(dur as i32));
    }
}

const SECONDS_PER_MINUTE: i32 = 60;
const SECONDS_PER_HOUR: i32 = 60 * SECONDS_PER_MINUTE;
const SECONDS_PER_DAY: i32 = 24 * SECONDS_PER_HOUR;
const DAYS_PER_YEAR: i32 = 365;
const DAYS_PER_LEAP_YEAR: i32 = DAYS_PER_YEAR + 1;
const EPOCH_MONTH: i32 = 1;
const EPOCH_YEAR: i32 = 1970;

fn get_date_time(timestamp: i32) -> Vec<(String, i32)> {
    let mut days = timestamp / SECONDS_PER_DAY;
    let mut year = EPOCH_YEAR;
    while days >= get_days_for_year(year) {
        days -= get_days_for_year(year);
        year += 1;
    }
    let days_per_month = get_days_per_month(year);
    let mut month = EPOCH_MONTH;
    while days >= days_per_month[month as usize] {
        days -= days_per_month[month as usize];
        month += 1;
    }
    let day = days + 1;
    let seconds_remaining = timestamp % SECONDS_PER_DAY;
    let hour = seconds_remaining / SECONDS_PER_HOUR;
    let minute = (seconds_remaining / SECONDS_PER_MINUTE) % SECONDS_PER_MINUTE;
    let second = seconds_remaining % SECONDS_PER_MINUTE;
    vec![
        ("year".to_string(), year),
        ("month".to_string(), month),
        ("day".to_string(), day),
        ("hour".to_string(), hour + 1),
        ("minute".to_string(), minute),
        ("second".to_string(), second),
    ]
}

fn is_leap_year(year: i32) -> bool {
    year % 400 == 0 || (year % 4 == 0 && year % 100 != 0)
}

fn get_days_for_year(year: i32) -> i32 {
    if is_leap_year(year) {
        DAYS_PER_LEAP_YEAR
    } else {
        DAYS_PER_YEAR
    }
}

fn get_days_per_month(year: i32) -> Vec<i32> {
    vec![
        0,
        31,
        if is_leap_year(year) { 29 } else { 28 },
        31,
        30,
        31,
        30,
        31,
        31,
        30,
        31,
        30,
        31,
    ]
}

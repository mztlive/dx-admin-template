use std::fmt::{self, Display, Formatter, Write};
use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::time::{SystemTime, UNIX_EPOCH};

const SECONDS_PER_DAY: i64 = 86_400;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NaiveDate {
    days_since_epoch: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Duration {
    days: i64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Weekday {
    Monday = 0,
    Tuesday = 1,
    Wednesday = 2,
    Thursday = 3,
    Friday = 4,
    Saturday = 5,
    Sunday = 6,
}

impl Weekday {
    pub fn num_days_from_monday(self) -> u8 {
        self as u8
    }
}

impl Duration {
    pub fn days(days: i64) -> Self {
        Self { days }
    }

    pub fn num_days(self) -> i64 {
        self.days
    }
}

#[derive(Clone, Copy)]
pub struct FormattedDate<'a> {
    date: NaiveDate,
    pattern: &'a str,
}

impl<'a> Display for FormattedDate<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&self.date.format_internal(self.pattern))
    }
}

impl NaiveDate {
    pub fn from_ymd_opt(year: i32, month: u32, day: u32) -> Option<Self> {
        if month == 0 || month > 12 {
            return None;
        }
        if day == 0 || day > days_in_month(year, month) {
            return None;
        }
        Some(Self {
            days_since_epoch: days_from_civil(year, month, day)?,
        })
    }

    pub fn with_day(self, day: u32) -> Option<Self> {
        Self::from_ymd_opt(self.year(), self.month(), day)
    }

    pub fn year(self) -> i32 {
        self.components().0
    }

    pub fn month(self) -> u32 {
        self.components().1
    }

    pub fn day(self) -> u32 {
        self.components().2
    }

    pub fn weekday(self) -> Weekday {
        let idx = (self.days_since_epoch as i64 + 3).rem_euclid(7) as u8;
        match idx {
            0 => Weekday::Monday,
            1 => Weekday::Tuesday,
            2 => Weekday::Wednesday,
            3 => Weekday::Thursday,
            4 => Weekday::Friday,
            5 => Weekday::Saturday,
            _ => Weekday::Sunday,
        }
    }

    pub fn format<'a>(self, pattern: &'a str) -> FormattedDate<'a> {
        FormattedDate { date: self, pattern }
    }

    pub fn from_system_time(time: SystemTime) -> Option<Self> {
        match time.duration_since(UNIX_EPOCH) {
            Ok(duration) => {
                let days = (duration.as_secs() as i64) / SECONDS_PER_DAY;
                Self::from_days_since_epoch(days)
            }
            Err(err) => {
                let duration = err.duration();
                let days = (duration.as_secs() as i64 + SECONDS_PER_DAY - 1) / SECONDS_PER_DAY;
                Self::from_days_since_epoch(-days)
            }
        }
    }

    pub fn from_days_since_epoch(days: i64) -> Option<Self> {
        let value: i32 = days.try_into().ok()?;
        Some(Self {
            days_since_epoch: value,
        })
    }

    fn components(self) -> (i32, u32, u32) {
        civil_from_days(self.days_since_epoch)
    }

    fn format_internal(self, pattern: &str) -> String {
        let (year, month, day) = self.components();
        let mut output = String::with_capacity(pattern.len() + 8);
        let mut chars = pattern.chars();
        while let Some(ch) = chars.next() {
            if ch == '%' {
                if let Some(spec) = chars.next() {
                    match spec {
                        'Y' => {
                            let _ = write!(output, "{year:04}");
                        }
                        'm' => {
                            let _ = write!(output, "{month:02}");
                        }
                        'd' => {
                            let _ = write!(output, "{day:02}");
                        }
                        'B' => output.push_str(full_month(month)),
                        'b' => output.push_str(short_month(month)),
                        '%' => output.push('%'),
                        other => {
                            output.push('%');
                            output.push(other);
                        }
                    }
                }
            } else {
                output.push(ch);
            }
        }
        output
    }
}

impl Add<Duration> for NaiveDate {
    type Output = Self;

    fn add(self, rhs: Duration) -> Self::Output {
        let total = self.days_since_epoch as i64 + rhs.days;
        Self::from_days_since_epoch(total).expect("date overflow")
    }
}

impl AddAssign<Duration> for NaiveDate {
    fn add_assign(&mut self, rhs: Duration) {
        *self = *self + rhs;
    }
}

impl Sub<Duration> for NaiveDate {
    type Output = Self;

    fn sub(self, rhs: Duration) -> Self::Output {
        let total = self.days_since_epoch as i64 - rhs.days;
        Self::from_days_since_epoch(total).expect("date overflow")
    }
}

impl SubAssign<Duration> for NaiveDate {
    fn sub_assign(&mut self, rhs: Duration) {
        *self = *self - rhs;
    }
}

impl Sub for NaiveDate {
    type Output = Duration;

    fn sub(self, rhs: NaiveDate) -> Self::Output {
        Duration::days(self.days_since_epoch as i64 - rhs.days_since_epoch as i64)
    }
}

fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => 0,
    }
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

fn days_from_civil(year: i32, month: u32, day: u32) -> Option<i32> {
    let y = year - if month <= 2 { 1 } else { 0 };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let mp = month as i32 + if month > 2 { -3 } else { 9 };
    let doy = (153 * mp + 2) / 5 + day as i32 - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    let days = era * 146097 + doe - 719_468;
    days.try_into().ok()
}

fn civil_from_days(days: i32) -> (i32, u32, u32) {
    let days = days as i64 + 719_468;
    let era = if days >= 0 {
        days / 146_097
    } else {
        (days - 146_096) / 146_097
    };
    let doe = days - era * 146_097;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let mut y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = mp + if mp < 10 { 3 } else { -9 };
    y += if month <= 2 { 1 } else { 0 };
    (y as i32, month as u32, day as u32)
}

fn full_month(month: u32) -> &'static str {
    match month {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => "Unknown",
    }
}

fn short_month(month: u32) -> &'static str {
    match month {
        1 => "Jan",
        2 => "Feb",
        3 => "Mar",
        4 => "Apr",
        5 => "May",
        6 => "Jun",
        7 => "Jul",
        8 => "Aug",
        9 => "Sep",
        10 => "Oct",
        11 => "Nov",
        12 => "Dec",
        _ => "Unk",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_ymd() {
        let date = NaiveDate::from_ymd_opt(2024, 6, 30).unwrap();
        assert_eq!(date.year(), 2024);
        assert_eq!(date.month(), 6);
        assert_eq!(date.day(), 30);
        assert_eq!(date.weekday().num_days_from_monday(), 6);
    }

    #[test]
    fn add_days() {
        let mut date = NaiveDate::from_ymd_opt(2024, 1, 31).unwrap();
        date += Duration::days(1);
        assert_eq!((date.year(), date.month(), date.day()), (2024, 2, 1));
    }
}

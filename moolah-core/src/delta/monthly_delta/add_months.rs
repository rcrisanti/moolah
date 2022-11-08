use crate::errors::MoolahCoreError;
use chrono::{Datelike, Duration, NaiveDate};
use std::ops::{Add, Mul};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MonthDay {
    day: u8,
}

impl Default for MonthDay {
    fn default() -> Self {
        MonthDay { day: 1 }
    }
}

impl TryFrom<u8> for MonthDay {
    type Error = MoolahCoreError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value > 31 || value < 1 {
            return Err(MoolahCoreError::MonthDayOutOfRange(value.into()));
        }
        Ok(MonthDay { day: value })
    }
}

impl TryFrom<u32> for MonthDay {
    type Error = MoolahCoreError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value > 31 || value < 1 {
            return Err(MoolahCoreError::MonthDayOutOfRange(value));
        }
        Ok(MonthDay {
            day: value
                .try_into()
                .expect("This should never panic bc you already checked the value"),
        })
    }
}

impl Into<u32> for MonthDay {
    fn into(self) -> u32 {
        self.day.into()
    }
}

impl Into<u32> for &MonthDay {
    fn into(self) -> u32 {
        self.day.into()
    }
}

impl Mul<u32> for MonthDay {
    type Output = MultiMonthDuration;

    fn mul(self, rhs: u32) -> Self::Output {
        MultiMonthDuration { n_months: rhs }
    }
}

pub struct MultiMonthDuration {
    n_months: u32,
}

impl Add<NaiveDate> for MultiMonthDuration {
    type Output = NaiveDate;

    fn add(self, rhs: NaiveDate) -> Self::Output {
        let mut month = MonthAdded::Exact(rhs);
        for _ in 0..self.n_months {
            month = add_month(&month, 0);
        }
        match month {
            MonthAdded::Exact(date) => date,
            MonthAdded::Rounded(date, _) => date,
        }
    }
}

enum MonthAdded {
    Exact(NaiveDate),
    Rounded(NaiveDate, MonthDay),
}

const MAX_RECURSION: usize = 5;
fn add_month(date: &MonthAdded, recursion_level: usize) -> MonthAdded {
    if recursion_level >= MAX_RECURSION {
        panic!("Reached max recursion level - this most likely means an impossible date was attempted to be created");
    }

    let (date, target_day) = match date {
        MonthAdded::Exact(date) => (
            date,
            date.day().try_into().expect("this should never throw"),
        ),
        MonthAdded::Rounded(date, month_day) => (date, *month_day),
    };

    let month = match date.month() {
        12 => 1,
        _ => date.month() + 1,
    };
    let year = match month {
        1 => date.year() + 1,
        _ => date.year(),
    };
    let day = match recursion_level {
        0 => target_day.into(),
        _ => date.day(),
    };

    match NaiveDate::from_ymd_opt(year, month, day) {
        Some(date) => {
            if date.day() == target_day.into() {
                MonthAdded::Exact(date)
            } else {
                MonthAdded::Rounded(date, target_day)
            }
        }
        None => add_month(
            &MonthAdded::Rounded(*date - Duration::days(1), target_day),
            recursion_level + 1,
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mult_month_day() {
        let md: MonthDay = 17u32.try_into().unwrap();
        assert_eq!((md * 7).n_months, 7);
    }

    #[test]
    fn test_add_multi_months() {
        let d = NaiveDate::from_ymd(2022, 10, 31);

        assert_eq!(
            MultiMonthDuration { n_months: 1 } + d,
            NaiveDate::from_ymd(2022, 11, 30)
        );
        assert_eq!(
            MultiMonthDuration { n_months: 2 } + d,
            NaiveDate::from_ymd(2022, 12, 31)
        );
        assert_eq!(
            MultiMonthDuration { n_months: 3 } + d,
            NaiveDate::from_ymd(2023, 1, 31)
        );
        assert_eq!(
            MultiMonthDuration { n_months: 4 } + d,
            NaiveDate::from_ymd(2023, 2, 28)
        );
        assert_eq!(
            MultiMonthDuration { n_months: 5 } + d,
            NaiveDate::from_ymd(2023, 3, 31)
        );
        assert_eq!(
            MultiMonthDuration { n_months: 16 } + d,
            NaiveDate::from_ymd(2024, 2, 29) // leap year
        );
    }

    #[test]
    #[should_panic]
    fn test_panic_add_month_over_max_recursion() {
        let _ = MultiMonthDuration { n_months: 1 } + NaiveDate::from_ymd(400000, 10, 30);
    }
}

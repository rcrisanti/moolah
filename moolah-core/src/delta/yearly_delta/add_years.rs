use chrono::{Datelike, NaiveDate};

use crate::{date_helpers::naive_ymd, errors::MoolahCoreError};

#[derive(Clone, Copy)]
pub struct MultiYearDuration {
    n_years: u32,
}

impl MultiYearDuration {
    pub fn new(n_years: u32) -> Self {
        MultiYearDuration { n_years }
    }

    pub fn try_add(self, rhs: NaiveDate) -> Result<NaiveDate, MoolahCoreError> {
        let mut date = YearAdded::Exact(rhs);
        for _ in 0..self.n_years {
            date = add_year(&date, 0)?;
        }
        Ok(match date {
            YearAdded::Exact(date) => date,
            YearAdded::Rounded {
                date,
                target_day: _,
            } => date,
        })
    }
}

enum YearAdded {
    Exact(NaiveDate),
    Rounded { date: NaiveDate, target_day: u32 },
}

const MAX_RECURSION: usize = 2;
fn add_year(year_added: &YearAdded, recursion_level: usize) -> Result<YearAdded, MoolahCoreError> {
    if recursion_level >= MAX_RECURSION {
        panic!("Reached max recursion level - this most likely means an impossible date was attempted to be created");
    }

    let (date, target_day) = match year_added {
        YearAdded::Exact(date) => (date, date.day()),
        YearAdded::Rounded { date, target_day } => (date, *target_day),
    };

    let day = if recursion_level == 0 {
        target_day
    } else {
        date.day()
    };

    if (date.month() == 2) & (day == 29) {
        // handle leap years
        match NaiveDate::from_ymd_opt(date.year() + 1, date.month(), day) {
            Some(date) => Ok(YearAdded::Exact(date)),
            None => add_year(
                &YearAdded::Rounded {
                    date: naive_ymd(date.year(), date.month(), day - 1)?,
                    target_day,
                },
                recursion_level + 1,
            ),
        }
    } else {
        let date = naive_ymd(date.year() + 1, date.month(), date.day())?;
        match year_added {
            YearAdded::Exact(_) => Ok(YearAdded::Exact(date)),
            YearAdded::Rounded {
                date: _,
                target_day: _,
            } => Ok(YearAdded::Rounded { date, target_day }),
        }
    }
}

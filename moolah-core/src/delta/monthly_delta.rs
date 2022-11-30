pub mod add_months;

use super::{reasonable_bounds, Delta, Uncertainty};
use crate::{date_helpers::naive_ymd, errors::MoolahCoreError};
pub use add_months::MonthDay;
use chrono::{Datelike, Local, NaiveDate};

pub struct MonthlyDelta {
    name: String,
    value: f64,
    uncertainty: Option<Uncertainty>,
    start: NaiveDate,
    end: NaiveDate,
    on_month_day: MonthDay,
    skip_months: u16,
    dates: Vec<NaiveDate>,
}

impl Default for MonthlyDelta {
    fn default() -> Self {
        let today = Local::now().date_naive();

        MonthlyDelta {
            name: Default::default(),
            value: Default::default(),
            uncertainty: Default::default(),
            start: today,
            end: today,
            on_month_day: Default::default(),
            skip_months: Default::default(),
            dates: vec![today],
        }
    }
}

fn build_dates(
    start: &NaiveDate,
    end: &NaiveDate,
    on_month_day: &MonthDay,
    every_months: u32,
) -> Result<Vec<NaiveDate>, MoolahCoreError> {
    let month_day: u32 = on_month_day.into();
    let start = if start.day() > month_day {
        naive_ymd(start.year(), start.month() + 1, month_day)?
    } else {
        naive_ymd(start.year(), start.month(), month_day)?
    };

    let end = if end.day() >= on_month_day.into() {
        naive_ymd(end.year(), end.month(), month_day)?
    } else {
        naive_ymd(end.year(), end.month() - 1, month_day)?
    };

    let year_diff = (end.year() - start.year()) * 12;
    let month_diff: i32 = end.month() as i32 - start.month() as i32;
    let n_tot_months: i32 = year_diff + month_diff;

    if n_tot_months <= 0 {
        return Ok(vec![]);
    }
    let n_months: u32 = n_tot_months as u32 / every_months;

    let target_month_day = *on_month_day;
    Ok((0..=n_months)
        .map(|month_num| target_month_day * (month_num * every_months) + start)
        .collect())
}

impl MonthlyDelta {
    pub fn try_new(
        name: String,
        value: f64,
        uncertainty: Option<Uncertainty>,
        start: NaiveDate,
        end: NaiveDate,
        on_month_day: MonthDay,
        skip_months: u16,
    ) -> Result<Self, MoolahCoreError> {
        if let Some(Uncertainty::Bounds { low, high }) = uncertainty {
            reasonable_bounds(low, high, value)?;
        }

        if start > end {
            return Err(MoolahCoreError::StartAfterEnd { start, end });
        }

        Ok(MonthlyDelta {
            name,
            value,
            uncertainty,
            start,
            end,
            on_month_day,
            skip_months,
            dates: build_dates(&start, &end, &on_month_day, (skip_months + 1).into())?,
        })
    }

    pub fn start(&self) -> &NaiveDate {
        &self.start
    }

    pub fn end(&self) -> &NaiveDate {
        &self.end
    }

    pub fn on_month_day(&self) -> &MonthDay {
        &self.on_month_day
    }

    pub fn skip_months(&self) -> u16 {
        self.skip_months
    }
}

impl Delta for MonthlyDelta {
    fn name(&self) -> &str {
        &self.name
    }

    fn value(&self) -> f64 {
        self.value
    }

    fn uncertainty(&self) -> &Option<Uncertainty> {
        &self.uncertainty
    }

    fn dates(&self) -> &[NaiveDate] {
        &self.dates
    }
}

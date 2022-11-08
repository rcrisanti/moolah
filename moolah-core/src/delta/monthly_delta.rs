pub mod add_months;

use super::{reasonable_bounds, Delta, Uncertainty};
use crate::errors::MoolahCoreError;
pub use add_months::MonthDay;
use chrono::{Datelike, Local, NaiveDate};

pub struct MonthlyDelta {
    name: String,
    value: f32,
    uncertainty: Option<Uncertainty>,
    start: NaiveDate,
    end: NaiveDate,
    on_month_day: MonthDay,
    skip_months: u16,
    dates: Vec<NaiveDate>,
}

impl Default for MonthlyDelta {
    fn default() -> Self {
        let today = Local::today().naive_local();

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
) -> Vec<NaiveDate> {
    let month_day: u32 = on_month_day.into();
    let start = if start.day() > month_day {
        NaiveDate::from_ymd(start.year(), start.month() + 1, month_day)
    } else {
        NaiveDate::from_ymd(start.year(), start.month(), month_day)
    };

    let end = if end.day() >= on_month_day.into() {
        NaiveDate::from_ymd(end.year(), end.month(), month_day)
    } else {
        NaiveDate::from_ymd(end.year(), end.month() - 1, month_day)
    };

    let year_diff: u32 = ((end.year() - start.year()) * 12)
        .try_into()
        .expect("could not create u32 for gap_months");

    if start.month() > end.month() {
        return vec![];
    }

    let month_diff = end.month() - start.month();
    let n_months: u32 = (year_diff + month_diff) / every_months;

    let target_month_day = *on_month_day;
    (0..=n_months)
        .map(|month_num| target_month_day * (month_num * every_months) + start)
        .collect()
}

impl MonthlyDelta {
    pub fn try_new(
        name: String,
        value: f32,
        uncertainty: Option<Uncertainty>,
        start: NaiveDate,
        end: NaiveDate,
        on_month_day: u8,
        skip_months: u16,
    ) -> Result<Self, MoolahCoreError> {
        if let Some(Uncertainty::Bounds { low, high }) = uncertainty {
            reasonable_bounds(low, high, value)?;
        }

        if start > end {
            return Err(MoolahCoreError::StartAfterEnd { start, end });
        }

        let month_day: MonthDay = on_month_day.try_into()?;

        Ok(MonthlyDelta {
            name,
            value,
            uncertainty,
            start,
            end,
            on_month_day: month_day.clone(),
            skip_months,
            dates: build_dates(&start, &end, &month_day, (skip_months + 1).into()),
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

    fn value(&self) -> f32 {
        self.value
    }

    fn uncertainty(&self) -> &Option<Uncertainty> {
        &self.uncertainty
    }

    fn dates(&self) -> &[NaiveDate] {
        &self.dates
    }
}

use super::{reasonable_bounds, Delta, Uncertainty};
use crate::errors::MoolahCoreError;
use add_years::MultiYearDuration;
use chrono::{Datelike, Local, NaiveDate};

mod add_years;

pub struct YearlyDelta {
    name: String,
    value: f64,
    uncertainty: Option<Uncertainty>,
    start: NaiveDate,
    end: NaiveDate,
    skip_years: u16,
    dates: Vec<NaiveDate>,
}

impl Default for YearlyDelta {
    fn default() -> Self {
        let today = Local::now().date_naive();

        YearlyDelta {
            name: Default::default(),
            value: Default::default(),
            uncertainty: Default::default(),
            start: today,
            end: today,
            skip_years: Default::default(),
            dates: vec![today],
        }
    }
}

fn build_dates(
    start: &NaiveDate,
    end: &NaiveDate,
    every_years: u16,
) -> Result<Vec<NaiveDate>, MoolahCoreError> {
    let date_offset = if (end.month() < start.month())
        | ((end.month() == start.month()) & (end.day() < start.day()))
    {
        -1
    } else {
        0
    };
    let n_years = (end.year() - start.year() + date_offset) / i32::from(every_years);
    let n_years: u32 = n_years.try_into().unwrap_or_default();

    (0..=n_years)
        .map(|year_num| MultiYearDuration::new(every_years as u32 * year_num).try_add(*start))
        .collect()
}

impl YearlyDelta {
    pub fn try_new(
        name: String,
        value: f64,
        uncertainty: Option<Uncertainty>,
        start: NaiveDate,
        end: NaiveDate,
        skip_years: u16,
    ) -> Result<Self, MoolahCoreError> {
        if let Some(Uncertainty::Bounds { low, high }) = uncertainty {
            reasonable_bounds(low, high, value)?;
        }

        if start > end {
            return Err(MoolahCoreError::StartAfterEnd { start, end });
        }

        Ok(YearlyDelta {
            name,
            value,
            uncertainty,
            start,
            end,
            skip_years,
            dates: build_dates(&start, &end, skip_years + 1)?,
        })
    }

    pub fn start(&self) -> &NaiveDate {
        &self.start
    }

    pub fn end(&self) -> &NaiveDate {
        &self.end
    }

    pub fn skip_years(&self) -> u16 {
        self.skip_years
    }
}

impl Delta for YearlyDelta {
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

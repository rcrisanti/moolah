use super::{reasonable_bounds, Delta, Uncertainty};
use crate::errors::MoolahCoreError;
use chrono::{Duration, Local, NaiveDate};

pub struct DailyDelta {
    name: String,
    value: f64,
    uncertainty: Option<Uncertainty>,
    start: NaiveDate,
    end: NaiveDate,
    skip_days: u32,
    dates: Vec<NaiveDate>,
}

impl Default for DailyDelta {
    fn default() -> Self {
        let today = Local::today().naive_local();

        DailyDelta {
            name: Default::default(),
            value: Default::default(),
            uncertainty: Default::default(),
            start: today,
            end: today,
            skip_days: Default::default(),
            dates: vec![today],
        }
    }
}

fn build_dates(start: &NaiveDate, end: &NaiveDate, every_days: i64) -> Vec<NaiveDate> {
    let duration = Duration::days(every_days.into());
    let n_days: i32 = ((*end - *start).num_days() / every_days)
        .try_into()
        .expect("Too many days");
    (0..=n_days).map(|date| *start + duration * date).collect()
}

impl DailyDelta {
    pub fn try_new(
        name: String,
        value: f64,
        uncertainty: Option<Uncertainty>,
        start: NaiveDate,
        end: NaiveDate,
        skip_days: u32,
    ) -> Result<Self, MoolahCoreError> {
        if let Some(Uncertainty::Bounds { low, high }) = uncertainty {
            reasonable_bounds(low, high, value)?;
        }

        if start > end {
            return Err(MoolahCoreError::StartAfterEnd { start, end });
        }

        Ok(DailyDelta {
            name,
            value,
            uncertainty,
            start,
            end,
            skip_days,
            dates: build_dates(&start, &end, (skip_days + 1).into()),
        })
    }

    pub fn start(&self) -> &NaiveDate {
        &self.start
    }

    pub fn end(&self) -> &NaiveDate {
        &self.end
    }

    pub fn skip_days(&self) -> u32 {
        self.skip_days
    }
}

impl Delta for DailyDelta {
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

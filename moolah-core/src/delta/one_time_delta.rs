use super::{reasonable_bounds, Delta, Uncertainty};
use crate::errors::MoolahCoreError;
use chrono::{Local, NaiveDate};

pub struct OneTimeDelta {
    name: String,
    value: f64,
    uncertainty: Option<Uncertainty>,
    date: NaiveDate,
    dates: Vec<NaiveDate>,
}

impl Default for OneTimeDelta {
    fn default() -> Self {
        let today = Local::now().date_naive();
        OneTimeDelta {
            name: Default::default(),
            value: Default::default(),
            uncertainty: Default::default(),
            date: today,
            dates: vec![today],
        }
    }
}

impl OneTimeDelta {
    pub fn try_new(
        name: String,
        value: f64,
        uncertainty: Option<Uncertainty>,
        date: NaiveDate,
    ) -> Result<Self, MoolahCoreError> {
        if let Some(Uncertainty::Bounds { low, high }) = uncertainty {
            reasonable_bounds(low, high, value)?;
        }

        Ok(OneTimeDelta {
            name,
            value,
            uncertainty,
            date,
            dates: vec![date],
        })
    }

    pub fn date(&self) -> &NaiveDate {
        &self.date
    }
}

impl Delta for OneTimeDelta {
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

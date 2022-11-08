use super::{reasonable_bounds, Delta, Uncertainty};
use crate::errors::MoolahCoreError;
use chrono::NaiveDate;

#[derive(Default)]
pub struct CustomDelta {
    name: String,
    value: f32,
    uncertainty: Option<Uncertainty>,
    dates: Vec<NaiveDate>,
}

impl CustomDelta {
    pub fn try_new(
        name: String,
        value: f32,
        uncertainty: Option<Uncertainty>,
        dates: Vec<NaiveDate>,
    ) -> Result<Self, MoolahCoreError> {
        if let Some(Uncertainty::Bounds { low, high }) = uncertainty {
            reasonable_bounds(low, high, value)?;
        }

        Ok(CustomDelta {
            name,
            value,
            uncertainty,
            dates,
        })
    }
}

impl Delta for CustomDelta {
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

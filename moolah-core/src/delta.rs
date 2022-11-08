pub mod custom_delta;
pub mod daily_delta;
pub mod monthly_delta;
pub mod one_time_delta;
pub mod weekly_delta;
pub mod yearly_delta;

#[cfg(test)]
mod tests;

use crate::errors::MoolahCoreError;
use chrono::NaiveDate;

pub use custom_delta::CustomDelta;
pub use daily_delta::DailyDelta;
pub use monthly_delta::{MonthDay, MonthlyDelta};
pub use one_time_delta::OneTimeDelta;
pub use weekly_delta::WeeklyDelta;
pub use yearly_delta::YearlyDelta;

pub struct PositiveF32 {
    value: f32,
}

impl TryFrom<f32> for PositiveF32 {
    type Error = MoolahCoreError;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        if value < 0.0 {
            Err(MoolahCoreError::UnexpectedNegative(value))
        } else {
            Ok(PositiveF32 { value })
        }
    }
}

pub enum UncertaintyType {
    Dollars(PositiveF32),
    Percent(PositiveF32),
}

pub enum Uncertainty {
    Balanced(UncertaintyType),
    Unbalanced {
        low: UncertaintyType,
        high: UncertaintyType,
    },
    Bounds {
        low: f32,
        high: f32,
    },
}

fn reasonable_bounds(low: f32, high: f32, value: f32) -> Result<(), MoolahCoreError> {
    if (low <= value) & (value <= high) {
        Ok(())
    } else {
        Err(MoolahCoreError::IllogicalUncertaintyBounds { low, high, value })
    }
}

pub trait Delta {
    fn name(&self) -> &str;

    fn value(&self) -> f32;

    fn uncertainty(&self) -> &Option<Uncertainty>;

    fn dates(&self) -> &[NaiveDate];

    fn max_uncertainty_value(&self) -> f32 {
        match self.uncertainty() {
            Some(Uncertainty::Balanced(UncertaintyType::Dollars(unc))) => self.value() + unc.value,
            Some(Uncertainty::Balanced(UncertaintyType::Percent(unc))) => {
                self.value() + unc.value / 100.0 * self.value().abs()
            }
            Some(Uncertainty::Unbalanced {
                low: _,
                high: UncertaintyType::Dollars(unc),
            }) => self.value() + unc.value,
            Some(Uncertainty::Unbalanced {
                low: _,
                high: UncertaintyType::Percent(unc),
            }) => self.value() + unc.value / 100.0 * self.value().abs(),
            Some(Uncertainty::Bounds { low: _, high: unc }) => *unc,
            None => self.value(),
        }
    }

    fn min_uncertainty_value(&self) -> f32 {
        match self.uncertainty() {
            Some(Uncertainty::Balanced(UncertaintyType::Dollars(unc))) => self.value() - unc.value,
            Some(Uncertainty::Balanced(UncertaintyType::Percent(unc))) => {
                self.value() - unc.value / 100.0 * self.value().abs()
            }
            Some(Uncertainty::Unbalanced {
                low: UncertaintyType::Dollars(unc),
                high: _,
            }) => self.value() - unc.value,
            Some(Uncertainty::Unbalanced {
                low: UncertaintyType::Percent(unc),
                high: _,
            }) => self.value() - unc.value / 100.0 * self.value().abs(),
            Some(Uncertainty::Bounds { low: unc, high: _ }) => *unc,
            None => self.value(),
        }
    }
}

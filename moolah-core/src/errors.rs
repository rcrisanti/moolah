use chrono::NaiveDate;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MoolahCoreError {
    #[error("cannot create a `PositiveF32` from negative value {0}")]
    UnexpectedNegative(f32),

    #[error("illogical bounded uncertainty [{low}, {high}] for value {value}")]
    IllogicalUncertaintyBounds { low: f32, high: f32, value: f32 },

    #[error("start ({start}) cannot be after end ({end})")]
    StartAfterEnd { start: NaiveDate, end: NaiveDate },

    #[error("month day `{0}` must be in range [1, 31]")]
    MonthDayOutOfRange(u32),
}

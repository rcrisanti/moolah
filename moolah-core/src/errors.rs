use chrono::NaiveDate;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MoolahCoreError {
    #[error("cannot create a `PositiveF64` from negative value {0}")]
    UnexpectedNegative(f64),

    #[error("illogical bounded uncertainty [{low}, {high}] for value {value}")]
    IllogicalUncertaintyBounds { low: f64, high: f64, value: f64 },

    #[error("start ({start}) cannot be after end ({end})")]
    StartAfterEnd { start: NaiveDate, end: NaiveDate },

    #[error("month day `{0}` must be in range [1, 31]")]
    MonthDayOutOfRange(i64),
}

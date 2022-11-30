use chrono::NaiveDate;

use crate::errors::MoolahCoreError;

pub fn naive_ymd(year: i32, month: u32, day: u32) -> Result<NaiveDate, MoolahCoreError> {
    NaiveDate::from_ymd_opt(year, month, day)
        .ok_or_else(|| MoolahCoreError::InvalidDate(format!("{}-{}-{}", year, month, day)))
}

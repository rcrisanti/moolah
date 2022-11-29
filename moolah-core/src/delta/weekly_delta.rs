use super::{reasonable_bounds, Delta, Uncertainty};
use crate::errors::MoolahCoreError;
use chrono::{Datelike, Duration, Local, NaiveDate, Weekday};

pub struct WeeklyDelta {
    name: String,
    value: f64,
    uncertainty: Option<Uncertainty>,
    start: NaiveDate,
    end: NaiveDate,
    on_weekday: Weekday,
    skip_weeks: u32,
    dates: Vec<NaiveDate>,
}

impl Default for WeeklyDelta {
    fn default() -> Self {
        let today = Local::today().naive_local();

        WeeklyDelta {
            name: Default::default(),
            value: Default::default(),
            uncertainty: Default::default(),
            start: today,
            end: today,
            on_weekday: Weekday::Mon,
            skip_weeks: Default::default(),
            dates: vec![today],
        }
    }
}

fn round_up_to_next_weekday(date: &NaiveDate, weekday: &Weekday) -> NaiveDate {
    let days_diff =
        (weekday.number_from_monday() as i64 - date.weekday().number_from_monday() as i64) % 7;
    let days = if days_diff < 0 {
        7 + days_diff
    } else {
        days_diff
    };
    *date + Duration::days(days)
}

fn round_back_to_prev_weekday(date: &NaiveDate, weekday: &Weekday) -> NaiveDate {
    let days_diff =
        (date.weekday().number_from_monday() as i64 - weekday.number_from_monday() as i64) % 7;
    let days = if days_diff < 0 {
        7 + days_diff
    } else {
        days_diff
    };
    *date - Duration::days(days)
}

fn build_dates(
    start: &NaiveDate,
    end: &NaiveDate,
    on_weekday: &Weekday,
    every_weeks: i64,
) -> Vec<NaiveDate> {
    let start = round_up_to_next_weekday(&start, &on_weekday);
    let end = round_back_to_prev_weekday(&end, &on_weekday);
    let duration = Duration::weeks(every_weeks);
    let n_weeks: i32 = ((end - start).num_weeks() / every_weeks)
        .try_into()
        .expect("Too many weeks");
    (0..=n_weeks).map(|date| start + duration * date).collect()
}

impl WeeklyDelta {
    pub fn try_new(
        name: String,
        value: f64,
        uncertainty: Option<Uncertainty>,
        start: NaiveDate,
        end: NaiveDate,
        on_weekday: Option<Weekday>,
        skip_weeks: u32,
    ) -> Result<Self, MoolahCoreError> {
        if let Some(Uncertainty::Bounds { low, high }) = uncertainty {
            reasonable_bounds(low, high, value)?;
        }

        if start > end {
            return Err(MoolahCoreError::StartAfterEnd { start, end });
        }

        let weekday = match on_weekday {
            Some(weekday) => weekday,
            None => start.weekday(),
        };

        Ok(WeeklyDelta {
            name,
            value,
            uncertainty,
            start,
            end,
            on_weekday: weekday,
            skip_weeks,
            dates: build_dates(&start, &end, &weekday, (skip_weeks + 1).into()),
        })
    }

    pub fn start(&self) -> &NaiveDate {
        &self.start
    }

    pub fn end(&self) -> &NaiveDate {
        &self.end
    }

    pub fn on_weekday(&self) -> &Weekday {
        &self.on_weekday
    }

    pub fn skip_weeks(&self) -> u32 {
        self.skip_weeks
    }
}

impl Delta for WeeklyDelta {
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

#[cfg(test)]
mod private_functions {
    use super::*;

    #[test]
    fn test_round_date_up() {
        let date = NaiveDate::from_ymd(2022, 11, 2); // a Wednesday
        assert_eq!(
            round_up_to_next_weekday(&date, &Weekday::Mon),
            NaiveDate::from_ymd(2022, 11, 7)
        );
        assert_eq!(
            round_up_to_next_weekday(&date, &Weekday::Tue),
            NaiveDate::from_ymd(2022, 11, 8)
        );
        assert_eq!(
            round_up_to_next_weekday(&date, &Weekday::Wed),
            NaiveDate::from_ymd(2022, 11, 2)
        );
        assert_eq!(
            round_up_to_next_weekday(&date, &Weekday::Thu),
            NaiveDate::from_ymd(2022, 11, 3)
        );
        assert_eq!(
            round_up_to_next_weekday(&date, &Weekday::Fri),
            NaiveDate::from_ymd(2022, 11, 4)
        );
        assert_eq!(
            round_up_to_next_weekday(&date, &Weekday::Sat),
            NaiveDate::from_ymd(2022, 11, 5)
        );
        assert_eq!(
            round_up_to_next_weekday(&date, &Weekday::Sun),
            NaiveDate::from_ymd(2022, 11, 6)
        );
    }

    #[test]
    fn test_round_date_back() {
        let date = NaiveDate::from_ymd(2022, 11, 2); // a Wednesday
        assert_eq!(
            round_back_to_prev_weekday(&date, &Weekday::Mon),
            NaiveDate::from_ymd(2022, 10, 31)
        );
        assert_eq!(
            round_back_to_prev_weekday(&date, &Weekday::Tue),
            NaiveDate::from_ymd(2022, 11, 1)
        );
        assert_eq!(
            round_back_to_prev_weekday(&date, &Weekday::Wed),
            NaiveDate::from_ymd(2022, 11, 2)
        );
        assert_eq!(
            round_back_to_prev_weekday(&date, &Weekday::Thu),
            NaiveDate::from_ymd(2022, 10, 27)
        );
        assert_eq!(
            round_back_to_prev_weekday(&date, &Weekday::Fri),
            NaiveDate::from_ymd(2022, 10, 28)
        );
        assert_eq!(
            round_back_to_prev_weekday(&date, &Weekday::Sat),
            NaiveDate::from_ymd(2022, 10, 29)
        );
        assert_eq!(
            round_back_to_prev_weekday(&date, &Weekday::Sun),
            NaiveDate::from_ymd(2022, 10, 30)
        );
    }
}

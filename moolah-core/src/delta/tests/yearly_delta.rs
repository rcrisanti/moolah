use super::*;
use chrono::{Duration, Local};

#[test]
fn test_default() {
    let d = YearlyDelta::default();
    let today = Local::now().date_naive();

    assert_eq!(d.name(), "");
    assert_eq!(d.value(), 0.0);
    assert!(d.uncertainty().is_none());
    assert_eq!(d.dates(), &[today]);
    assert_eq!(*d.start(), today);
    assert_eq!(*d.end(), today);
    assert_eq!(d.skip_years(), 0);
}

#[test]
fn test_start_cannot_be_later_than_end() {
    let d = NaiveDate::from_ymd_opt(2022, 11, 1).unwrap();
    assert!(YearlyDelta::try_new(String::from("test"), 0.0, None, d, d, 0).is_ok());
    assert!(
        YearlyDelta::try_new(String::from("test"), 0.0, None, d, d - Duration::days(1), 0).is_err()
    );
}

#[test]
fn test_reasonable_bounds() {
    let date = NaiveDate::from_ymd_opt(2022, 10, 30).unwrap();

    assert!(YearlyDelta::try_new(
        String::from("test"),
        0.0,
        Some(Uncertainty::Bounds {
            low: 0.0,
            high: 0.0
        }),
        date,
        date,
        0
    )
    .is_ok());

    assert!(YearlyDelta::try_new(
        String::from("test"),
        0.0,
        Some(Uncertainty::Bounds {
            low: 0.0,
            high: 0.1
        }),
        date,
        date,
        0
    )
    .is_ok());

    assert!(YearlyDelta::try_new(
        String::from("test"),
        0.0,
        Some(Uncertainty::Bounds {
            low: -0.1,
            high: 0.0
        }),
        date,
        date,
        0
    )
    .is_ok());

    assert!(YearlyDelta::try_new(
        String::from("test"),
        0.0,
        Some(Uncertainty::Bounds {
            low: -0.1,
            high: 0.1
        }),
        date,
        date,
        0
    )
    .is_ok());

    assert!(YearlyDelta::try_new(
        String::from("test"),
        -1.0,
        Some(Uncertainty::Bounds {
            low: -0.1,
            high: 0.1
        }),
        date,
        date,
        0
    )
    .is_err());

    assert!(YearlyDelta::try_new(
        String::from("test"),
        1.0,
        Some(Uncertainty::Bounds {
            low: -0.1,
            high: 0.1
        }),
        date,
        date,
        0
    )
    .is_err());

    assert!(YearlyDelta::try_new(
        String::from("test"),
        0.0,
        Some(Uncertainty::Bounds {
            low: 0.1,
            high: -0.1
        }),
        date,
        date,
        0
    )
    .is_err());
}

fn test_yearly_dates(
    start: NaiveDate,
    end: NaiveDate,
    skip_years: u16,
    expected_dates: &[NaiveDate],
) {
    let d = YearlyDelta::try_new(String::from("test"), 0.0, None, start, end, skip_years).unwrap();

    let dates = d.dates();
    dbg!(dates);
    for date in expected_dates.iter() {
        assert!(dates.contains(date), "date = {}", date);
    }
    assert_eq!(dates.len(), expected_dates.len());
}

// Test where start day is same as end day
#[test]
fn test_falls_on_same_year_day_skip_years_0() {
    let expected_dates = vec![
        NaiveDate::from_ymd_opt(2024, 2, 29).unwrap(),
        NaiveDate::from_ymd_opt(2025, 2, 28).unwrap(),
        NaiveDate::from_ymd_opt(2026, 2, 28).unwrap(),
        NaiveDate::from_ymd_opt(2027, 2, 28).unwrap(),
        NaiveDate::from_ymd_opt(2028, 2, 29).unwrap(),
        NaiveDate::from_ymd_opt(2029, 2, 28).unwrap(),
        NaiveDate::from_ymd_opt(2030, 2, 28).unwrap(),
        NaiveDate::from_ymd_opt(2031, 2, 28).unwrap(),
        NaiveDate::from_ymd_opt(2032, 2, 29).unwrap(),
    ];

    test_yearly_dates(
        NaiveDate::from_ymd_opt(2024, 2, 29).unwrap(),
        NaiveDate::from_ymd_opt(2032, 2, 29).unwrap(),
        0,
        &expected_dates,
    );
}

#[test]
fn test_falls_on_same_year_day_skip_years_1() {
    let expected_dates = vec![
        NaiveDate::from_ymd_opt(2024, 2, 29).unwrap(),
        NaiveDate::from_ymd_opt(2026, 2, 28).unwrap(),
        NaiveDate::from_ymd_opt(2028, 2, 29).unwrap(),
        NaiveDate::from_ymd_opt(2030, 2, 28).unwrap(),
        NaiveDate::from_ymd_opt(2032, 2, 29).unwrap(),
    ];

    test_yearly_dates(
        NaiveDate::from_ymd_opt(2024, 2, 29).unwrap(),
        NaiveDate::from_ymd_opt(2032, 2, 29).unwrap(),
        1,
        &expected_dates,
    );
}

#[test]
fn test_falls_on_same_year_day_skip_years_2() {
    let expected_dates = vec![
        NaiveDate::from_ymd_opt(2024, 2, 29).unwrap(),
        NaiveDate::from_ymd_opt(2027, 2, 28).unwrap(),
        NaiveDate::from_ymd_opt(2030, 2, 28).unwrap(),
    ];

    test_yearly_dates(
        NaiveDate::from_ymd_opt(2024, 2, 29).unwrap(),
        NaiveDate::from_ymd_opt(2032, 2, 29).unwrap(),
        2,
        &expected_dates,
    );
}

#[test]
fn test_falls_on_same_year_day_skip_years_3() {
    let expected_dates = vec![
        NaiveDate::from_ymd_opt(2024, 2, 29).unwrap(),
        NaiveDate::from_ymd_opt(2028, 2, 29).unwrap(),
        NaiveDate::from_ymd_opt(2032, 2, 29).unwrap(),
    ];

    test_yearly_dates(
        NaiveDate::from_ymd_opt(2024, 2, 29).unwrap(),
        NaiveDate::from_ymd_opt(2032, 2, 29).unwrap(),
        3,
        &expected_dates,
    );
}

#[test]
fn test_falls_on_same_year_day_skip_years_100() {
    let expected_dates = vec![NaiveDate::from_ymd_opt(2024, 2, 29).unwrap()];

    test_yearly_dates(
        NaiveDate::from_ymd_opt(2024, 2, 29).unwrap(),
        NaiveDate::from_ymd_opt(2032, 2, 29).unwrap(),
        100,
        &expected_dates,
    );
}

// Test where start day is before end day
#[test]
fn test_falls_before_year_day_skip_years_0() {
    let expected_dates = vec![
        NaiveDate::from_ymd_opt(2024, 2, 29).unwrap(),
        NaiveDate::from_ymd_opt(2025, 2, 28).unwrap(),
        NaiveDate::from_ymd_opt(2026, 2, 28).unwrap(),
        NaiveDate::from_ymd_opt(2027, 2, 28).unwrap(),
        NaiveDate::from_ymd_opt(2028, 2, 29).unwrap(),
        NaiveDate::from_ymd_opt(2029, 2, 28).unwrap(),
        NaiveDate::from_ymd_opt(2030, 2, 28).unwrap(),
        NaiveDate::from_ymd_opt(2031, 2, 28).unwrap(),
        NaiveDate::from_ymd_opt(2032, 2, 29).unwrap(),
    ];

    test_yearly_dates(
        NaiveDate::from_ymd_opt(2024, 2, 29).unwrap(),
        NaiveDate::from_ymd_opt(2032, 3, 17).unwrap(),
        0,
        &expected_dates,
    );
}

#[test]
fn test_falls_before_year_day_skip_years_1() {
    let expected_dates = vec![
        NaiveDate::from_ymd_opt(2024, 2, 29).unwrap(),
        NaiveDate::from_ymd_opt(2026, 2, 28).unwrap(),
        NaiveDate::from_ymd_opt(2028, 2, 29).unwrap(),
        NaiveDate::from_ymd_opt(2030, 2, 28).unwrap(),
        NaiveDate::from_ymd_opt(2032, 2, 29).unwrap(),
    ];

    test_yearly_dates(
        NaiveDate::from_ymd_opt(2024, 2, 29).unwrap(),
        NaiveDate::from_ymd_opt(2032, 3, 17).unwrap(),
        1,
        &expected_dates,
    );
}

#[test]
fn test_falls_before_year_day_skip_years_2() {
    let expected_dates = vec![
        NaiveDate::from_ymd_opt(2024, 2, 29).unwrap(),
        NaiveDate::from_ymd_opt(2027, 2, 28).unwrap(),
        NaiveDate::from_ymd_opt(2030, 2, 28).unwrap(),
    ];

    test_yearly_dates(
        NaiveDate::from_ymd_opt(2024, 2, 29).unwrap(),
        NaiveDate::from_ymd_opt(2032, 3, 17).unwrap(),
        2,
        &expected_dates,
    );
}

#[test]
fn test_falls_before_year_day_skip_years_3() {
    let expected_dates = vec![
        NaiveDate::from_ymd_opt(2024, 2, 29).unwrap(),
        NaiveDate::from_ymd_opt(2028, 2, 29).unwrap(),
        NaiveDate::from_ymd_opt(2032, 2, 29).unwrap(),
    ];

    test_yearly_dates(
        NaiveDate::from_ymd_opt(2024, 2, 29).unwrap(),
        NaiveDate::from_ymd_opt(2032, 3, 17).unwrap(),
        3,
        &expected_dates,
    );
}

#[test]
fn test_falls_before_year_day_skip_years_100() {
    let expected_dates = vec![NaiveDate::from_ymd_opt(2024, 2, 29).unwrap()];

    test_yearly_dates(
        NaiveDate::from_ymd_opt(2024, 2, 29).unwrap(),
        NaiveDate::from_ymd_opt(2032, 3, 17).unwrap(),
        100,
        &expected_dates,
    );
}

// Test where start day is after end day
#[test]
fn test_falls_after_year_day_skip_years_0() {
    let expected_dates = vec![
        NaiveDate::from_ymd_opt(2024, 2, 29).unwrap(),
        NaiveDate::from_ymd_opt(2025, 2, 28).unwrap(),
        NaiveDate::from_ymd_opt(2026, 2, 28).unwrap(),
        NaiveDate::from_ymd_opt(2027, 2, 28).unwrap(),
        NaiveDate::from_ymd_opt(2028, 2, 29).unwrap(),
        NaiveDate::from_ymd_opt(2029, 2, 28).unwrap(),
        NaiveDate::from_ymd_opt(2030, 2, 28).unwrap(),
        NaiveDate::from_ymd_opt(2031, 2, 28).unwrap(),
    ];

    test_yearly_dates(
        NaiveDate::from_ymd_opt(2024, 2, 29).unwrap(),
        NaiveDate::from_ymd_opt(2032, 2, 17).unwrap(),
        0,
        &expected_dates,
    );
}

#[test]
fn test_falls_after_year_day_skip_years_1() {
    let expected_dates = vec![
        NaiveDate::from_ymd_opt(2024, 2, 29).unwrap(),
        NaiveDate::from_ymd_opt(2026, 2, 28).unwrap(),
        NaiveDate::from_ymd_opt(2028, 2, 29).unwrap(),
        NaiveDate::from_ymd_opt(2030, 2, 28).unwrap(),
    ];

    test_yearly_dates(
        NaiveDate::from_ymd_opt(2024, 2, 29).unwrap(),
        NaiveDate::from_ymd_opt(2032, 2, 17).unwrap(),
        1,
        &expected_dates,
    );
}

#[test]
fn test_falls_after_year_day_skip_years_2() {
    let expected_dates = vec![
        NaiveDate::from_ymd_opt(2024, 2, 29).unwrap(),
        NaiveDate::from_ymd_opt(2027, 2, 28).unwrap(),
        NaiveDate::from_ymd_opt(2030, 2, 28).unwrap(),
    ];

    test_yearly_dates(
        NaiveDate::from_ymd_opt(2024, 2, 29).unwrap(),
        NaiveDate::from_ymd_opt(2032, 2, 17).unwrap(),
        2,
        &expected_dates,
    );
}

#[test]
fn test_falls_after_year_day_skip_years_3() {
    let expected_dates = vec![
        NaiveDate::from_ymd_opt(2024, 2, 29).unwrap(),
        NaiveDate::from_ymd_opt(2028, 2, 29).unwrap(),
    ];

    test_yearly_dates(
        NaiveDate::from_ymd_opt(2024, 2, 29).unwrap(),
        NaiveDate::from_ymd_opt(2032, 2, 17).unwrap(),
        3,
        &expected_dates,
    );
}

#[test]
fn test_falls_after_year_day_skip_years_100() {
    let expected_dates = vec![NaiveDate::from_ymd_opt(2024, 2, 29).unwrap()];

    test_yearly_dates(
        NaiveDate::from_ymd_opt(2024, 2, 29).unwrap(),
        NaiveDate::from_ymd_opt(2032, 2, 17).unwrap(),
        100,
        &expected_dates,
    );
}

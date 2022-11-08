use super::*;
use chrono::{Duration, Local};

#[test]
fn test_default() {
    let d = MonthlyDelta::default();
    let today = Local::today().naive_local();

    assert_eq!(d.name(), "");
    assert_eq!(d.value(), 0.0);
    assert!(d.uncertainty().is_none());
    assert_eq!(d.dates(), &[today]);
    assert_eq!(*d.start(), today);
    assert_eq!(*d.end(), today);
    assert_eq!(*d.on_month_day(), 1u8.try_into().unwrap());
    assert_eq!(d.skip_months(), 0);
}

#[test]
fn test_start_cannot_be_later_than_end() {
    let d = NaiveDate::from_ymd(2022, 11, 1);
    assert!(MonthlyDelta::try_new(String::from("test"), 0.0, None, d, d, 1, 0).is_ok());
    assert!(MonthlyDelta::try_new(
        String::from("test"),
        0.0,
        None,
        d,
        d - Duration::days(1),
        1,
        0
    )
    .is_err());
}

#[test]
fn test_reasonable_bounds() {
    let date = NaiveDate::from_ymd(2022, 10, 30);

    assert!(MonthlyDelta::try_new(
        String::from("test"),
        0.0,
        Some(Uncertainty::Bounds {
            low: 0.0,
            high: 0.0
        }),
        date,
        date,
        1,
        0
    )
    .is_ok());

    assert!(MonthlyDelta::try_new(
        String::from("test"),
        0.0,
        Some(Uncertainty::Bounds {
            low: 0.0,
            high: 0.1
        }),
        date,
        date,
        1,
        0
    )
    .is_ok());

    assert!(MonthlyDelta::try_new(
        String::from("test"),
        0.0,
        Some(Uncertainty::Bounds {
            low: -0.1,
            high: 0.0
        }),
        date,
        date,
        1,
        0
    )
    .is_ok());

    assert!(MonthlyDelta::try_new(
        String::from("test"),
        0.0,
        Some(Uncertainty::Bounds {
            low: -0.1,
            high: 0.1
        }),
        date,
        date,
        1,
        0
    )
    .is_ok());

    assert!(MonthlyDelta::try_new(
        String::from("test"),
        -1.0,
        Some(Uncertainty::Bounds {
            low: -0.1,
            high: 0.1
        }),
        date,
        date,
        1,
        0
    )
    .is_err());

    assert!(MonthlyDelta::try_new(
        String::from("test"),
        1.0,
        Some(Uncertainty::Bounds {
            low: -0.1,
            high: 0.1
        }),
        date,
        date,
        1,
        0
    )
    .is_err());

    assert!(MonthlyDelta::try_new(
        String::from("test"),
        0.0,
        Some(Uncertainty::Bounds {
            low: 0.1,
            high: -0.1
        }),
        date,
        date,
        1,
        0
    )
    .is_err());
}

fn test_monthly_dates(
    start: NaiveDate,
    end: NaiveDate,
    on_month_day: u8,
    skip_months: u16,
    expected_dates: &[NaiveDate],
) {
    let d = MonthlyDelta::try_new(
        String::from("test"),
        0.0,
        None,
        start,
        end,
        on_month_day,
        skip_months,
    )
    .unwrap();

    let dates = d.dates();
    dbg!(dates);
    for date in expected_dates.iter() {
        assert!(dates.contains(date), "date = {}", date);
    }
    assert_eq!(dates.len(), expected_dates.len());
}

// Test where start & end dates are on the specified month day
#[test]
fn test_falls_on_month_day_skip_months_0() {
    let expected_dates = vec![
        NaiveDate::from_ymd(2022, 10, 30),
        NaiveDate::from_ymd(2022, 11, 30),
        NaiveDate::from_ymd(2022, 12, 30),
        NaiveDate::from_ymd(2023, 1, 30),
        NaiveDate::from_ymd(2023, 2, 28),
        NaiveDate::from_ymd(2023, 3, 30),
        NaiveDate::from_ymd(2023, 4, 30),
        NaiveDate::from_ymd(2023, 5, 30),
        NaiveDate::from_ymd(2023, 6, 30),
        NaiveDate::from_ymd(2023, 7, 30),
        NaiveDate::from_ymd(2023, 8, 30),
        NaiveDate::from_ymd(2023, 9, 30),
        NaiveDate::from_ymd(2023, 10, 30),
        NaiveDate::from_ymd(2023, 11, 30),
        NaiveDate::from_ymd(2023, 12, 30),
        NaiveDate::from_ymd(2024, 1, 30),
        NaiveDate::from_ymd(2024, 2, 29),
        NaiveDate::from_ymd(2024, 3, 30),
        NaiveDate::from_ymd(2024, 4, 30),
        NaiveDate::from_ymd(2024, 5, 30),
        NaiveDate::from_ymd(2024, 6, 30),
        NaiveDate::from_ymd(2024, 7, 30),
        NaiveDate::from_ymd(2024, 8, 30),
        NaiveDate::from_ymd(2024, 9, 30),
        NaiveDate::from_ymd(2024, 10, 30),
        NaiveDate::from_ymd(2024, 11, 30),
        NaiveDate::from_ymd(2024, 12, 30),
    ];

    test_monthly_dates(
        NaiveDate::from_ymd(2022, 10, 30),
        NaiveDate::from_ymd(2024, 12, 30),
        30,
        0,
        &expected_dates,
    );
}

#[test]
fn test_falls_on_month_day_skip_months_1() {
    let expected_dates = vec![
        NaiveDate::from_ymd(2022, 10, 30),
        NaiveDate::from_ymd(2022, 12, 30),
        NaiveDate::from_ymd(2023, 2, 28),
        NaiveDate::from_ymd(2023, 4, 30),
        NaiveDate::from_ymd(2023, 6, 30),
        NaiveDate::from_ymd(2023, 8, 30),
        NaiveDate::from_ymd(2023, 10, 30),
        NaiveDate::from_ymd(2023, 12, 30),
        NaiveDate::from_ymd(2024, 2, 29),
        NaiveDate::from_ymd(2024, 4, 30),
        NaiveDate::from_ymd(2024, 6, 30),
        NaiveDate::from_ymd(2024, 8, 30),
        NaiveDate::from_ymd(2024, 10, 30),
        NaiveDate::from_ymd(2024, 12, 30),
    ];

    test_monthly_dates(
        NaiveDate::from_ymd(2022, 10, 30),
        NaiveDate::from_ymd(2024, 12, 30),
        30,
        1,
        &expected_dates,
    );
}

#[test]
fn test_falls_on_month_day_skip_months_2() {
    let expected_dates = vec![
        NaiveDate::from_ymd(2022, 10, 30),
        NaiveDate::from_ymd(2023, 1, 30),
        NaiveDate::from_ymd(2023, 4, 30),
        NaiveDate::from_ymd(2023, 7, 30),
        NaiveDate::from_ymd(2023, 10, 30),
        NaiveDate::from_ymd(2024, 1, 30),
        NaiveDate::from_ymd(2024, 4, 30),
        NaiveDate::from_ymd(2024, 7, 30),
        NaiveDate::from_ymd(2024, 10, 30),
    ];

    test_monthly_dates(
        NaiveDate::from_ymd(2022, 10, 30),
        NaiveDate::from_ymd(2024, 12, 30),
        30,
        2,
        &expected_dates,
    );
}

#[test]
fn test_falls_on_month_day_skip_months_3() {
    let expected_dates = vec![
        NaiveDate::from_ymd(2022, 10, 30),
        NaiveDate::from_ymd(2023, 2, 28),
        NaiveDate::from_ymd(2023, 6, 30),
        NaiveDate::from_ymd(2023, 10, 30),
        NaiveDate::from_ymd(2024, 2, 29),
        NaiveDate::from_ymd(2024, 6, 30),
        NaiveDate::from_ymd(2024, 10, 30),
    ];

    test_monthly_dates(
        NaiveDate::from_ymd(2022, 10, 30),
        NaiveDate::from_ymd(2024, 12, 30),
        30,
        3,
        &expected_dates,
    );
}

#[test]
fn test_falls_on_month_day_skip_months_100() {
    let expected_dates = vec![NaiveDate::from_ymd(2022, 10, 30)];

    test_monthly_dates(
        NaiveDate::from_ymd(2022, 10, 30),
        NaiveDate::from_ymd(2024, 12, 30),
        30,
        100,
        &expected_dates,
    );
}

// Test where start & end dates are after the specified month day
#[test]
fn test_falls_after_month_day_skip_months_0() {
    let expected_dates = vec![
        NaiveDate::from_ymd(2022, 11, 30),
        NaiveDate::from_ymd(2022, 12, 30),
        NaiveDate::from_ymd(2023, 1, 30),
        NaiveDate::from_ymd(2023, 2, 28),
        NaiveDate::from_ymd(2023, 3, 30),
        NaiveDate::from_ymd(2023, 4, 30),
        NaiveDate::from_ymd(2023, 5, 30),
        NaiveDate::from_ymd(2023, 6, 30),
        NaiveDate::from_ymd(2023, 7, 30),
        NaiveDate::from_ymd(2023, 8, 30),
        NaiveDate::from_ymd(2023, 9, 30),
        NaiveDate::from_ymd(2023, 10, 30),
        NaiveDate::from_ymd(2023, 11, 30),
        NaiveDate::from_ymd(2023, 12, 30),
        NaiveDate::from_ymd(2024, 1, 30),
        NaiveDate::from_ymd(2024, 2, 29),
        NaiveDate::from_ymd(2024, 3, 30),
        NaiveDate::from_ymd(2024, 4, 30),
        NaiveDate::from_ymd(2024, 5, 30),
        NaiveDate::from_ymd(2024, 6, 30),
        NaiveDate::from_ymd(2024, 7, 30),
        NaiveDate::from_ymd(2024, 8, 30),
        NaiveDate::from_ymd(2024, 9, 30),
        NaiveDate::from_ymd(2024, 10, 30),
        NaiveDate::from_ymd(2024, 11, 30),
        NaiveDate::from_ymd(2024, 12, 30),
    ];

    test_monthly_dates(
        NaiveDate::from_ymd(2022, 10, 31),
        NaiveDate::from_ymd(2024, 12, 31),
        30,
        0,
        &expected_dates,
    );
}

#[test]
fn test_falls_after_month_day_skip_months_1() {
    let expected_dates = vec![
        NaiveDate::from_ymd(2022, 11, 30),
        NaiveDate::from_ymd(2023, 1, 30),
        NaiveDate::from_ymd(2023, 3, 30),
        NaiveDate::from_ymd(2023, 5, 30),
        NaiveDate::from_ymd(2023, 7, 30),
        NaiveDate::from_ymd(2023, 9, 30),
        NaiveDate::from_ymd(2023, 11, 30),
        NaiveDate::from_ymd(2024, 1, 30),
        NaiveDate::from_ymd(2024, 3, 30),
        NaiveDate::from_ymd(2024, 5, 30),
        NaiveDate::from_ymd(2024, 7, 30),
        NaiveDate::from_ymd(2024, 9, 30),
        NaiveDate::from_ymd(2024, 11, 30),
    ];

    test_monthly_dates(
        NaiveDate::from_ymd(2022, 10, 31),
        NaiveDate::from_ymd(2024, 12, 31),
        30,
        1,
        &expected_dates,
    );
}

#[test]
fn test_falls_after_month_day_skip_months_2() {
    let expected_dates = vec![
        NaiveDate::from_ymd(2022, 11, 30),
        NaiveDate::from_ymd(2023, 2, 28),
        NaiveDate::from_ymd(2023, 5, 30),
        NaiveDate::from_ymd(2023, 8, 30),
        NaiveDate::from_ymd(2023, 11, 30),
        NaiveDate::from_ymd(2024, 2, 29),
        NaiveDate::from_ymd(2024, 5, 30),
        NaiveDate::from_ymd(2024, 8, 30),
        NaiveDate::from_ymd(2024, 11, 30),
    ];

    test_monthly_dates(
        NaiveDate::from_ymd(2022, 10, 31),
        NaiveDate::from_ymd(2024, 12, 31),
        30,
        2,
        &expected_dates,
    );
}

#[test]
fn test_falls_after_month_day_skip_months_3() {
    let expected_dates = vec![
        NaiveDate::from_ymd(2022, 11, 30),
        NaiveDate::from_ymd(2023, 3, 30),
        NaiveDate::from_ymd(2023, 7, 30),
        NaiveDate::from_ymd(2023, 11, 30),
        NaiveDate::from_ymd(2024, 3, 30),
        NaiveDate::from_ymd(2024, 7, 30),
        NaiveDate::from_ymd(2024, 11, 30),
    ];

    test_monthly_dates(
        NaiveDate::from_ymd(2022, 10, 31),
        NaiveDate::from_ymd(2024, 12, 31),
        30,
        3,
        &expected_dates,
    );
}

#[test]
fn test_falls_after_month_day_skip_months_100() {
    let expected_dates = vec![NaiveDate::from_ymd(2022, 11, 30)];

    test_monthly_dates(
        NaiveDate::from_ymd(2022, 10, 31),
        NaiveDate::from_ymd(2024, 12, 31),
        30,
        100,
        &expected_dates,
    );
}

// Test where start & end dates are before the specified month day
#[test]
fn test_falls_before_month_day_skip_months_0() {
    let expected_dates = vec![
        NaiveDate::from_ymd(2022, 10, 30),
        NaiveDate::from_ymd(2022, 11, 30),
        NaiveDate::from_ymd(2022, 12, 30),
        NaiveDate::from_ymd(2023, 1, 30),
        NaiveDate::from_ymd(2023, 2, 28),
        NaiveDate::from_ymd(2023, 3, 30),
        NaiveDate::from_ymd(2023, 4, 30),
        NaiveDate::from_ymd(2023, 5, 30),
        NaiveDate::from_ymd(2023, 6, 30),
        NaiveDate::from_ymd(2023, 7, 30),
        NaiveDate::from_ymd(2023, 8, 30),
        NaiveDate::from_ymd(2023, 9, 30),
        NaiveDate::from_ymd(2023, 10, 30),
        NaiveDate::from_ymd(2023, 11, 30),
        NaiveDate::from_ymd(2023, 12, 30),
        NaiveDate::from_ymd(2024, 1, 30),
        NaiveDate::from_ymd(2024, 2, 29),
        NaiveDate::from_ymd(2024, 3, 30),
        NaiveDate::from_ymd(2024, 4, 30),
        NaiveDate::from_ymd(2024, 5, 30),
        NaiveDate::from_ymd(2024, 6, 30),
        NaiveDate::from_ymd(2024, 7, 30),
        NaiveDate::from_ymd(2024, 8, 30),
        NaiveDate::from_ymd(2024, 9, 30),
        NaiveDate::from_ymd(2024, 10, 30),
        NaiveDate::from_ymd(2024, 11, 30),
    ];

    test_monthly_dates(
        NaiveDate::from_ymd(2022, 10, 29),
        NaiveDate::from_ymd(2024, 12, 12),
        30,
        0,
        &expected_dates,
    );
}

#[test]
fn test_falls_before_month_day_skip_months_1() {
    let expected_dates = vec![
        NaiveDate::from_ymd(2022, 10, 30),
        NaiveDate::from_ymd(2022, 12, 30),
        NaiveDate::from_ymd(2023, 2, 28),
        NaiveDate::from_ymd(2023, 4, 30),
        NaiveDate::from_ymd(2023, 6, 30),
        NaiveDate::from_ymd(2023, 8, 30),
        NaiveDate::from_ymd(2023, 10, 30),
        NaiveDate::from_ymd(2023, 12, 30),
        NaiveDate::from_ymd(2024, 2, 29),
        NaiveDate::from_ymd(2024, 4, 30),
        NaiveDate::from_ymd(2024, 6, 30),
        NaiveDate::from_ymd(2024, 8, 30),
        NaiveDate::from_ymd(2024, 10, 30),
    ];

    test_monthly_dates(
        NaiveDate::from_ymd(2022, 10, 29),
        NaiveDate::from_ymd(2024, 12, 12),
        30,
        1,
        &expected_dates,
    );
}

#[test]
fn test_falls_before_month_day_skip_months_2() {
    let expected_dates = vec![
        NaiveDate::from_ymd(2022, 10, 30),
        NaiveDate::from_ymd(2023, 1, 30),
        NaiveDate::from_ymd(2023, 4, 30),
        NaiveDate::from_ymd(2023, 7, 30),
        NaiveDate::from_ymd(2023, 10, 30),
        NaiveDate::from_ymd(2024, 1, 30),
        NaiveDate::from_ymd(2024, 4, 30),
        NaiveDate::from_ymd(2024, 7, 30),
        NaiveDate::from_ymd(2024, 10, 30),
    ];

    test_monthly_dates(
        NaiveDate::from_ymd(2022, 10, 29),
        NaiveDate::from_ymd(2024, 12, 12),
        30,
        2,
        &expected_dates,
    );
}

#[test]
fn test_falls_before_month_day_skip_months_3() {
    let expected_dates = vec![
        NaiveDate::from_ymd(2022, 10, 30),
        NaiveDate::from_ymd(2023, 2, 28),
        NaiveDate::from_ymd(2023, 6, 30),
        NaiveDate::from_ymd(2023, 10, 30),
        NaiveDate::from_ymd(2024, 2, 29),
        NaiveDate::from_ymd(2024, 6, 30),
        NaiveDate::from_ymd(2024, 10, 30),
    ];

    test_monthly_dates(
        NaiveDate::from_ymd(2022, 10, 29),
        NaiveDate::from_ymd(2024, 12, 12),
        30,
        3,
        &expected_dates,
    );
}

#[test]
fn test_falls_before_month_day_skip_months_100() {
    let expected_dates = vec![NaiveDate::from_ymd(2022, 10, 30)];

    test_monthly_dates(
        NaiveDate::from_ymd(2022, 10, 29),
        NaiveDate::from_ymd(2024, 12, 12),
        30,
        100,
        &expected_dates,
    );
}

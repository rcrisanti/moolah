use chrono::{Duration, Local};

use super::*;

#[test]
fn test_default() {
    let d = DailyDelta::default();
    let today = Local::now().date_naive();

    assert_eq!(d.name(), "");
    assert_eq!(d.value(), 0.0);
    assert!(d.uncertainty().is_none());
    assert_eq!(d.dates(), &[today]);
    assert_eq!(*d.start(), today);
    assert_eq!(*d.end(), today);
    assert_eq!(d.skip_days(), 0);
}

#[test]
fn test_start_cannot_be_later_than_end() {
    let d = NaiveDate::from_ymd_opt(2022, 11, 1).unwrap();
    assert!(DailyDelta::try_new(String::from("test"), 0.0, None, d, d, 0).is_ok());
    assert!(
        DailyDelta::try_new(String::from("test"), 0.0, None, d, d - Duration::days(1), 0).is_err()
    );
}

#[test]
fn test_reasonable_bounds() {
    let date = NaiveDate::from_ymd_opt(2022, 10, 30).unwrap();

    assert!(DailyDelta::try_new(
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

    assert!(DailyDelta::try_new(
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

    assert!(DailyDelta::try_new(
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

    assert!(DailyDelta::try_new(
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

    assert!(DailyDelta::try_new(
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

    assert!(DailyDelta::try_new(
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

    assert!(DailyDelta::try_new(
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

#[test]
fn skip_days_0() {
    let d = DailyDelta::try_new(
        String::from("test"),
        1000.0,
        None,
        NaiveDate::from_ymd_opt(2022, 10, 1).unwrap(),
        NaiveDate::from_ymd_opt(2022, 10, 5).unwrap(),
        0,
    )
    .expect("couldn't make daily delta");

    let expected_dates = vec![
        NaiveDate::from_ymd_opt(2022, 10, 1).unwrap(),
        NaiveDate::from_ymd_opt(2022, 10, 2).unwrap(),
        NaiveDate::from_ymd_opt(2022, 10, 3).unwrap(),
        NaiveDate::from_ymd_opt(2022, 10, 4).unwrap(),
        NaiveDate::from_ymd_opt(2022, 10, 5).unwrap(),
    ];

    let dates = d.dates();
    for date in expected_dates.iter() {
        assert!(dates.contains(date), "date = {}", date);
    }
    assert_eq!(dates.len(), expected_dates.len());
}

#[test]
fn skip_days_1() {
    let d = DailyDelta::try_new(
        String::from("test"),
        1000.0,
        None,
        NaiveDate::from_ymd_opt(2022, 10, 1).unwrap(),
        NaiveDate::from_ymd_opt(2022, 10, 5).unwrap(),
        1,
    )
    .expect("couldn't make daily delta");

    let expected_dates = vec![
        NaiveDate::from_ymd_opt(2022, 10, 1).unwrap(),
        NaiveDate::from_ymd_opt(2022, 10, 3).unwrap(),
        NaiveDate::from_ymd_opt(2022, 10, 5).unwrap(),
    ];

    let dates = d.dates();
    for date in expected_dates.iter() {
        assert!(dates.contains(date), "date = {}", date);
    }
    assert_eq!(dates.len(), expected_dates.len());
}

#[test]
fn skip_days_2() {
    let d = DailyDelta::try_new(
        String::from("test"),
        1000.0,
        None,
        NaiveDate::from_ymd_opt(2022, 10, 1).unwrap(),
        NaiveDate::from_ymd_opt(2022, 10, 5).unwrap(),
        2,
    )
    .expect("couldn't make daily delta");

    let expected_dates = vec![
        NaiveDate::from_ymd_opt(2022, 10, 1).unwrap(),
        NaiveDate::from_ymd_opt(2022, 10, 4).unwrap(),
    ];

    let dates = d.dates();
    for date in expected_dates.iter() {
        assert!(dates.contains(date), "date = {}", date);
    }
    assert_eq!(dates.len(), expected_dates.len());
}

#[test]
fn skip_days_3() {
    let d = DailyDelta::try_new(
        String::from("test"),
        1000.0,
        None,
        NaiveDate::from_ymd_opt(2022, 10, 1).unwrap(),
        NaiveDate::from_ymd_opt(2022, 10, 5).unwrap(),
        3,
    )
    .expect("couldn't make daily delta");

    let expected_dates = vec![
        NaiveDate::from_ymd_opt(2022, 10, 1).unwrap(),
        NaiveDate::from_ymd_opt(2022, 10, 5).unwrap(),
    ];

    let dates = d.dates();
    for date in expected_dates.iter() {
        assert!(dates.contains(date), "date = {}", date);
    }
    assert_eq!(dates.len(), expected_dates.len());
}

#[test]
fn skip_days_4() {
    let d = DailyDelta::try_new(
        String::from("test"),
        1000.0,
        None,
        NaiveDate::from_ymd_opt(2022, 10, 1).unwrap(),
        NaiveDate::from_ymd_opt(2022, 10, 5).unwrap(),
        4,
    )
    .expect("couldn't make daily delta");

    let expected_dates = vec![NaiveDate::from_ymd_opt(2022, 10, 1).unwrap()];

    let dates = d.dates();
    for date in expected_dates.iter() {
        assert!(dates.contains(date), "date = {}", date);
    }
    assert_eq!(dates.len(), expected_dates.len());
}

#[test]
fn skip_days_100() {
    let d = DailyDelta::try_new(
        String::from("test"),
        1000.0,
        None,
        NaiveDate::from_ymd_opt(2022, 10, 1).unwrap(),
        NaiveDate::from_ymd_opt(2022, 10, 5).unwrap(),
        100,
    )
    .expect("couldn't make daily delta");

    let expected_dates = vec![NaiveDate::from_ymd_opt(2022, 10, 1).unwrap()];

    let dates = d.dates();
    for date in expected_dates.iter() {
        assert!(dates.contains(date), "date = {}", date);
    }
    assert_eq!(dates.len(), expected_dates.len());
}

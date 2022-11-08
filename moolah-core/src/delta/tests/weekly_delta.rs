use super::*;
use chrono::{Duration, Local, Weekday};

#[test]
fn test_default() {
    let d = WeeklyDelta::default();
    let today = Local::today().naive_local();

    assert_eq!(d.name(), "");
    assert_eq!(d.value(), 0.0);
    assert!(d.uncertainty().is_none());
    assert_eq!(d.dates(), &[today]);
    assert_eq!(*d.start(), today);
    assert_eq!(*d.end(), today);
    assert_eq!(*d.on_weekday(), Weekday::Mon);
    assert_eq!(d.skip_weeks(), 0);
}

#[test]
fn test_start_cannot_be_later_than_end() {
    let d = NaiveDate::from_ymd(2022, 11, 1);
    assert!(
        WeeklyDelta::try_new(String::from("test"), 0.0, None, d, d, Some(Weekday::Mon), 0).is_ok()
    );
    assert!(WeeklyDelta::try_new(
        String::from("test"),
        0.0,
        None,
        d,
        d - Duration::days(1),
        Some(Weekday::Mon),
        0
    )
    .is_err());
}

#[test]
fn test_reasonable_bounds() {
    let date = NaiveDate::from_ymd(2022, 10, 30);

    assert!(WeeklyDelta::try_new(
        String::from("test"),
        0.0,
        Some(Uncertainty::Bounds {
            low: 0.0,
            high: 0.0
        }),
        date,
        date,
        Some(Weekday::Mon),
        0
    )
    .is_ok());

    assert!(WeeklyDelta::try_new(
        String::from("test"),
        0.0,
        Some(Uncertainty::Bounds {
            low: 0.0,
            high: 0.1
        }),
        date,
        date,
        Some(Weekday::Mon),
        0
    )
    .is_ok());

    assert!(WeeklyDelta::try_new(
        String::from("test"),
        0.0,
        Some(Uncertainty::Bounds {
            low: -0.1,
            high: 0.0
        }),
        date,
        date,
        Some(Weekday::Mon),
        0
    )
    .is_ok());

    assert!(WeeklyDelta::try_new(
        String::from("test"),
        0.0,
        Some(Uncertainty::Bounds {
            low: -0.1,
            high: 0.1
        }),
        date,
        date,
        Some(Weekday::Mon),
        0
    )
    .is_ok());

    assert!(WeeklyDelta::try_new(
        String::from("test"),
        -1.0,
        Some(Uncertainty::Bounds {
            low: -0.1,
            high: 0.1
        }),
        date,
        date,
        Some(Weekday::Mon),
        0
    )
    .is_err());

    assert!(WeeklyDelta::try_new(
        String::from("test"),
        1.0,
        Some(Uncertainty::Bounds {
            low: -0.1,
            high: 0.1
        }),
        date,
        date,
        Some(Weekday::Mon),
        0
    )
    .is_err());

    assert!(WeeklyDelta::try_new(
        String::from("test"),
        0.0,
        Some(Uncertainty::Bounds {
            low: 0.1,
            high: -0.1
        }),
        date,
        date,
        Some(Weekday::Mon),
        0
    )
    .is_err());
}

fn test_weekly_dates(
    start: NaiveDate,
    end: NaiveDate,
    weekday: Option<Weekday>,
    skip_weeks: u32,
    expected_dates: &[NaiveDate],
) {
    let d = WeeklyDelta::try_new(
        String::from("test"),
        0.0,
        None,
        start,
        end,
        weekday,
        skip_weeks,
    )
    .unwrap();

    let dates = d.dates();
    dbg!(dates);
    for date in expected_dates.iter() {
        assert!(dates.contains(date), "date = {}", date);
    }
    assert_eq!(dates.len(), expected_dates.len());
}

// Test where start & end falls on the specified weekday
#[test]
fn test_falls_on_weekday_skip_weeks_0() {
    let expected_dates = vec![
        NaiveDate::from_ymd(2022, 12, 19),
        NaiveDate::from_ymd(2022, 12, 26),
        NaiveDate::from_ymd(2023, 1, 2),
        NaiveDate::from_ymd(2023, 1, 9),
        NaiveDate::from_ymd(2023, 1, 16),
    ];

    test_weekly_dates(
        NaiveDate::from_ymd(2022, 12, 19),
        NaiveDate::from_ymd(2023, 1, 16),
        Some(Weekday::Mon),
        0,
        &expected_dates,
    );
}

#[test]
fn test_falls_on_weekday_skip_weeks_1() {
    let expected_dates = vec![
        NaiveDate::from_ymd(2022, 12, 19),
        NaiveDate::from_ymd(2023, 1, 2),
        NaiveDate::from_ymd(2023, 1, 16),
    ];

    test_weekly_dates(
        NaiveDate::from_ymd(2022, 12, 19),
        NaiveDate::from_ymd(2023, 1, 16),
        Some(Weekday::Mon),
        1,
        &expected_dates,
    );
}

#[test]
fn test_falls_on_weekday_skip_weeks_2() {
    let expected_dates = vec![
        NaiveDate::from_ymd(2022, 12, 19),
        NaiveDate::from_ymd(2023, 1, 9),
    ];

    test_weekly_dates(
        NaiveDate::from_ymd(2022, 12, 19),
        NaiveDate::from_ymd(2023, 1, 16),
        Some(Weekday::Mon),
        2,
        &expected_dates,
    );
}

#[test]
fn test_falls_on_weekday_skip_weeks_3() {
    let expected_dates = vec![
        NaiveDate::from_ymd(2022, 12, 19),
        NaiveDate::from_ymd(2023, 1, 16),
    ];

    test_weekly_dates(
        NaiveDate::from_ymd(2022, 12, 19),
        NaiveDate::from_ymd(2023, 1, 16),
        Some(Weekday::Mon),
        3,
        &expected_dates,
    );
}

#[test]
fn test_falls_on_weekday_skip_weeks_4() {
    let expected_dates = vec![NaiveDate::from_ymd(2022, 12, 19)];

    test_weekly_dates(
        NaiveDate::from_ymd(2022, 12, 19),
        NaiveDate::from_ymd(2023, 1, 16),
        Some(Weekday::Mon),
        4,
        &expected_dates,
    );
}

#[test]
fn test_falls_on_weekday_skip_weeks_100() {
    let expected_dates = vec![NaiveDate::from_ymd(2022, 12, 19)];

    test_weekly_dates(
        NaiveDate::from_ymd(2022, 12, 19),
        NaiveDate::from_ymd(2023, 1, 16),
        Some(Weekday::Mon),
        100,
        &expected_dates,
    );
}

// Test where start & end falls on weekday before specified
#[test]
fn test_falls_before_weekday_skip_weeks_0() {
    let expected_dates = vec![
        NaiveDate::from_ymd(2022, 12, 21),
        NaiveDate::from_ymd(2022, 12, 28),
        NaiveDate::from_ymd(2023, 1, 4),
        NaiveDate::from_ymd(2023, 1, 11),
    ];

    test_weekly_dates(
        NaiveDate::from_ymd(2022, 12, 19),
        NaiveDate::from_ymd(2023, 1, 16),
        Some(Weekday::Wed),
        0,
        &expected_dates,
    );
}

#[test]
fn test_falls_before_weekday_skip_weeks_1() {
    let expected_dates = vec![
        NaiveDate::from_ymd(2022, 12, 21),
        NaiveDate::from_ymd(2023, 1, 4),
    ];

    test_weekly_dates(
        NaiveDate::from_ymd(2022, 12, 19),
        NaiveDate::from_ymd(2023, 1, 16),
        Some(Weekday::Wed),
        1,
        &expected_dates,
    );
}

#[test]
fn test_falls_before_weekday_skip_weeks_2() {
    let expected_dates = vec![
        NaiveDate::from_ymd(2022, 12, 21),
        NaiveDate::from_ymd(2023, 1, 11),
    ];

    test_weekly_dates(
        NaiveDate::from_ymd(2022, 12, 19),
        NaiveDate::from_ymd(2023, 1, 16),
        Some(Weekday::Wed),
        2,
        &expected_dates,
    );
}

#[test]
fn test_falls_before_weekday_skip_weeks_3() {
    let expected_dates = vec![NaiveDate::from_ymd(2022, 12, 21)];

    test_weekly_dates(
        NaiveDate::from_ymd(2022, 12, 19),
        NaiveDate::from_ymd(2023, 1, 16),
        Some(Weekday::Wed),
        3,
        &expected_dates,
    );
}

#[test]
fn test_falls_before_weekday_skip_weeks_100() {
    let expected_dates = vec![NaiveDate::from_ymd(2022, 12, 21)];

    test_weekly_dates(
        NaiveDate::from_ymd(2022, 12, 19),
        NaiveDate::from_ymd(2023, 1, 16),
        Some(Weekday::Wed),
        100,
        &expected_dates,
    );
}

// Test where start & end falls on weekday after specified
#[test]
fn test_falls_after_weekday_skip_weeks_0() {
    let expected_dates = vec![
        NaiveDate::from_ymd(2022, 12, 28),
        NaiveDate::from_ymd(2023, 1, 4),
        NaiveDate::from_ymd(2023, 1, 11),
        NaiveDate::from_ymd(2023, 1, 18),
    ];

    test_weekly_dates(
        NaiveDate::from_ymd(2022, 12, 23),
        NaiveDate::from_ymd(2023, 1, 21),
        Some(Weekday::Wed),
        0,
        &expected_dates,
    );
}

#[test]
fn test_falls_after_weekday_skip_weeks_1() {
    let expected_dates = vec![
        NaiveDate::from_ymd(2022, 12, 28),
        NaiveDate::from_ymd(2023, 1, 11),
    ];

    test_weekly_dates(
        NaiveDate::from_ymd(2022, 12, 23),
        NaiveDate::from_ymd(2023, 1, 21),
        Some(Weekday::Wed),
        1,
        &expected_dates,
    );
}

#[test]
fn test_falls_after_weekday_skip_weeks_2() {
    let expected_dates = vec![
        NaiveDate::from_ymd(2022, 12, 28),
        NaiveDate::from_ymd(2023, 1, 18),
    ];

    test_weekly_dates(
        NaiveDate::from_ymd(2022, 12, 23),
        NaiveDate::from_ymd(2023, 1, 21),
        Some(Weekday::Wed),
        2,
        &expected_dates,
    );
}

#[test]
fn test_falls_after_weekday_skip_weeks_3() {
    let expected_dates = vec![NaiveDate::from_ymd(2022, 12, 28)];

    test_weekly_dates(
        NaiveDate::from_ymd(2022, 12, 23),
        NaiveDate::from_ymd(2023, 1, 21),
        Some(Weekday::Wed),
        3,
        &expected_dates,
    );
}

#[test]
fn test_falls_after_weekday_skip_weeks_100() {
    let expected_dates = vec![NaiveDate::from_ymd(2022, 12, 28)];

    test_weekly_dates(
        NaiveDate::from_ymd(2022, 12, 23),
        NaiveDate::from_ymd(2023, 1, 21),
        Some(Weekday::Wed),
        100,
        &expected_dates,
    );
}

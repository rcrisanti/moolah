use super::*;
use chrono::{Local, NaiveDate};

#[test]
fn test_default() {
    let d = OneTimeDelta::default();

    assert_eq!(d.name(), "");
    assert_eq!(d.value(), 0.0);
    assert!(d.uncertainty().is_none());
    assert_eq!(*d.date(), Local::today().naive_local());
    assert_eq!(d.dates(), &[Local::today().naive_local()]);
}

#[test]
fn test_dates() {
    let date = NaiveDate::from_ymd(2022, 10, 30);
    let d = OneTimeDelta::try_new(String::from("test"), 1000.0, None, date)
        .expect("Could not build OneTimeDelta");

    assert_eq!(d.dates().len(), 1);
    assert!(d.dates().contains(&date));
}

#[test]
fn test_reasonable_bounds() {
    let date = NaiveDate::from_ymd(2022, 10, 30);

    assert!(OneTimeDelta::try_new(
        String::from("test"),
        0.0,
        Some(Uncertainty::Bounds {
            low: 0.0,
            high: 0.0
        }),
        date
    )
    .is_ok());

    assert!(OneTimeDelta::try_new(
        String::from("test"),
        0.0,
        Some(Uncertainty::Bounds {
            low: 0.0,
            high: 0.1
        }),
        date
    )
    .is_ok());

    assert!(OneTimeDelta::try_new(
        String::from("test"),
        0.0,
        Some(Uncertainty::Bounds {
            low: -0.1,
            high: 0.0
        }),
        date
    )
    .is_ok());

    assert!(OneTimeDelta::try_new(
        String::from("test"),
        0.0,
        Some(Uncertainty::Bounds {
            low: -0.1,
            high: 0.1
        }),
        date
    )
    .is_ok());

    assert!(OneTimeDelta::try_new(
        String::from("test"),
        -1.0,
        Some(Uncertainty::Bounds {
            low: -0.1,
            high: 0.1
        }),
        date
    )
    .is_err());

    assert!(OneTimeDelta::try_new(
        String::from("test"),
        1.0,
        Some(Uncertainty::Bounds {
            low: -0.1,
            high: 0.1
        }),
        date
    )
    .is_err());

    assert!(OneTimeDelta::try_new(
        String::from("test"),
        0.0,
        Some(Uncertainty::Bounds {
            low: 0.1,
            high: -0.1
        }),
        date
    )
    .is_err());
}

#[test]
fn test_balanced_uncertainty() {
    let date = NaiveDate::from_ymd(2022, 10, 30);

    // dollar uncertainty w/ positive value
    let d = OneTimeDelta::try_new(
        String::from("test"),
        1000.0,
        Some(Uncertainty::Balanced(UncertaintyType::Dollars(
            3.0.try_into().unwrap(),
        ))),
        date,
    )
    .expect("Could not build OneTimeDelta");
    assert_eq!(d.max_uncertainty_value(), 1003.0);
    assert_eq!(d.min_uncertainty_value(), 997.0);

    // dollar uncertainty w/ negative value
    let d = OneTimeDelta::try_new(
        String::from("test"),
        -1000.0,
        Some(Uncertainty::Balanced(UncertaintyType::Dollars(
            3.0.try_into().unwrap(),
        ))),
        date,
    )
    .expect("Could not build OneTimeDelta");
    assert_eq!(d.max_uncertainty_value(), -997.0);
    assert_eq!(d.min_uncertainty_value(), -1003.0);

    // percent uncertainty w/ positive value
    let d = OneTimeDelta::try_new(
        String::from("test"),
        1000.0,
        Some(Uncertainty::Balanced(UncertaintyType::Percent(
            3.0.try_into().unwrap(),
        ))),
        date,
    )
    .expect("Could not build OneTimeDelta");
    assert_eq!(d.max_uncertainty_value(), 1030.0);
    assert_eq!(d.min_uncertainty_value(), 970.0);

    // percent uncertainty w/ negative value
    let d = OneTimeDelta::try_new(
        String::from("test"),
        -1000.0,
        Some(Uncertainty::Balanced(UncertaintyType::Percent(
            3.0.try_into().unwrap(),
        ))),
        date,
    )
    .expect("Could not build OneTimeDelta");
    assert_eq!(d.max_uncertainty_value(), -970.0);
    assert_eq!(d.min_uncertainty_value(), -1030.0);
}

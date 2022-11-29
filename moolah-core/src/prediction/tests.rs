use super::*;
use crate::delta::{
    CustomDelta, DailyDelta, MonthlyDelta, OneTimeDelta, Uncertainty, UncertaintyType, WeeklyDelta,
    YearlyDelta,
};
use chrono::Weekday;
use std::fmt::Debug;

#[test]
fn test_all_deltas_insertable() {
    let p = Prediction::new(
        String::from("test"),
        NaiveDate::from_ymd(2022, 10, 10),
        1000.0,
        vec![
            Box::new(DailyDelta::default()),
            Box::new(OneTimeDelta::default()),
            Box::new(WeeklyDelta::default()),
            Box::new(MonthlyDelta::default()),
            Box::new(YearlyDelta::default()),
            Box::new(CustomDelta::default()),
        ],
    );

    assert_eq!(p.deltas().len(), 6);
}

#[test]
fn test_default_delta_state() {
    let def = AggregatedDelta::default();
    let manual = AggregatedDelta {
        value: 0.0,
        min_uncertainty_val: 0.0,
        max_uncertainty_val: 0.0,
        impactful_deltas: vec![],
    };
    assert_eq!(def, manual);
}

fn assert_btrees_eq<K: Debug + PartialEq, V: Debug + PartialEq>(
    expected: &BTreeMap<K, V>,
    calculated: &BTreeMap<K, V>,
) {
    dbg!(&expected, &calculated);
    assert_eq!(calculated.len(), expected.len(), "length not equal");
    for ((exp_key, exp_val), (calc_key, calc_val)) in expected.iter().zip(calculated.iter()) {
        assert_eq!(exp_key, calc_key);
        assert_eq!(exp_val, calc_val, "values not equal for key {:?}", exp_key);
    }
}

#[test]
fn test_no_deltas() {
    let p = Prediction::default();
    let pred = p.predict(&Local::today().naive_local());

    let expected = BTreeMap::from([(Local::today().naive_local(), PredictionState::default())]);
    assert_btrees_eq(&expected, &pred);
}

#[test]
fn test_1_delta() {
    let start_pred = NaiveDate::from_ymd(2022, 10, 28);
    let p = Prediction {
        start: start_pred,
        initial_value: 500.0,
        deltas: vec![Box::new(
            OneTimeDelta::try_new(
                "test".into(),
                100.0,
                None,
                NaiveDate::from_ymd(2022, 10, 31),
            )
            .unwrap(),
        )],
        ..Default::default()
    };

    let end_pred = NaiveDate::from_ymd(2022, 11, 1);

    let expected = BTreeMap::from([
        (
            NaiveDate::from_ymd(2022, 10, 28),
            PredictionState::new(500.0, 500.0, 500.0, [].into()),
        ),
        (
            NaiveDate::from_ymd(2022, 10, 31),
            PredictionState::new(600.0, 600.0, 600.0, ["test".into()].into()),
        ),
    ]);

    assert_btrees_eq(&expected, &p.predict(&end_pred));
}

#[test]
fn test_2_deltas_balanced_uncertainty() {
    let start_pred = NaiveDate::from_ymd(2022, 10, 28);
    let p = Prediction {
        start: start_pred,
        initial_value: 500.0,
        deltas: vec![
            Box::new(
                OneTimeDelta::try_new(
                    "test 1".into(),
                    200.0,
                    Some(Uncertainty::Balanced(UncertaintyType::Dollars(
                        15.0.try_into().unwrap(),
                    ))),
                    NaiveDate::from_ymd(2022, 10, 31),
                )
                .unwrap(),
            ),
            Box::new(
                OneTimeDelta::try_new(
                    "test 2".into(),
                    100.0,
                    Some(Uncertainty::Balanced(UncertaintyType::Percent(
                        5.0.try_into().unwrap(),
                    ))),
                    NaiveDate::from_ymd(2022, 10, 31),
                )
                .unwrap(),
            ),
        ],
        ..Default::default()
    };

    let end_pred = NaiveDate::from_ymd(2022, 11, 1);

    let expected = BTreeMap::from([
        (
            NaiveDate::from_ymd(2022, 10, 28),
            PredictionState::new(500.0, 500.0, 500.0, [].into()),
        ),
        (
            NaiveDate::from_ymd(2022, 10, 31),
            PredictionState::new(
                800.0,
                780.0,
                820.0,
                ["test 1".into(), "test 2".into()].into(),
            ),
        ),
    ]);

    assert_btrees_eq(&expected, &p.predict(&end_pred));
}

#[test]
fn test_2_deltas_unbalanced_uncertainty() {
    let start_pred = NaiveDate::from_ymd(2022, 10, 28);
    let p = Prediction {
        start: start_pred,
        initial_value: 500.0,
        deltas: vec![
            Box::new(
                OneTimeDelta::try_new(
                    "test 1".into(),
                    200.0,
                    Some(Uncertainty::Unbalanced {
                        low: UncertaintyType::Dollars(15.0.try_into().unwrap()),
                        high: UncertaintyType::Percent(10.0.try_into().unwrap()),
                    }),
                    NaiveDate::from_ymd(2022, 10, 31),
                )
                .unwrap(),
            ),
            Box::new(
                OneTimeDelta::try_new(
                    "test 2".into(),
                    100.0,
                    Some(Uncertainty::Unbalanced {
                        low: UncertaintyType::Percent(5.0.try_into().unwrap()),
                        high: UncertaintyType::Dollars(5.0.try_into().unwrap()),
                    }),
                    NaiveDate::from_ymd(2022, 10, 31),
                )
                .unwrap(),
            ),
        ],
        ..Default::default()
    };

    let end_pred = NaiveDate::from_ymd(2022, 11, 1);

    let expected = BTreeMap::from([
        (
            NaiveDate::from_ymd(2022, 10, 28),
            PredictionState::new(500.0, 500.0, 500.0, [].into()),
        ),
        (
            NaiveDate::from_ymd(2022, 10, 31),
            PredictionState::new(
                800.0,
                780.0,
                825.0,
                ["test 1".into(), "test 2".into()].into(),
            ),
        ),
    ]);

    assert_btrees_eq(&expected, &p.predict(&end_pred));
}

#[test]
fn test_prediction_all_delta_types() {
    let start_pred = NaiveDate::from_ymd(2022, 10, 28);
    let p = Prediction {
        start: start_pred,
        deltas: vec![
            Box::new(
                OneTimeDelta::try_new(
                    "one time".into(),
                    200.0,
                    Some(Uncertainty::Unbalanced {
                        low: UncertaintyType::Dollars(15.0.try_into().unwrap()),
                        high: UncertaintyType::Percent(10.0.try_into().unwrap()),
                    }),
                    NaiveDate::from_ymd(2022, 10, 31),
                )
                .unwrap(),
            ),
            Box::new(
                YearlyDelta::try_new(
                    "yearly".into(),
                    -155.0,
                    None,
                    NaiveDate::from_ymd(2022, 7, 1),
                    NaiveDate::from_ymd(2024, 12, 14),
                    0,
                )
                .unwrap(),
            ),
            Box::new(
                MonthlyDelta::try_new(
                    "monthly".into(),
                    1234.0,
                    Some(Uncertainty::Balanced(UncertaintyType::Dollars(
                        12.0.try_into().unwrap(),
                    ))),
                    start_pred,
                    NaiveDate::from_ymd(2022, 12, 31),
                    3.try_into().unwrap(),
                    1,
                )
                .unwrap(),
            ),
            Box::new(
                WeeklyDelta::try_new(
                    "weekly".into(),
                    -13.0,
                    None,
                    NaiveDate::from_ymd(2022, 11, 3),
                    NaiveDate::from_ymd(2022, 12, 12),
                    Some(Weekday::Wed),
                    1,
                )
                .unwrap(),
            ),
            Box::new(
                DailyDelta::try_new(
                    "daily".into(),
                    1.0,
                    None,
                    NaiveDate::from_ymd(2022, 11, 22),
                    NaiveDate::from_ymd(2022, 12, 7),
                    2,
                )
                .unwrap(),
            ),
            Box::new(
                CustomDelta::try_new(
                    "custom".into(),
                    -15.0,
                    Some(Uncertainty::Bounds {
                        low: -20.0,
                        high: -12.0,
                    }),
                    vec![
                        NaiveDate::from_ymd(2022, 11, 12),
                        NaiveDate::from_ymd(2022, 12, 1),
                    ],
                )
                .unwrap(),
            ),
        ],
        ..Default::default()
    };

    let end_pred = NaiveDate::from_ymd(2023, 8, 1);

    let expected = BTreeMap::from([
        (
            NaiveDate::from_ymd(2022, 10, 28),
            PredictionState::default(),
        ),
        (
            NaiveDate::from_ymd(2022, 10, 31),
            PredictionState::new(200.0, 185.0, 220.0, ["one time".into()].into()),
        ),
        (
            NaiveDate::from_ymd(2022, 11, 3),
            PredictionState::new(1434.0, 1407.0, 1466.0, ["monthly".into()].into()),
        ),
        (
            NaiveDate::from_ymd(2022, 11, 9),
            PredictionState::new(1421.0, 1394.0, 1453.0, ["weekly".into()].into()),
        ),
        (
            NaiveDate::from_ymd(2022, 11, 12),
            PredictionState::new(1406.0, 1374.0, 1441.0, ["custom".into()].into()),
        ),
        (
            NaiveDate::from_ymd(2022, 11, 22),
            PredictionState::new(1407.0, 1375.0, 1442.0, ["daily".into()].into()),
        ),
        (
            NaiveDate::from_ymd(2022, 11, 23),
            PredictionState::new(1394.0, 1362.0, 1429.0, ["weekly".into()].into()),
        ),
        (
            NaiveDate::from_ymd(2022, 11, 25),
            PredictionState::new(1395.0, 1363.0, 1430.0, ["daily".into()].into()),
        ),
        (
            NaiveDate::from_ymd(2022, 11, 28),
            PredictionState::new(1396.0, 1364.0, 1431.0, ["daily".into()].into()),
        ),
        (
            NaiveDate::from_ymd(2022, 12, 1),
            PredictionState::new(
                1382.0,
                1345.0,
                1420.0,
                ["daily".into(), "custom".into()].into(),
            ),
        ),
        (
            NaiveDate::from_ymd(2022, 12, 4),
            PredictionState::new(1383.0, 1346.0, 1421.0, ["daily".into()].into()),
        ),
        (
            NaiveDate::from_ymd(2022, 12, 7),
            PredictionState::new(
                1371.0,
                1334.0,
                1409.0,
                ["weekly".into(), "daily".into()].into(),
            ),
        ),
        (
            NaiveDate::from_ymd(2023, 7, 1),
            PredictionState::new(1216.0, 1179.0, 1254.0, ["yearly".into()].into()),
        ),
    ]);

    assert_btrees_eq(&expected, &p.predict(&end_pred));
}

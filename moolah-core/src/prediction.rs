#[cfg(test)]
mod tests;

use crate::delta::Delta;
use chrono::{Local, NaiveDate};
use std::collections::{BTreeMap, HashSet};

pub struct Prediction {
    name: String,
    start: NaiveDate,
    initial_value: f64,
    deltas: Vec<Box<dyn Delta>>,
}

impl Default for Prediction {
    fn default() -> Self {
        Prediction {
            name: Default::default(),
            start: Local::now().date_naive(),
            initial_value: Default::default(),
            deltas: Default::default(),
        }
    }
}

impl Prediction {
    pub fn new(
        name: String,
        start: NaiveDate,
        initial_value: f64,
        deltas: Vec<Box<dyn Delta>>,
    ) -> Self {
        Prediction {
            name,
            start,
            initial_value,
            deltas,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn start(&self) -> &NaiveDate {
        &self.start
    }

    pub fn initial_value(&self) -> f64 {
        self.initial_value
    }

    pub fn deltas(&self) -> &[Box<dyn Delta>] {
        &self.deltas
    }
}

#[derive(Debug, PartialEq, Default)]
pub(crate) struct AggregatedDelta<'a> {
    value: f64,
    min_uncertainty_val: f64,
    max_uncertainty_val: f64,
    impactful_deltas: Vec<&'a str>,
}

impl<'a> AggregatedDelta<'a> {
    pub fn update(&mut self, delta: &'a dyn Delta) {
        self.value += delta.value();
        self.min_uncertainty_val += delta.min_uncertainty_value();
        self.max_uncertainty_val += delta.max_uncertainty_value();
        self.impactful_deltas.push(delta.name());
    }
}

impl Prediction {
    fn aggregate_deltas(&self, end: &NaiveDate) -> BTreeMap<NaiveDate, AggregatedDelta> {
        let initial = AggregatedDelta::default();
        let mut deltas: BTreeMap<NaiveDate, AggregatedDelta> =
            BTreeMap::from([(*self.start(), initial)]);

        for delta in self.deltas() {
            for date in delta.dates() {
                if (*date >= self.start) & (date <= end) {
                    deltas
                        .entry(*date)
                        .and_modify(|pred_state| pred_state.update(&**delta))
                        .or_insert_with(|| {
                            let mut pred = AggregatedDelta::default();
                            pred.update(&**delta);
                            pred
                        });
                }
            }
        }

        // Add in empty delta at start date if no deltas have been there
        deltas.entry(self.start).or_default();

        deltas
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct PredictionState {
    value: f64,
    min_uncertainty_val: f64,
    max_uncertainty_val: f64,
    impactful_deltas: HashSet<String>,
}

impl PredictionState {
    pub fn new(
        value: f64,
        min_uncertainty_val: f64,
        max_uncertainty_val: f64,
        impactful_deltas: HashSet<String>,
    ) -> Self {
        PredictionState {
            value,
            min_uncertainty_val,
            max_uncertainty_val,
            impactful_deltas,
        }
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    fn from(previous_pred_state: &PredictionState, delta_agg: &AggregatedDelta) -> Self {
        PredictionState {
            value: previous_pred_state.value + delta_agg.value,
            min_uncertainty_val: previous_pred_state.min_uncertainty_val
                + delta_agg.min_uncertainty_val,
            max_uncertainty_val: previous_pred_state.max_uncertainty_val
                + delta_agg.max_uncertainty_val,
            impactful_deltas: delta_agg
                .impactful_deltas
                .clone()
                .into_iter()
                .map(|name| name.into())
                .collect(),
        }
    }
}

impl Prediction {
    pub fn predict(&self, end: &NaiveDate) -> BTreeMap<NaiveDate, PredictionState> {
        let agg_deltas = self.aggregate_deltas(end);

        let init_pred_state = PredictionState::new(
            self.initial_value,
            self.initial_value,
            self.initial_value,
            [].into(),
        );

        agg_deltas
            .iter()
            .scan(init_pred_state, |pred_state, (date, agg_delta)| {
                *pred_state = PredictionState::from(pred_state, agg_delta);
                Some((*date, pred_state.clone()))
            })
            .collect::<BTreeMap<_, _>>()
    }
}

// SPDX-FileCopyrightText: 2023 Davidson <twister@davidson.fr>
// SPDX-License-Identifier: GPL-3.0-or-later

use itertools::Itertools;
use serde::{Deserialize, Serialize};
use stats::median;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct TestReport {
    name: String,
    pub energy: f64,
    pub transfer: u64,
    pub storage: u64,
}

impl TestReport {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            energy: 0.0,
            transfer: 0,
            storage: 0,
        }
    }
}

// ===

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Report {
    details: Vec<TestReport>,
    total: Vec<TestReport>,
}

impl Report {
    pub fn new() -> Self {
        Self {
            details: Vec::new(),
            total: Vec::new(),
        }
    }
    pub fn add_test_report(&mut self, ir: TestReport) {
        self.details.push(ir);
    }
    fn median(&self, mut values: Vec<f64>) -> f64 {
        values.sort_by(|a, b| a.partial_cmp(b).unwrap());

        if 0 == values.len() % 2 {
            (values[(values.len() / 2) - 1] + values[values.len() / 2]) / 2.0
        } else {
            values[values.len() / 2]
        }
    }
    pub fn compute_total(&mut self) {
        self.total = Vec::new();
        let mut tr_dict_e = HashMap::<String, Vec<f64>>::new();
        let mut tr_dict_t = HashMap::<String, Vec<f64>>::new();
        let mut tr_dict_s = HashMap::<String, Vec<f64>>::new();

        for tr in &self.details {
            if !tr_dict_e.contains_key(&tr.name) {
                tr_dict_e.insert(String::from(&tr.name), vec![tr.energy]);
                tr_dict_t.insert(String::from(&tr.name), vec![tr.transfer as f64]);
                tr_dict_s.insert(String::from(&tr.name), vec![tr.storage as f64]);
            } else {
                tr_dict_e.get_mut(&tr.name).unwrap().push(tr.energy);
                tr_dict_s
                    .get_mut(&tr.name)
                    .unwrap()
                    .push(tr.transfer as f64);
                tr_dict_s.get_mut(&tr.name).unwrap().push(tr.storage as f64);
            }
        }
        for key in tr_dict_e.keys().sorted() {
            let mut tr = TestReport::new(key);
            tr.energy = median(tr_dict_e[key].clone().into_iter()).unwrap();
            tr.transfer = median(tr_dict_t[key].clone().into_iter()).unwrap() as u64;
            tr.storage = median(tr_dict_s[key].clone().into_iter()).unwrap() as u64;

            self.total.push(tr);
        }
    }
}

// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test_report() -> Result<(), serde_yaml::Error> {
        let mut tr = TestReport::new("Test 1");
        tr.energy = 1.11;
        tr.transfer = 2;
        tr.storage = 3;

        assert_eq!("Test 1", tr.name);
        assert_eq!(1.11, tr.energy);
        assert_eq!(2, tr.transfer);
        assert_eq!(3, tr.storage);

        let yaml = serde_yaml::to_string(&tr)?;
        assert_eq!(
            yaml,
            "name: Test 1\nenergy: 1.11\ntransfer: 2\nstorage: 3\n"
        );

        Ok(())
    }

    #[test]
    fn test_report_median() {
        let r = Report::new();

        let values_odd = vec![8.11, 20.2, 7.11, 9.10, 8.09, 7.9, 8.1];
        assert_eq!(8.1, r.median(values_odd));

        let values_even = vec![8.11, 20.2, 7.11, 9.10, 8.09, 7.9];
        assert_eq!(8.1, r.median(values_even));
    }

    #[test]
    fn test_report() -> Result<(), serde_yaml::Error> {
        let mut r = Report::new();

        let mut tr = TestReport::new("Test 1");
        tr.energy = 1.0;
        r.add_test_report(tr);
        tr = TestReport::new("Test 2");
        tr.energy = 2.0;
        r.add_test_report(tr);

        tr = TestReport::new("Test 1");
        tr.energy = 3.0;
        r.add_test_report(tr);
        tr = TestReport::new("Test 2");
        tr.energy = 4.0;
        r.add_test_report(tr);

        r.compute_total();

        let yaml = serde_yaml::to_string(&r)?;
        let expected_yaml: &str = "details:
- name: Test 1
  energy: 1.0
  transfer: 0
  storage: 0
- name: Test 2
  energy: 2.0
  transfer: 0
  storage: 0
- name: Test 1
  energy: 3.0
  transfer: 0
  storage: 0
- name: Test 2
  energy: 4.0
  transfer: 0
  storage: 0
total:
- name: Test 1
  energy: 2.0
  transfer: 0
  storage: 0
- name: Test 2
  energy: 3.0
  transfer: 0
  storage: 0
";

        assert_eq!(expected_yaml, yaml);

        Ok(())
    }
}

use itertools::Itertools;
use serde::{Deserialize, Serialize};
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
    pub fn compute_total(&mut self) {
        self.total = Vec::new();
        let mut tr_dict_nb = HashMap::<String, u64>::new();
        let mut tr_dict_e = HashMap::<String, f64>::new();
        let mut tr_dict_t = HashMap::<String, u64>::new();
        let mut tr_dict_s = HashMap::<String, u64>::new();

        for tr in &self.details {
            if !tr_dict_nb.contains_key(&tr.name) {
                tr_dict_nb.insert(String::from(&tr.name), 1);
                tr_dict_e.insert(String::from(&tr.name), tr.energy);
                tr_dict_t.insert(String::from(&tr.name), tr.transfer);
                tr_dict_s.insert(String::from(&tr.name), tr.storage);
            } else {
                *tr_dict_nb.get_mut(&tr.name).unwrap() += 1;
                *tr_dict_e.get_mut(&tr.name).unwrap() += tr.energy;
                *tr_dict_t.get_mut(&tr.name).unwrap() += tr.transfer;
                *tr_dict_s.get_mut(&tr.name).unwrap() += tr.storage;
            }
        }
        for key in tr_dict_e.keys().sorted() {
            let mut tr = TestReport::new(key);
            tr.energy = tr_dict_e[key] / (tr_dict_nb[key] as f64);
            tr.transfer = tr_dict_t[key] / tr_dict_nb[key];
            tr.storage = tr_dict_s[key] / tr_dict_nb[key];

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

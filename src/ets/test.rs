// SPDX-FileCopyrightText: 2023 Davidson <twister@davidson.fr>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::error::Error;

use super::system_call::SystemCall;

pub trait Test {
    fn name(&self) -> &String;
    fn services_names(&self) -> &Vec<String>;
    fn run(&mut self) -> Result<(), Box<dyn Error>>;
}

// ===

pub struct SystemCallTest {
    name: String,
    services_names: Vec<String>,
    system_call: SystemCall,
}

impl SystemCallTest {
    pub fn new(name: &str, commandline: &str) -> Self {
        Self {
            name: name.into(),
            services_names: vec![],
            system_call: SystemCall::new(commandline),
        }
    }

    pub fn add_service_name(&mut self, service_name: &str) {
        self.services_names.push(service_name.into());
    }
}

impl Test for SystemCallTest {
    fn name(&self) -> &String {
        &self.name
    }

    fn services_names(&self) -> &Vec<String> {
        &self.services_names
    }

    fn run(&mut self) -> Result<(), Box<dyn Error>> {
        self.system_call.execute()?;

        Ok(())
    }
}

// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    use std::time::Instant;

    #[test]
    fn system_call_test_creation_with_command_line() {
        let sct = SystemCallTest::new("TestName", "ls");
        assert_eq!(sct.name, "TestName");
        assert!(sct.services_names.is_empty());
        assert_eq!(sct.system_call.path(), "ls");
    }

    #[test]
    fn system_call_test_add_service_name() {
        let mut sct = SystemCallTest::new("TestName", "ls");
        assert!(sct.services_names.is_empty());
        sct.add_service_name("sn1");
        assert_eq!(sct.services_names.len(), 1);
        sct.add_service_name("sn2");
        assert_eq!(sct.services_names.len(), 2);
        let mut i = 0;
        for sn in sct.services_names.iter() {
            match i {
                0 => assert_eq!(sn, "sn1"),
                1 => assert_eq!(sn, "sn2"),
                _ => panic!("Uncovered service name"),
            }
            i += 1;
        }
    }

    #[test]
    fn system_call_test_run() {
        let mut sct = SystemCallTest::new("TestName", "ls");
        assert!(!sct.run().is_err());
    }

    #[test]
    fn system_call_test_run_long_process() {
        let now = Instant::now();
        let mut sct = SystemCallTest::new("TestName", "sleep 5");

        assert!(!sct.run().is_err());
        assert!(now.elapsed().as_secs() >= 5);
    }
}

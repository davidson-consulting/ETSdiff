// SPDX-FileCopyrightText: 2023 Davidson <twister@davidson.fr>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

use super::etscomponent::{EComponent, ETSComponent, OctetsComponent, SComponent, TComponent};
use super::iteration_scheduler::{IterationScheduler, SchedulerType, StageredScheduler};
use super::report::{Report, TestReport};
use super::service::ServicesLink;
use super::test::Test;

pub struct ETSdiff {
    pub services: ServicesLink,
    pub tests: Vec<Box<dyn Test>>,
    pub e_component: Option<EComponent>,
    pub s_component: Option<SComponent>,
    pub t_component: Option<TComponent>,
    pub scheduler: Option<Box<dyn IterationScheduler>>,
    pub report: Report,
}

impl ETSdiff {
    pub fn new() -> Self {
        let mut ret = Self {
            services: Rc::new(RefCell::new(Vec::new())),
            tests: Vec::new(),
            e_component: None,
            s_component: None,
            t_component: None,
            scheduler: None,
            report: Report::new(),
        };
        ret.set_s_component();
        ret.set_t_component();
        ret.set_e_component();

        ret
    }

    pub fn set_s_component(&mut self) {
        self.s_component = Some(SComponent::new(&self.services));
    }

    pub fn set_t_component(&mut self) {
        self.t_component = Some(TComponent::new(&self.services));
    }

    pub fn set_e_component(&mut self) {
        self.e_component = Some(EComponent::new(&self.services));
    }

    pub fn set_scheduler(&mut self, st: SchedulerType, nb_iteration: u32) {
        self.scheduler = match st {
            SchedulerType::StageredScheduler => {
                Some(Box::new(StageredScheduler::new(nb_iteration)))
            }
        }
    }

    pub fn get_ordered_tests_list(&mut self) -> Vec<u32> {
        if self.scheduler.is_none() {
            self.set_scheduler(SchedulerType::StageredScheduler, 2);
        }
        self.scheduler
            .as_ref()
            .unwrap()
            .get_ordered_list(self.tests.len().try_into().unwrap())
    }

    fn prepare_etscomponents(&mut self) {
        println!("Preparing ETSComponents...");
        match self.t_component {
            None => println!("  No TComponent"),
            Some(ref mut c) => {
                println!("  Preparing TComponent");
                c.before_campaign();
            }
        }
        match self.s_component {
            None => println!("  No SComponent"),
            Some(ref mut c) => {
                println!("  Preparing SComponent");
                c.before_campaign();
            }
        }
        match self.e_component {
            None => println!("  No EComponent"),
            Some(ref mut c) => {
                println!("  Preparing EComponent");
                c.before_campaign();
            }
        }
    }
    fn release_etscomponents(&mut self) {
        println!("Releasing ETSComponents...");
        match self.t_component {
            None => println!("  No TComponent"),
            Some(ref mut c) => {
                println!("  Releasing TComponent");
                c.after_campaign();
            }
        }
        match self.s_component {
            None => println!("  No SComponent"),
            Some(ref mut c) => {
                println!("  Releasing SComponent");
                c.after_campaign();
            }
        }
        match self.e_component {
            None => println!("  No EComponent"),
            Some(ref mut c) => {
                println!("  Realeasing EComponent");
                c.after_campaign();
            }
        }
    }

    /*
    fn prepare_services(&mut self, test: &dyn Test) {
        let mut services = self.services.borrow_mut();
        println!("Prepare {} services...", services.len());
        for s in &mut *services {
            if test.services_names().contains(&s.name) {
                if s.prepare.is_none() {
                    println!("  Service {} don't have prepare method", s.name);
                } else if s.prepare().is_err() {
                    eprintln!("  Error when preparing service {}", s.name);
                } else {
                    println!("  Service {} prepare()", s.name);
                }
            }
        }
        println!("--\n");
    }
    fn clean_services(&mut self, test: &dyn Test) {
        let mut services = self.services.borrow_mut();
        println!("    Clean {} services...", services.len());
        for s in &mut *services {
            if test.services_names().contains(&s.name) {
                if s.clean.is_none() {
                    println!("      Service {} don't have clean method", s.name);
                } else if s.clean().is_err() {
                    eprintln!("      Error when cleaning service {}", s.name);
                } else {
                    println!("      Service {} clean()", s.name);
                }
            }
        }
        println!("--\n");
    }
    fn release_services(&mut self, test: &dyn Test) {
        let mut services = self.services.borrow_mut();
        println!("Release {} services...", services.len());
        for s in &mut *services {
            if test.services_names().contains(&s.name) {
                if s.release.is_none() {
                    println!("  Service {} don't have release method", s.name);
                } else if s.release().is_err() {
                    eprintln!("  Error when releasing service {}", s.name);
                } else {
                    println!("  Service {} release()", s.name);
                }
            }
        }
        println!("--\n");
    }
    */

    pub fn execute(&mut self) -> Result<(), Box<dyn Error>> {
        println!("Nb tests: {:?}", self.tests.len());

        let tests_order = self.get_ordered_tests_list();
        println!("Ordered test: {tests_order:?}");
        println!("--\n");

        self.prepare_etscomponents();
        println!("--\n");

        println!("Iterations:");
        for itest in tests_order {
            let test = &mut self.tests[itest as usize];
            let mut tr = TestReport::new(test.name());
            println!("  [TEST: {}]", test.name());

            {
                // self.prepare_services(test.as_ref()); TODO: undersant how to deal with borrow *self more than once
                let mut services = self.services.borrow_mut();
                println!("    Prepare services...");
                for s in &mut *services {
                    if test.services_names().contains(&s.name) {
                        if s.prepare.is_none() {
                            println!("      Service {} don't have prepare method", s.name);
                        } else if s.prepare().is_err() {
                            eprintln!("      Error when preparing service {}", s.name);
                        } else {
                            println!("      Service {} prepare()", s.name);
                        }
                    }
                }
            }

            println!("    Starting ETSComponents...");
            match self.t_component {
                None => println!("      No TComponent to start"),
                Some(ref mut c) => {
                    println!("      Starting TComponent");
                    c.before_test(test.as_ref());
                }
            }
            match self.s_component {
                None => println!("      No SComponent to start"),
                Some(ref mut c) => {
                    println!("      Starting SComponent");
                    c.before_test(test.as_ref());
                }
            }
            match self.e_component {
                None => println!("      No EComponent to start"),
                Some(ref mut c) => {
                    println!("      Starting EComponent");
                    c.before_test(test.as_ref());
                }
            }

            println!("    => Running test...");
            test.run()?;

            println!("    Stoping ETSComponents...");
            match self.e_component {
                None => println!("      No EComponent to stop"),
                Some(ref mut c) => {
                    println!("      Stoping EComponent");
                    c.after_test(test.as_ref());
                }
            }
            match self.t_component {
                None => println!("      No TComponent to stop"),
                Some(ref mut c) => {
                    println!("      Stoping TComponent");
                    c.after_test(test.as_ref());
                }
            }
            match self.s_component {
                None => println!("      No SComponent to stop"),
                Some(ref mut c) => {
                    println!("      Stoping SComponent");
                    c.after_test(test.as_ref());
                }
            }

            println!("    Get results...");
            match self.e_component {
                None => println!("      No EComponent"),
                Some(ref mut c) => {
                    println!("      EComponent -> {} Joules", c.to_joules());
                    tr.energy = c.to_joules();
                }
            }
            match self.t_component {
                None => println!("      No TComponent"),
                Some(ref mut c) => {
                    println!("      TComponent -> {} Ko", c.to_octets() / 1024);
                    tr.transfer = c.to_octets();
                }
            }
            match self.s_component {
                None => println!("      No SComponent"),
                Some(ref mut c) => {
                    println!("      SComponent -> {} Ko", c.to_octets() / 1024);
                    tr.storage = c.to_octets();
                }
            }
            self.report.add_test_report(tr);

            {
                // self.clean_services(test.as_ref()); TODO: undersant how to deal with borrow *self more than once
                // self.release_services(test.as_ref()); TODO: undersant how to deal with borrow *self more than once
                let mut services = self.services.borrow_mut();
                println!("    Clean and Release services...");
                for s in &mut *services {
                    if test.services_names().contains(&s.name) {
                        if s.clean.is_none() {
                            println!("      Service {} don't have clean method", s.name);
                        } else if s.clean().is_err() {
                            eprintln!("      Error when cleaning service {}", s.name);
                        } else {
                            println!("      Service {} clean()", s.name);
                        }

                        if s.release.is_none() {
                            println!("      Service {} don't have release method", s.name);
                        } else if s.release().is_err() {
                            eprintln!("      Error when releasing service {}", s.name);
                        } else {
                            println!("      Service {} release()", s.name);
                        }
                    }
                }
            }
        }
        println!("--\n");

        self.release_etscomponents();

        // report
        println!("Finalizing report");
        self.report.compute_total();

        Ok(())
    }
}

impl Default for ETSdiff {
    fn default() -> Self {
        Self::new()
    }
}

// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::service::Service;
    use super::super::test::SystemCallTest;

    #[test]
    fn etsdiff_creation() {
        let etsd = ETSdiff::new();
        let services = etsd.services.borrow();

        assert_eq!(0, services.len());
        assert_eq!(0, etsd.tests.len());
        assert!(etsd.scheduler.is_none());
    }

    #[test]
    fn etsdiff_services() {
        let etsd = ETSdiff::new();
        let mut services = etsd.services.borrow_mut();

        assert_eq!(0, services.len());
        services.push(Service::new("Service 1"));
        assert_eq!(1, services.len());
        services.push(Service::new("Service 2"));
        assert_eq!(2, services.len());
    }

    #[test]
    fn etsdiff_tests() {
        let mut etsd = ETSdiff::new();

        assert_eq!(0, etsd.tests.len());
        etsd.tests
            .push(Box::new(SystemCallTest::new("Test 1", "echo \"T1\"")));
        assert_eq!(1, etsd.tests.len());
        etsd.tests
            .push(Box::new(SystemCallTest::new("Test 2", "echo \"T2\"")));
        assert_eq!(2, etsd.tests.len());
    }

    #[test]
    fn etsdiff_set_e_component() {
        let etsd = ETSdiff::new();

        assert!(etsd.e_component.is_some());
    }

    #[test]
    fn etsdiff_set_t_component() {
        let etsd = ETSdiff::new();

        assert!(etsd.s_component.is_some());
    }

    #[test]
    fn etsdiff_set_s_component() {
        let etsd = ETSdiff::new();

        assert!(etsd.s_component.is_some());
    }

    #[test]
    fn etsdiff_set_scheduler() {
        let mut etsd = ETSdiff::new();

        assert!(etsd.scheduler.is_none());

        etsd.set_scheduler(SchedulerType::StageredScheduler, 7);
        assert!(etsd.scheduler.is_some());
        assert_eq!(7, etsd.scheduler.unwrap().nb_iteration());
    }

    #[test]
    fn etsdiff_defult_scheduler() {
        let mut etsd = ETSdiff::new();

        etsd.tests
            .push(Box::new(SystemCallTest::new("Test 1", "echo \"T1\"")));
        etsd.tests
            .push(Box::new(SystemCallTest::new("Test 2", "echo \"T2\"")));
        assert_eq!(2, etsd.tests.len());

        assert!(etsd.scheduler.is_none());
        assert_eq!(vec![0, 1, 0, 1], etsd.get_ordered_tests_list());
        assert!(etsd.scheduler.is_some());
    }
}

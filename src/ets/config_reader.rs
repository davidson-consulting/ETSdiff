// SPDX-FileCopyrightText: 2023 Davidson <twister@davidson.fr>
// SPDX-License-Identifier: GPL-3.0-or-later

use toml::Table;

use super::etsdiff::ETSdiff;
use super::iteration_scheduler::SchedulerType;
use super::service::Service;
use super::system_call::SystemCall;
use super::test::SystemCallTest;

pub trait ConfigReader {
    fn read(config: &str, etsd: &mut ETSdiff);
}

// ===
pub struct TOMLConfigReader<'a> {
    toml: &'a Table,
    etsd: &'a mut ETSdiff,
}

impl<'a> ConfigReader for TOMLConfigReader<'a> {
    fn read(config: &str, etsd: &mut ETSdiff) {
        let mut cr = TOMLConfigReader {
            toml: &config.parse::<Table>().unwrap(),
            etsd,
        };

        if let Some(table) = cr.toml["Scheduler"].as_table() {
            cr.read_scheduler(table);
        }

        if cr.toml.contains_key("Services") {
            if let Some(table) = cr.toml["Services"].as_table() {
                for name in table.keys() {
                    cr.read_service(name, table[name].as_table().unwrap());
                }
            }
        }

        if cr.toml.contains_key("Tests") {
            if let Some(table) = cr.toml["Tests"].as_table() {
                for name in table.keys() {
                    cr.read_test(name, table[name].as_table().unwrap());
                }
            }
        }
    }
}

impl<'a> TOMLConfigReader<'a> {
    fn read_scheduler(&mut self, toml_scheduler: &Table) {
        if let Some(nb_iteration) = toml_scheduler["nb_iteration"].as_integer() {
            if let Some(stype) = toml_scheduler["type"].as_str() {
                if stype == "StageredScheduler" {
                    self.etsd
                        .set_scheduler(SchedulerType::StageredScheduler, nb_iteration as u32);
                }
            }
        }
    }

    fn read_service(&mut self, name: &str, toml_service: &Table) {
        let mut s = Service::new(name);

        if toml_service.contains_key("process_name") {
            s.set_process_name(toml_service["process_name"].as_str().unwrap());
        }

        if toml_service.contains_key("ports") {
            for port in toml_service["ports"].as_array().unwrap() {
                if let Some(port) = port.as_integer() {
                    s.add_port(port as u32);
                }
            }
        }

        if toml_service.contains_key("prepare") {
            s.prepare = Some(SystemCall::new(toml_service["prepare"].as_str().unwrap()));
        }

        if toml_service.contains_key("clean") {
            s.clean = Some(SystemCall::new(toml_service["clean"].as_str().unwrap()));
        }

        if toml_service.contains_key("release") {
            s.release = Some(SystemCall::new(toml_service["release"].as_str().unwrap()));
        }

        if toml_service.contains_key("storage_paths") {
            for path in toml_service["storage_paths"].as_array().unwrap() {
                s.add_storage_path(path.as_str().unwrap());
            }
        }

        let mut services = self.etsd.services.borrow_mut();
        services.push(s);
    }

    fn read_test(&mut self, name: &str, toml_test: &Table) {
        if toml_test.contains_key("type") && toml_test["type"].as_str().unwrap() == "SystemCall" {
            let mut test = SystemCallTest::new(name, toml_test["command_line"].as_str().unwrap());

            if toml_test.contains_key("services_names") {
                for sn in toml_test["services_names"].as_array().unwrap() {
                    test.add_service_name(sn.as_str().unwrap());
                }
            }
            self.etsd.tests.push(Box::new(test));
        }

        // TODO: Error report test without type
    }
}

// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::etsdiff::ETSdiff;

    static TOML_TEST: &str = r#"
[Scheduler]
type = "StageredScheduler"
nb_iteration = 5

    
[Services]
    
[Services."Service 1"]
ports = [ 8_080, 4_326 ]
prepare = "ls -a -l"
release = "ls"
storage_paths = [ "/tmp/s1/queue", "/tmp/s1/session" ]

[Services."Service 2"]
process_name = "pns2"
clean = "ls -a"

[Services."Service 3"]
process_name = "pns3"
clean = "ls -l"


[Tests]

[Tests."Test 1"]
type = "SystemCall"
services_names = [ "Service 1", "Service 2" ]
command_line = "/bin/ls -a -l"

[Tests."Test 2"]
type = "SystemCall"
services_names = [ "Service 3" ]
command_line = "ls"
    "#;

    #[test]
    fn toml_config_reader_services() {
        let mut etsd = ETSdiff::new();

        TOMLConfigReader::read(TOML_TEST, &mut etsd);

        let services = etsd.services.borrow();
        assert_eq!(3, services.len());

        // 1st service
        let s1 = &services[0];
        assert_eq!("Service 1", s1.name);

        assert!(s1.process_name.is_none());

        assert_eq!(2, s1.ports.len());
        assert_eq!(8080, s1.ports[0]);
        assert_eq!(4326, s1.ports[1]);

        assert!(s1.prepare.is_some());
        let prepare = s1.prepare.as_ref().unwrap();
        assert_eq!(prepare.path(), "ls");
        assert_eq!(prepare.arguments(), ["-a", "-l"]);

        assert!(s1.clean.is_none());

        assert!(s1.release.is_some());
        let release = s1.release.as_ref().unwrap();
        assert_eq!(release.path(), "ls");
        assert_eq!(0, release.arguments().len());

        assert_eq!(2, s1.storage_paths.len());
        assert_eq!("/tmp/s1/queue", s1.storage_paths[0]);
        assert_eq!("/tmp/s1/session", s1.storage_paths[1]);

        // 2nd service
        let s2 = &services[1];
        assert_eq!("Service 2", s2.name);

        assert!(s2.process_name.is_some());
        assert_eq!("pns2", s2.process_name.as_ref().unwrap());

        assert_eq!(0, s2.ports.len());

        assert!(s2.prepare.is_none());

        assert!(s2.clean.is_some());
        let clean = s2.clean.as_ref().unwrap();
        assert_eq!(clean.path(), "ls");
        assert_eq!(clean.arguments(), ["-a"]);

        assert!(s2.release.is_none());

        assert_eq!(0, s2.storage_paths.len());

        // 3rd service
        let s3 = &services[2];
        assert_eq!("Service 3", s3.name);

        assert!(s3.process_name.is_some());
        assert_eq!("pns3", s3.process_name.as_ref().unwrap());

        assert_eq!(0, s3.ports.len());

        assert!(s3.prepare.is_none());

        assert!(s3.clean.is_some());
        let clean = s3.clean.as_ref().unwrap();
        assert_eq!(clean.path(), "ls");
        assert_eq!(clean.arguments(), ["-l"]);

        assert!(s3.release.is_none());

        assert_eq!(0, s3.storage_paths.len());
    }

    #[test]
    fn toml_config_reader_tests() {
        let mut etsd = ETSdiff::new();

        TOMLConfigReader::read(TOML_TEST, &mut etsd);

        assert_eq!(2, etsd.tests.len());

        // 1st test
        assert_eq!("Test 1", etsd.tests[0].name());
        assert_eq!(2, etsd.tests[0].services_names().len());
        let mut i = 0;
        for service_name in etsd.tests[0].services_names().iter() {
            match i {
                0 => assert_eq!(service_name, "Service 1"),
                1 => assert_eq!(service_name, "Service 2"),
                _ => panic!("Unexpected service name"),
            }
            i += 1;
        }
        assert!(!&etsd.tests[0].run().is_err());

        // 2nd test
        assert_eq!("Test 2", etsd.tests[1].name());
        assert_eq!(1, etsd.tests[1].services_names().len());
        let mut i = 0;
        for service_name in etsd.tests[1].services_names().iter() {
            match i {
                0 => assert_eq!(service_name, "Service 3"),
                _ => panic!("Unexpected service name"),
            }
            i += 1;
        }
        assert!(!&etsd.tests[1].run().is_err());
    }

    #[test]
    fn toml_config_reader_iteration_scheduler() {
        let mut etsd = ETSdiff::new();

        TOMLConfigReader::read(TOML_TEST, &mut etsd);

        assert_eq!(5, etsd.scheduler.unwrap().nb_iteration());
    }
}

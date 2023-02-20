// SPDX-FileCopyrightText: 2023 Davidson <twister@davidson.fr>
// SPDX-License-Identifier: GPL-3.0-or-later

use yaml_rust::yaml::Yaml;
use yaml_rust::YamlLoader;

use super::etsdiff::ETSdiff;
use super::iteration_scheduler::SchedulerType;
use super::service::Service;
use super::system_call::SystemCall;
use super::test::{SystemCallTest, Test};

pub trait ConfigReader {
    fn read(config: &str, etsd: &mut ETSdiff);
}

// ===

pub struct YAMLConfigReader<'a> {
    root_yaml: &'a Yaml,
    etsd: &'a mut ETSdiff,
}

impl<'a> ConfigReader for YAMLConfigReader<'a> {
    fn read(config: &str, etsd: &mut ETSdiff) {
        let mut cr = YAMLConfigReader {
            root_yaml: &YamlLoader::load_from_str(config).unwrap()[0],
            etsd,
        };

        if !cr.root_yaml["Scheduler"].is_badvalue() {
            cr.read_scheduler(&cr.root_yaml["Scheduler"]);
        }

        if !cr.root_yaml["Services"].is_badvalue() {
            for service in cr.root_yaml["Services"].as_vec().unwrap() {
                cr.read_service(service);
            }
        }

        if !cr.root_yaml["Tests"].is_badvalue() {
            for test in cr.root_yaml["Tests"].as_vec().unwrap() {
                cr.read_test(test);
            }
        }
    }
}

impl<'a> YAMLConfigReader<'a> {
    fn read_service(&mut self, yaml_service: &Yaml) {
        let mut s = Service::new(yaml_service["name"].as_str().unwrap()); // TODO: What's if no name provided

        if !yaml_service["process_name"].is_badvalue() {
            s.set_process_name(yaml_service["process_name"].as_str().unwrap());
        }

        if !yaml_service["ports"].is_badvalue() {
            for port in yaml_service["ports"].as_vec().unwrap() {
                match u32::try_from(port.as_i64().unwrap()) {
                    Ok(p) => s.add_port(p),
                    Err(_e) => (), // TODO: Error managment
                }
            }
        }

        if !yaml_service["prepare"].is_badvalue() {
            s.prepare = Some(SystemCall::new(yaml_service["prepare"].as_str().unwrap()));
        }

        if !yaml_service["clean"].is_badvalue() {
            s.clean = Some(SystemCall::new(yaml_service["clean"].as_str().unwrap()));
        }

        if !yaml_service["release"].is_badvalue() {
            s.release = Some(SystemCall::new(yaml_service["release"].as_str().unwrap()));
        }

        if !yaml_service["storage_paths"].is_badvalue() {
            for path in yaml_service["storage_paths"].as_vec().unwrap() {
                s.add_storage_path(path.as_str().unwrap());
            }
        }

        let mut services = self.etsd.services.borrow_mut();
        services.push(s);
    }

    fn read_test(&mut self, yaml_test: &Yaml) {
        if !yaml_test["type"].is_badvalue() && yaml_test["type"].as_str().unwrap() == "SystemCall" {
            let mut test = SystemCallTest::new(
                yaml_test["name"].as_str().unwrap(),
                yaml_test["command_line"].as_str().unwrap(),
            );
            dbg!(test.name());

            if !yaml_test["services_names"].is_badvalue() {
                for sn in yaml_test["services_names"].as_vec().unwrap() {
                    dbg!(sn);
                    test.add_service_name(sn.as_str().unwrap());
                }
            }
            self.etsd.tests.push(Box::new(test));
        }

        // TODO: Error report test without type
    }

    fn read_scheduler(&mut self, yaml_scheduler: &Yaml) {
        if !yaml_scheduler["type"].is_badvalue() && !yaml_scheduler["nb_iteration"].is_badvalue() {
            match u32::try_from(yaml_scheduler["nb_iteration"].as_i64().unwrap()) {
                Ok(nb_iteration) => {
                    if yaml_scheduler["type"].as_str().unwrap() == "StageredScheduler" {
                        self.etsd
                            .set_scheduler(SchedulerType::StageredScheduler, nb_iteration);
                    }
                }
                Err(_e) => (), // TODO: Error managment
            }
        }
    }
}

// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::etsdiff::ETSdiff;

    static YAML_TEST: &str = "
Scheduler:
  type: \"StageredScheduler\"
  nb_iteration: 5
Services:
  -
    name: \"Service 1\"
    ports:
      - 8080
      - 4326
    prepare: \"ls -a -l\"
    release: ls
    storage_paths:
      - /tmp/s1/queue
      - /tmp/s1/session
  -
    name: \"Service 2\"
    process_name: \"pns2\"
    clean: \"ls -a\"
  -
    name: \"Service 3\"
    process_name: \"pns3\"
    clean: \"ls -l\"
Tests:
  -
    type: SystemCall
    name: \"Test 1\"
    services_names:
      - \"Service 1\"
      - \"Service 2\"
    command_line: \"/bin/ls -a -l\"
  -
    type: SystemCall
    name: \"Test 2\"
    services_names:
      - \"Service 3\"
    command_line: ls
";

    #[test]
    fn yaml_config_reader_services() {
        let mut etsd = ETSdiff::new();

        YAMLConfigReader::read(YAML_TEST, &mut etsd);

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
    fn yaml_config_reader_tests() {
        let mut etsd = ETSdiff::new();

        YAMLConfigReader::read(YAML_TEST, &mut etsd);

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
    fn yaml_config_reader_iteration_scheduler() {
        let mut etsd = ETSdiff::new();

        YAMLConfigReader::read(YAML_TEST, &mut etsd);

        assert_eq!(5, etsd.scheduler.unwrap().nb_iteration());
    }
}

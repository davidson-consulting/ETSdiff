// SPDX-FileCopyrightText: 2023 Davidson <twister@davidson.fr>
// SPDX-License-Identifier: GPL-3.0-or-later

use cgroups_rs::{Cgroup, CgroupPid};
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;
use sysinfo::{System, SystemExt};

use super::system_call::SystemCall;

pub type ServicesLink = Rc<RefCell<Vec<Service>>>;

// ===

pub struct Service {
    pub name: String,
    pub process_name: Option<String>,
    cgroup: Option<Cgroup>,
    pub ports: Vec<u32>,
    pub prepare: Option<SystemCall>,
    pub clean: Option<SystemCall>,
    pub release: Option<SystemCall>,
    pub storage_paths: Vec<String>,
}

impl Service {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            process_name: None,
            cgroup: None,
            ports: Vec::new(),
            prepare: None,
            clean: None,
            release: None,
            storage_paths: Vec::new(),
        }
    }

    pub fn set_process_name(&mut self, process_name: &str) {
        self.process_name = Some(process_name.into());
    }

    pub fn add_port(&mut self, port: u32) {
        self.ports.push(port);
    }

    pub fn add_storage_path(&mut self, path: &str) {
        self.storage_paths.push(path.into());
    }

    pub fn prepare(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(ref mut sc) = self.prepare {
            sc.execute()?;
        }

        #[cfg(not(tarpaulin))]
        if let Some(ref pname) = self.process_name {
            let cg = Cgroup::new(
                cgroups_rs::hierarchies::auto(),
                format!("controlled.slice/{pname}"),
            );

            let mut s = System::new();

            for _i in 0..10 {
                let process_list = s.get_process_by_name(pname);
                if !process_list.is_empty() {
                    for process in process_list {
                        cg.add_task(CgroupPid::from(process.pid as u64)).unwrap();
                    }
                    break;
                }
                std::thread::sleep(std::time::Duration::from_millis(500));
                s.refresh_processes();
            }

            self.cgroup = Some(cg);
        }

        Ok(())
    }

    pub fn clean(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(ref mut sc) = self.clean {
            sc.execute()?;
        }

        Ok(())
    }

    pub fn release(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(ref mut sc) = self.release {
            sc.execute()?;
        }

        #[cfg(not(tarpaulin))]
        if let Some(ref mut cg) = self.cgroup {
            if let Some(ref pname) = self.process_name {
                let s = System::new();
                for process in s.get_process_by_name(pname) {
                    cg.remove_task(CgroupPid::from(process.pid as u64));
                }
            }
            if cg.delete().is_err() {
                eprintln!("Can't delete cgroup");
            }
            self.cgroup = None;
        }

        Ok(())
    }
}

// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    use std::path::Path;

    #[test]
    fn service_creation() {
        let s = Service::new("Test Service");

        assert_eq!("Test Service", s.name);
        assert!(s.process_name.is_none());
        assert_eq!(0, s.ports.len());
        assert!(s.prepare.is_none());
        assert!(s.clean.is_none());
        assert!(s.release.is_none());
        assert_eq!(0, s.storage_paths.len());
    }

    #[test]
    fn service_process_name() {
        let mut s = Service::new("Test Service");

        assert!(s.process_name.is_none());

        s.set_process_name("pidof");

        assert!(s.process_name.is_some());
        assert_eq!("pidof", s.process_name.unwrap());
    }

    #[test]
    fn service_add_ports() {
        let mut s = Service::new("Test Service");

        assert_eq!(0, s.ports.len());

        s.add_port(8080);

        assert_eq!(1, s.ports.len());

        s.add_port(4326);

        assert_eq!(2, s.ports.len());
    }

    #[test]
    fn service_add_storage_path() {
        let mut s = Service::new("Test Service");

        assert_eq!(0, s.storage_paths.len());

        s.add_storage_path("/path1");

        assert_eq!(1, s.storage_paths.len());

        s.add_storage_path("/path2");

        assert_eq!(2, s.storage_paths.len());
    }

    #[test]
    fn service_prepare() {
        let mut s = Service::new("Test Service");

        assert!(!s.prepare().is_err());

        s.prepare = Some(SystemCall::new("ls"));

        assert!(!s.prepare().is_err());
    }

    #[test]
    fn service_clean() {
        let mut s = Service::new("Test Service");

        assert!(!s.clean().is_err());

        s.clean = Some(SystemCall::new("ls -s"));

        assert!(!s.clean().is_err());
    }

    #[test]
    fn service_release() {
        let mut s = Service::new("Test Service");

        assert!(!s.release().is_err());

        s.release = Some(SystemCall::new("ls -a"));

        assert!(!s.release().is_err());
    }

    #[test]
    #[cfg(not(tarpaulin))]
    fn service_cgroups() {
        let test_path = Path::new("/sys/fs/cgroup/controlled.slice/cargo");

        let mut s = Service::new("Test Service");
        s.set_process_name("cargo"); // Assuming test with run with cargo

        s.prepare().unwrap();
        assert_eq!(true, test_path.is_dir());
        if let Some(ref cg) = s.cgroup {
            assert!(cg.tasks().len() > 0);
        }

        s.release().unwrap();
        assert_eq!(false, test_path.is_dir());
    }
}

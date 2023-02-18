// SPDX-FileCopyrightText: 2023 Davidson <twister@davidson.fr>
// SPDX-License-Identifier: GPL-3.0-or-later

use fs_extra::dir::get_size;
use inotify::{Inotify, WatchMask};
use rtshark::{RTShark, RTSharkBuilder};
use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::str::FromStr;
use systemctl;

use super::service::{Service, ServicesLink};

pub trait ETSComponent {
    fn min_iteration(&self) -> i32 {
        1
    }
    fn value(&self) -> f64 {
        0.0
    }
    fn before_campaign(&mut self) {}
    fn after_campaign(&mut self) {}
    fn before_test(&mut self) {}
    fn after_test(&mut self) {}
}

// ===

const VJOULE_SERVICE_NAME: &str = "vjoule_service";

pub struct EComponent {
    values: Vec<f64>,
    services: Weak<RefCell<Vec<Service>>>,
    vjoule_need_stop: bool,
}

impl ETSComponent for EComponent {
    fn value(&self) -> f64 {
        self.values.iter().sum()
    }
    fn before_campaign(&mut self) {
        if !systemctl::is_active(VJOULE_SERVICE_NAME).unwrap() {
            self.vjoule_need_stop = true;
            systemctl::restart(VJOULE_SERVICE_NAME).unwrap();
            // TODO better implementation need to wait service started to inotify results/cpu
            std::thread::sleep(std::time::Duration::from_millis(1000));
            if !systemctl::is_active(VJOULE_SERVICE_NAME).unwrap() {
                panic!("Can't start vjoule_service");
            }
        }
        self.wait_vjoule_signal();
    }
    fn after_campaign(&mut self) {
        if self.vjoule_need_stop {
            systemctl::stop(VJOULE_SERVICE_NAME).unwrap();
        }
    }
    fn before_test(&mut self) {
        let services_rc = Weak::upgrade(&self.services).unwrap();
        let services = services_rc.borrow();
        self.values.resize(services.len(), 0.0);
        self.wait_vjoule_signal();
        let mut i = 0;
        for s in &*services {
            if let Some(pn) = &s.process_name {
                let cpu_s = std::fs::read_to_string(format!(
                    "/etc/vjoule/results/controlled.slice/{pn}/cpu"
                ))
                .unwrap();
                self.values[i] = cpu_s[..cpu_s.len() - 1].parse::<f64>().unwrap();
                i += 1;
            }
        }
    }
    fn after_test(&mut self) {
        let services_rc = Weak::upgrade(&self.services).unwrap();
        let services = services_rc.borrow();
        self.wait_vjoule_signal();
        let mut i = 0;
        for s in &*services {
            if let Some(pn) = &s.process_name {
                let cpu_s = std::fs::read_to_string(format!(
                    "/etc/vjoule/results/controlled.slice/{pn}/cpu"
                ))
                .unwrap();
                self.values[i] = cpu_s[..cpu_s.len() - 1].parse::<f64>().unwrap() - self.values[i];
                i += 1;
            }
        }
    }
}

impl EComponent {
    pub fn new(services: &ServicesLink) -> Self {
        if !systemctl::exists(VJOULE_SERVICE_NAME).unwrap() {
            panic!("EComponent require installation of vjoule");
        }

        Self {
            values: vec![0.0],
            services: Rc::<RefCell<Vec<Service>>>::downgrade(services),
            vjoule_need_stop: false,
        }
    }
    pub fn to_joules(&self) -> f64 {
        self.value()
    }
    fn wait_vjoule_signal(&self) {
        let mut inotify = Inotify::init().expect("Error while initializing inotify instance");

        inotify
            .add_watch("/etc/vjoule/results/cpu", WatchMask::MODIFY)
            .expect("Failed to add watch");

        let mut buffer = [0; 1024];
        inotify
            .read_events_blocking(&mut buffer)
            .expect("Error while reading events");
    }
}

// ===

pub trait OctetsComponent: ETSComponent {
    fn to_octets(&self) -> u64 {
        0
    }
}

// ===

pub struct TComponent {
    value: u64,
    services: Weak<RefCell<Vec<Service>>>,
    rtshark: Option<RTShark>,
}

impl ETSComponent for TComponent {
    fn before_test(&mut self) {
        self.value = 0;

        let mut filter = String::from("");
        let services_rc = Weak::upgrade(&self.services).unwrap();
        let services = services_rc.borrow();
        for s in &*services {
            for p in &s.ports {
                if !filter.is_empty() {
                    filter = format!("{filter} or port {p}");
                } else {
                    filter = format!("port {p}");
                }
            }
        }
        filter = format!("host 127.0.0.1 and ({filter})");

        let builder = RTSharkBuilder::builder()
            .input_path("any")
            .output_path("/tmp/etsdiff.pcap")
            .live_capture()
            .capture_filter(&filter);

        match builder.spawn() {
            Err(err) => {
                eprintln!("Error running tshark writter: {err}");
            }
            Ok(rtshark) => {
                self.rtshark = Some(rtshark);
                std::thread::sleep(std::time::Duration::from_millis(1000)); // TODO better implementation
            }
        };
    }
    fn after_test(&mut self) {
        match &mut self.rtshark {
            Some(s) => {
                s.kill();
                self.rtshark = None;
            }
            None => (),
        }
        std::thread::sleep(std::time::Duration::from_millis(1000)); // TODO better implementation

        let builder = RTSharkBuilder::builder().input_path("/tmp/etsdiff.pcap");
        let mut rtshark = match builder.spawn() {
            Err(err) => {
                eprintln!("Error running tshark reader: {err}");
                return;
            }
            Ok(rtshark) => rtshark,
        };
        while let Some(packet) = rtshark.read().unwrap_or_else(|e| {
            eprintln!("Error parsing TShark output: {e}");
            None
        }) {
            for layer in packet {
                if layer.name() == "frame" {
                    if let Some(fl) = layer.metadata("frame.len") {
                        self.value += u64::from_str(fl.value()).unwrap_or(0);
                    }
                }
            }
        }

        std::fs::remove_file("/tmp/etsdiff.pcap").expect("No /tmp/etsdiff.pcap file to delete");
    }

    fn value(&self) -> f64 {
        self.to_octets() as f64
    }
}

impl OctetsComponent for TComponent {
    fn to_octets(&self) -> u64 {
        self.value
    }
}

impl TComponent {
    pub fn new(services: &ServicesLink) -> Self {
        Self {
            value: 0,
            services: Rc::<RefCell<Vec<Service>>>::downgrade(services),
            rtshark: None,
        }
    }
}

// ===

pub struct SComponent {
    services: Weak<RefCell<Vec<Service>>>,
}

impl ETSComponent for SComponent {
    fn value(&self) -> f64 {
        self.to_octets() as f64
    }
}

impl OctetsComponent for SComponent {
    fn to_octets(&self) -> u64 {
        let mut size: u64 = 0;
        let services_rc = Weak::upgrade(&self.services).unwrap();
        let services = services_rc.borrow();

        for s in &*services {
            for p in &s.storage_paths {
                size += get_size(p).unwrap();
            }
        }

        size
    }
}

impl SComponent {
    pub fn new(services: &ServicesLink) -> Self {
        Self {
            services: Rc::<RefCell<Vec<Service>>>::downgrade(services),
        }
    }
}

// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    use rouille::Server;
    use std::error::Error;
    use sysinfo::{System, SystemExt};

    use crate::ets::system_call::SystemCall;

    #[test]
    #[cfg(not(tarpaulin))]
    fn ecomponent_value() -> Result<(), Box<dyn Error>> {
        let si = System::new();

        let mut services: Vec<Service> = Vec::new();
        // find current process name
        let mut s = Service::new("Test 1");
        if let Some(p) = si.get_process(std::process::id() as i32) {
            s.set_process_name(&p.name);
        }

        services.push(s);

        let link: ServicesLink = Rc::new(RefCell::new(services));
        let mut ec = EComponent::new(&link);

        {
            let mut services = link.borrow_mut();
            for s in &mut *services {
                if s.prepare().is_err() {
                    eprintln!("Can't prepare service");
                }
            }
        }
        ec.before_campaign();
        ec.before_test();

        use std::{thread, time};
        let mut sc = SystemCall::new("ps aux");
        for _i in 0..10 {
            if sc.execute().is_err() {
                eprintln!("Can't do 'ps aux' for test");
            }
            thread::sleep(time::Duration::from_millis(100));
        }

        ec.after_test();
        {
            let mut services = link.borrow_mut();
            for s in &mut *services {
                if s.release().is_err() {
                    eprintln!("Can't release service");
                }
            }
        }

        assert!(ec.to_joules() > 0.0);
        println!("E.value => {}", ec.value());

        ec.after_campaign();

        Ok(())
    }

    #[test]
    #[cfg(not(tarpaulin))]
    fn tcomponent_value() -> Result<(), Box<dyn Error>> {
        // Create webserver
        let server1 = Server::new("localhost:8881", |request| {
            router!(request,
                (GET) (/simple) => {
                    println!("simple 0123456789");
                    rouille::Response::text("0123456789")
                },
                _ => rouille::Response::empty_404()
            )
        })
        .unwrap();
        println!("Listening on {:?}", server1.server_addr());
        let (handle1, sender1) = server1.stoppable();
        let server2 = Server::new("localhost:8882", |request| {
            router!(request,
                (GET) (/double) => {
                    println!("double 01234567890123456789");
                    rouille::Response::text("01234567890123456789")
                },
                _ => rouille::Response::empty_404()
            )
        })
        .unwrap();
        println!("Listening on {:?}", server2.server_addr());
        let (handle2, sender2) = server2.stoppable();

        // Creat TComponent with service
        let mut services: Vec<Service> = Vec::new();

        let mut s = Service::new("Test 1");
        s.add_port(8881);
        s.add_port(8882);
        services.push(s);

        let link: ServicesLink = Rc::new(RefCell::new(services));
        let mut tc = TComponent::new(&link);

        // Do tests requests
        tc.before_test();
        let res = reqwest::blocking::get("http://localhost:8881/simple").unwrap();
        let body = res.text().unwrap();
        assert_eq!("0123456789", body);
        let res = reqwest::blocking::get("http://localhost:8882/double").unwrap();
        let body = res.text().unwrap();
        assert_eq!("01234567890123456789", body);
        tc.after_test();
        let t1 = tc.to_octets();
        assert!(t1 > "0123456789".len() as u64);

        tc.before_test();
        let res = reqwest::blocking::get("http://localhost:8881/simple").unwrap();
        let body = res.text().unwrap();
        assert_eq!("0123456789", body);
        let res = reqwest::blocking::get("http://localhost:8882/double").unwrap();
        let body = res.text().unwrap();
        assert_eq!("01234567890123456789", body);
        tc.after_test();
        let t2 = tc.to_octets();
        assert!(t2 > "0123456789".len() as u64);
        assert_eq!(t1, t2);

        // Stopping webserver
        sender2.send(()).unwrap();
        handle2.join().unwrap();
        sender1.send(()).unwrap();
        handle1.join().unwrap();

        Ok(())
    }

    #[test]
    fn scomponent_min_iteration() {
        let services: Vec<Service> = Vec::new();
        let link: ServicesLink = Rc::new(RefCell::new(services));
        let sc = SComponent::new(&link);

        assert_eq!(1, sc.min_iteration());
    }

    #[test]
    fn scomponent_value() -> Result<(), Box<dyn Error>> {
        // creating some test files/paths
        let mut scall = SystemCall::new("mkdir -p /tmp/etsdiff/test2");
        scall.execute()?;
        scall = SystemCall::new("truncate -s 1 /tmp/etsdiff/test1.size");
        scall.execute()?;
        scall = SystemCall::new("truncate -s 2 /tmp/etsdiff/test2/1.size");
        scall.execute()?;
        scall = SystemCall::new("truncate -s 3 /tmp/etsdiff/test2/2.size");
        scall.execute()?;
        scall = SystemCall::new("truncate -s 1 /tmp/etsdiff/test3_1.size");
        scall.execute()?;
        scall = SystemCall::new("truncate -s 3 /tmp/etsdiff/test3_2.size");
        scall.execute()?;

        let mut services: Vec<Service> = Vec::new();

        let mut s = Service::new("Test 1");
        s.add_storage_path("/tmp/etsdiff/test1.size");
        services.push(s);

        s = Service::new("Test 2");
        s.add_storage_path("/tmp/etsdiff/test2/");
        services.push(s);

        s = Service::new("Test 3");
        s.add_storage_path("/tmp/etsdiff/test3_1.size");
        s.add_storage_path("/tmp/etsdiff/test3_2.size");
        services.push(s);

        assert_eq!(3, services.len());

        let link: ServicesLink = Rc::new(RefCell::new(services));
        let sc = SComponent::new(&link);
        assert_eq!(10, sc.to_octets());
        assert_eq!(10.0, sc.value());

        // cleaning test files/paths
        scall = SystemCall::new("rm -rf /tmp/etsdiff");
        scall.execute()?;

        Ok(())
    }
}

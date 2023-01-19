// SPDX-FileCopyrightText: 2023 Davidson <twister@davidson.fr>
// SPDX-License-Identifier: GPL-3.0-or-later

// TODO: remove when finished
//#![allow(unused)]

#[macro_use]
extern crate simple_error;
extern crate yaml_rust;

#[cfg(test)]
#[macro_use]
extern crate rouille;

use clap::{arg, value_parser, Command};
use std::env;
use std::fs::File;
use std::path::{Path, PathBuf};

pub mod ets;

use crate::ets::config_reader::{ConfigReader, YAMLConfigReader};
use crate::ets::etsdiff::ETSdiff;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("ETSDiff")
        .version("0.1")
        .author("Twister <twister@davidson.fr>")
        .about("Comparing programs with 3 criterias: Energy, Transfer and Storage")
        .arg(
            arg!([config] "YAML config file")
                .required(true)
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(-o --output <FILE> "YAML output file")
                .required(false)
                .value_parser(value_parser!(String)),
        )
        .get_matches();

    if let Some(config) = matches.get_one::<PathBuf>("config") {
        if !config.is_file() {
            eprintln!("Error: config file \"{}\" not found", config.display());
            std::process::exit(1);
        }

        let exec_path = env::current_dir().unwrap();
        let path = Path::new(config).parent().unwrap();
        assert!(env::set_current_dir(&path).is_ok());

        let yalm_config = std::fs::read_to_string(config).expect("could not read config file");

        let mut etsd = ETSdiff::new();
        YAMLConfigReader::read(&yalm_config, &mut etsd);

        match etsd.execute() {
            Err(e) => {
                eprintln!("Error while executing etsdiff...");
                eprintln!("{:?}", e);
                std::process::exit(1);
            }
            _ => (),
        }

        println!("\n=====\n");
        println!("{}", serde_yaml::to_string(&etsd.report)?);

        if let Some(output) = matches.get_one::<String>("output") {
            assert!(env::set_current_dir(exec_path).is_ok());
            serde_yaml::to_writer(&File::create(output)?, &etsd.report)?;
        }
    }

    Ok(())
}

// SPDX-FileCopyrightText: 2023 Davidson <twister@davidson.fr>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::error::Error;
use std::ffi::OsStr;
use std::process::{Command, Stdio};

pub struct SystemCall {
    command: Command,
}

impl SystemCall {
    pub fn new(commandline: &str) -> Self {
        let mut cmd = commandline.split_whitespace().collect::<Vec<&str>>();
        let args = cmd.split_off(1);
        let mut command = Command::new(cmd[0]);
        command.stdout(Stdio::null());
        command.stderr(Stdio::null());

        for i in &args {
            command.arg(i);
        }
        Self { command: command }
    }

    pub fn path(&self) -> String {
        let path = self.command.get_program().to_str().map(|s| s.to_string());

        match path {
            Some(path) => path,
            None => String::new(),
        }
    }

    pub fn arguments(&self) -> Vec<String> {
        let args: Vec<&OsStr> = self.command.get_args().collect();
        let mut ret: Vec<String> = Vec::new();

        for i in &args {
            let s = i.to_str().map(|s| s.to_string());
            match s {
                Some(s) => ret.push(s),
                None => (),
            }
        }
        ret
    }

    pub fn execute(&mut self) -> Result<(), Box<dyn Error>> {
        let status = self.command.status()?;

        if !status.success() {
            bail!("SystemCall.execute() return error");
        }

        Ok(())
    }
}

// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creation_from_command_line() {
        let sc = SystemCall::new("/bin/ls -l -u ets.rs");
        assert_eq!(sc.path(), "/bin/ls");
        assert_eq!(sc.arguments(), ["-l", "-u", "ets.rs"]);
    }

    #[test]
    fn execute_with_unknow_command() {
        let mut sc = SystemCall::new("/unknowpath/unknowcommand");
        assert!(sc.execute().is_err());
    }

    #[test]
    fn execute_command_that_failed() {
        let mut sc = SystemCall::new("ls /unknowpath/unknowcommand");
        assert!(sc.execute().is_err());
    }

    #[test]
    fn execute_with_success() {
        let mut sc = SystemCall::new("ls -l");
        assert!(!sc.execute().is_err());
    }
}

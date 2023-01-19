use std::error::Error;

use super::system_call::SystemCall;

pub trait Test {
    fn name(&self) -> &String;
    fn run(&mut self) -> Result<(), Box<dyn Error>>;
}

// ===

pub struct SystemCallTest {
    name: String,
    system_call: SystemCall,
}

impl SystemCallTest {
    pub fn new(name: &str, commandline: &str) -> Self {
        Self {
            name: name.into(),
            system_call: SystemCall::new(commandline),
        }
    }
}

impl Test for SystemCallTest {
    fn name(&self) -> &String {
        &self.name
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
        assert_eq!(sct.system_call.path(), "ls");
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

extern crate libc;
extern crate log;

#[macro_use]
extern crate cfg_if;

use std::error::Error;
use common::config::Config;
use std::cell::RefCell;

use log::{info, warn};

pub mod trace;
pub mod process;
pub mod error;

use crate::process::Process;
use crate::error::{DebugError, Reason};

pub struct Debugger {
    process: RefCell<Option<Process>>,
    file: Option<String>,
}

impl Debugger {
    pub fn from_config<C: Into<Config>>(cfg: C) -> Self {
        let cfg = cfg.into();

        let process = if cfg.should_attach {
            Some(Process::attach(cfg.pid.unwrap()))
        } else {
            None
        };

        Self {
            process: RefCell::new(process),
            file: cfg.file,
        }
    }

    pub fn run(&self, args: Vec<String>, env: Vec<String>) -> Result<(), DebugError> {
        {
            let maybe_process = self.process.borrow();
            if maybe_process.is_some() {
                return Err(DebugError::CannotStart(Reason::ProcessExists));
            }
        }

        match &self.file {
            Some(file) => {
                let process = Process::start(file.clone(), args, Some(env))?;
                info!("started process: {}", process.pid);
                self.process.replace(Some(process));
                Ok(())
            }
            None => return Err(DebugError::CannotStart(Reason::NoFile).into())
        }
    }

    pub fn proceed(&self) {
        if let Some(process) = &*self.process.borrow() {
            process.proceed()
        }
    }
}
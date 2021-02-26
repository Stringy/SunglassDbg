extern crate libc;
extern crate log;

#[macro_use]
extern crate cfg_if;

use std::path::PathBuf;
use crate::process::Process;
use std::error::Error;
use std::borrow::Borrow;
use common::config::Config;
use std::cell::RefCell;

use log::{info, warn};

pub mod trace;
pub mod process;

pub struct Debugger {
    process: RefCell<Option<Process>>,
    file: String,
}

impl Debugger {
    pub fn from_config<C: Into<Config>>(cfg: C) -> Self {
        let cfg = cfg.into();

        let process = if cfg.should_attach {
            Some(Process::attach(cfg.pid.unwrap()))
        } else {
            None
        };

        let file = match cfg.file {
            Some(f) => f,
            None => String::new(),
        };


        Self {
            process: RefCell::new(process),
            file,
        }
    }

    pub fn run(&self, args: Vec<String>, env: Vec<String>) -> Result<(), Box<dyn Error>> {
        {
            let maybe_process = self.process.borrow();
            if maybe_process.is_some() {
                warn!("Already tracing a process");
                return Ok(());
            }
        }

        let process = Process::start(self.file.clone(), args, Some(env))?;
        info!("started process: {}", process.pid);
        self.process.replace(Some(process));
        Ok(())
    }

    pub fn proceed(&self) {
        if let Some(process) = &*self.process.borrow() {
            process.proceed()
        }
    }
}
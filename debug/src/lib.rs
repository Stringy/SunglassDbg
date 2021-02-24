extern crate libc;

#[macro_use]
extern crate cfg_if;

use std::path::PathBuf;
use crate::process::Process;
use std::error::Error;
use std::borrow::Borrow;
use common::config::Config;

pub mod trace;
pub mod process;

pub struct Debugger {
    process: Option<Process>,
    file: String,
}

impl Debugger {
    pub fn from_config<C: Into<Config>>(cfg: C) -> Self {
        let cfg = cfg.into();

        let process = if cfg.should_attach {
            Process::attach(cfg.pid.unwrap())
        } else {
            Default::default()
        };

        let file = match cfg.file {
            Some(f) => f,
            None => String::new(),
        };


        Self {
            process: Some(process),
            file,
        }
    }

    pub fn start<P: Into<PathBuf>>(path: P) -> Self {
        let path = path.into();
        Self {
            process: Some(Process::start(path.clone(), vec![], None).unwrap()),
            file: String::from(path.to_str().unwrap()),
        }
    }

    pub fn proceed(&self) {
        if let Some(proc) = &self.process {
            proc.proceed()
        }
    }
}
extern crate libc;

#[macro_use]
extern crate cfg_if;

use std::path::PathBuf;
use crate::process::Process;
use std::error::Error;
use std::borrow::Borrow;

pub mod trace;
pub mod process;

pub struct Debugger {
    process: Process,
}

impl Debugger {
    pub fn start<P: Into<PathBuf>>(path: P) -> Self {
        Self {
            process: Process::start(path, vec![], None).unwrap()
        }
    }

    pub fn proceed(&self) {
        self.process.proceed()
    }
}
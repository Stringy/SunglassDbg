#[macro_use]
extern crate cfg_if;
extern crate libc;
extern crate log;
extern crate nix;

use std::cell::RefCell;

use log::info;

use common::config::Config;

use crate::breakpoint::Breakpoint;
use crate::error::{DebugError, Reason, Result};
use crate::process::Process;

pub mod trace;
pub mod process;
pub mod error;
pub mod breakpoint;

pub struct Debugger {
    process: RefCell<Option<Process>>,
    file: Option<String>,
    breakpoints: Vec<Breakpoint>,
}

impl Debugger {
    ///
    /// Create a new Debugger object from the provided config. It will
    /// set itself up read to debug the target, whether that's a
    /// running process, where we immediately try to attach,
    /// or a binary file on disk, in which case it will be launched
    /// manually later.
    ///
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
            breakpoints: Vec::new(),
        }
    }

    ///
    /// Run the debuggee process, using the file path from the config.
    /// the new process is immediately traced and is waiting to continue.
    ///
    pub fn run(&self, args: Vec<String>, env: Vec<String>) -> Result<()> {
        {
            let maybe_process = self.process.borrow();
            if maybe_process.is_some() {
                return Err(DebugError::InvalidOperation(Reason::ProcessExists));
            }
        }

        match &self.file {
            Some(file) => {
                let process = Process::start(file.clone(), args, Some(env))?;
                info!("started process: {}", process.pid);
                self.process.replace(Some(process));
                Ok(())
            }
            None => Err(DebugError::InvalidOperation(Reason::NoFile))
        }
    }

    ///
    /// Continue the traced process. Continues until a signal is received,
    /// or the process exits.
    ///
    pub fn proceed(&self) -> Result<()> {
        match &*self.process.borrow() {
            Some(process) => process.proceed(),
            None => Err(DebugError::InvalidOperation(Reason::NoProcess))
        }
    }

    ///
    /// Add a new breakpoint at the given address in the traced process.
    /// The address is expected to be absolute.
    ///
    /// TODO: abstract the breakpoint location so that it can be a source location
    ///       or a function, or an actual address. Might need our own CLI parser...
    ///
    pub fn add_breakpoint(&mut self, addr: u64) -> Result<()> {
        match &*self.process.borrow() {
            Some(process) => {
                let mut bp = Breakpoint::new(addr);
                bp.enable(process)?;
                self.breakpoints.push(bp);
                Ok(())
            }
            None => Err(DebugError::InvalidOperation(Reason::NoProcess))
        }
    }

    ///
    /// Removes a breakpoint, including disabling it in the traced process.
    ///
    pub fn remove_breakpoint(&mut self, idx: usize) -> Result<()> {
        match &*self.process.borrow() {
            Some(process) => {
                if let Some(bp) = self.breakpoints.get_mut(idx) {
                    if bp.enabled {
                        bp.disable(process)?;
                    }
                } else {
                    return Err(DebugError::InvalidOperation(Reason::NoSuchBreakpoint));
                }

                self.breakpoints.remove(idx);
                info!("breakpoint removed.");
                Ok(())
            }
            None => Err(DebugError::InvalidOperation(Reason::NoProcess))
        }
    }

    ///
    /// Logs all the known breakpoints in the debugger.
    ///
    pub fn list_breakpoints(&mut self) {
        for (i, bp) in self.breakpoints.iter().enumerate() {
            info!("{}: addr: {:x} {}", i + 1, bp.addr, if bp.enabled { "(enabled)" } else { "" });
        }
    }

    ///
    /// Enables the given breakpoint in the traced process.
    ///
    pub fn enable_breakpoint(&mut self, idx: usize) -> Result<()> {
        match &*self.process.borrow() {
            Some(process) => {
                if let Some(bp) = self.breakpoints.get_mut(idx) {
                    if !bp.enabled {
                        bp.enable(process)?;
                        info!("breakpoint at 0x{:x} enabled.", bp.addr);
                    }
                    Ok(())
                } else {
                    Err(DebugError::InvalidOperation(Reason::NoSuchBreakpoint))
                }
            }
            None => Err(DebugError::InvalidOperation(Reason::NoProcess))
        }
    }

    ///
    /// Disables the given breakpoint in the traced process.
    ///
    pub fn disable_breakpoint(&mut self, idx: usize) -> Result<()> {
        match &*self.process.borrow() {
            Some(process) => {
                if let Some(bp) = self.breakpoints.get_mut(idx) {
                    if bp.enabled {
                        bp.disable(process)?;
                        info!("breakpoint at 0x{:x} disabled.", bp.addr);
                    }
                    Ok(())
                } else {
                    Err(DebugError::InvalidOperation(Reason::NoSuchBreakpoint))
                }
            }
            None => Err(DebugError::InvalidOperation(Reason::NoProcess))
        }
    }
}
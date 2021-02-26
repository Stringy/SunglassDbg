use std::os::raw::c_long;

use nix::Error;
use nix::sys::ptrace::AddressType;
use nix::unistd::Pid;

use crate::trace::{ProcessTracer, Result, TraceError};

pub struct Tracer;

impl From<nix::Error> for TraceError {
    fn from(n: nix::Error) -> Self {
        match n {
            Error::Sys(n) => {
                TraceError::Sys(std::io::Error::from_raw_os_error(n as i32))
            }
            _ => panic!("unexpected tracing error: {}", n)
        }
    }
}

impl ProcessTracer for Tracer {
    fn read_text(pid: i32, addr: u64) -> Result<c_long> {
        nix::sys::ptrace::read(Pid::from_raw(pid), addr as AddressType).map_err(|e| e.into())
    }

    fn write_text(pid: i32, addr: u64, data: u64) -> Result<()> {
        unsafe {
            nix::sys::ptrace::write(Pid::from_raw(pid), addr as AddressType, data as *mut libc::c_void).map_err(|e| e.into())
        }
    }

    fn read_data(pid: i32, addr: u64) -> Result<c_long> {
        Tracer::read_text(pid, addr)
    }

    fn write_data(pid: i32, addr: u64, data: u64) -> Result<()> {
        Tracer::write_text(pid, addr, data)
    }

    fn proceed(pid: i32) -> Result<()> {
        nix::sys::ptrace::cont(Pid::from_raw(pid), None).map_err(|e| e.into())
    }

    fn trace_me() -> Result<()> {
        nix::sys::ptrace::traceme().map_err(|e| e.into())
    }

    fn step(pid: i32) -> Result<()> {
        nix::sys::ptrace::step(Pid::from_raw(pid), None).map_err(|e| e.into())
    }

    fn attach(pid: i32) -> Result<()> {
        nix::sys::ptrace::attach(Pid::from_raw(pid)).map_err(|e| e.into())
    }

    fn detach(pid: i32) -> Result<()> {
        nix::sys::ptrace::detach(Pid::from_raw(pid), None).map_err(|e| e.into())
    }
}
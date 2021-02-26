#[cfg_attr(any(target_os = "linux", target_os = "macos"), path = "nix.rs")]
mod tracer;

use tracer::Tracer;
use std::os::raw::c_long;
use std::fmt::{Display, Formatter};
use std::fmt;

pub type Result<T> = std::result::Result<T, TraceError>;

#[derive(Debug)]
pub enum TraceError {
    Sys(std::io::Error)
}

impl Display for TraceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TraceError::Sys(n) => write!(f, "system error: {}", n)
        }
    }
}

///
/// A `ProcessTracer` provides functionality for tracing and inspecting a
/// running process.
///
trait ProcessTracer {
    ///
    /// Read a word from the process text at the specified address
    ///
    fn read_text(pid: i32, addr: u64) -> Result<c_long>;

    ///
    /// Write a word to the process text at the specified address
    ///
    fn write_text(pid: i32, addr: u64, data: u64) -> Result<()>;

    ///
    /// Read a word from the process data at the specified address
    ///
    fn read_data(pid: i32, addr: u64) -> Result<c_long>;

    ///
    /// Write a word to the process data at the specified address
    ///
    fn write_data(pid: i32, addr: u64, data: u64) -> Result<()>;

    ///
    /// Continue execution of the process, until the next signal
    ///
    fn proceed(pid: i32) -> Result<()>;

    ///
    /// Indicate that this process is waiting to be traced.
    /// Typically used before exec.
    ///
    fn trace_me() -> Result<()>;

    ///
    /// Step a single instruction.
    ///
    /// TODO: add signal delivery
    ///
    fn step(pid: i32) -> Result<()>;

    ///
    /// Attach to a given PID
    ///
    fn attach(pid: i32) -> Result<()>;

    ///
    /// Detach from a given PID
    ///
    fn detach(pid: i32) -> Result<()>;
}

///
/// Read a word from the process text at the specified address
///
pub fn read_text(pid: i32, addr: u64) -> Result<c_long> {
    Tracer::read_text(pid, addr)
}

///
/// Write a word to the process text at the specified address
///
pub fn write_text(pid: i32, addr: u64, data: u64) -> Result<()> {
    Tracer::write_text(pid, addr, data)
}

///
/// Read a word from the process data at the specified address
///
pub fn read_data(pid: i32, addr: u64) -> Result<c_long> {
    Tracer::read_data(pid, addr)
}

///
/// Write a word to the process data at the specified address
///
pub fn write_data(pid: i32, addr: u64, data: u64) -> Result<()> {
    Tracer::write_data(pid, addr, data)
}

///
/// Continue execution of the process, until the next signal
///
pub fn proceed(pid: i32) -> Result<()> {
    Tracer::proceed(pid)
}

///
/// Indicate that this process is waiting to be traced.
/// Typically used before exec.
///
pub fn trace_me() -> Result<()> {
    Tracer::trace_me()
}

///
/// Step a single instruction.
///
pub fn step(pid: i32) -> Result<()> {
    Tracer::step(pid)
}

///
/// Attach to a given PID
///
pub fn attach(pid: i32) -> Result<()> {
    Tracer::attach(pid)
}

///
/// Detach from a given PID
///
pub fn detach(pid: i32) -> Result<()> {
    Tracer::detach(pid)
}

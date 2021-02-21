#[cfg_attr(any(target_os = "linux"), path = "ptrace.rs")]
#[cfg_attr(target_os = "macos", path = "macos.rs")]
mod tracer;

use tracer::Tracer;
use std::os::raw::c_int;

///
/// A `ProcessTracer` provides functionality for tracing and inspecting a
/// running process.
///
trait ProcessTracer {
    ///
    /// Read a word from the process text at the specified address
    ///
    fn read_text(pid: i32, addr: u64) -> c_int;

    ///
    /// Write a word to the process text at the specified address
    ///
    fn write_text(pid: i32, addr: u64, data: u64) -> c_int;

    ///
    /// Read a word from the process data at the specified address
    ///
    fn read_data(pid: i32, addr: u64) -> c_int;

    ///
    /// Write a word to the process data at the specified address
    ///
    fn write_data(pid: i32, addr: u64, data: u64) -> c_int;

    ///
    /// Continue execution of the process, until the next signal
    ///
    fn proceed(pid: i32) -> c_int;

    ///
    /// Indicate that this process is waiting to be traced.
    /// Typically used before exec.
    ///
    fn trace_me() -> c_int;

    ///
    /// Step a single instruction.
    ///
    fn step(pid: i32) -> c_int;

    ///
    /// Attach to a given PID
    ///
    fn attach(pid: i32) -> c_int;

    ///
    /// Detach from a given PID
    ///
    fn detach(pid: i32) -> c_int;
}

///
/// Read a word from the process text at the specified address
///
pub fn read_text(pid: i32, addr: u64) -> c_int {
    Tracer::read_text(pid, addr)
}

///
/// Write a word to the process text at the specified address
///
pub fn write_text(pid: i32, addr: u64, data: u64) -> c_int {
    Tracer::write_text(pid, addr, data)
}

///
/// Read a word from the process data at the specified address
///
pub fn read_data(pid: i32, addr: u64) -> c_int {
    Tracer::read_data(pid, addr)
}

///
/// Write a word to the process data at the specified address
///
pub fn write_data(pid: i32, addr: u64, data: u64) -> c_int {
    Tracer::write_data(pid, addr, data)
}

///
/// Continue execution of the process, until the next signal
///
pub fn proceed(pid: i32) -> c_int {
    Tracer::proceed(pid)
}

///
/// Indicate that this process is waiting to be traced.
/// Typically used before exec.
///
pub fn trace_me() -> c_int {
    Tracer::trace_me()
}

///
/// Step a single instruction.
///
pub fn step(pid: i32) -> c_int {
    Tracer::step(pid)
}

///
/// Attach to a given PID
///
pub fn attach(pid: i32) -> c_int {
    Tracer::attach(pid)
}

///
/// Detach from a given PID
///
pub fn detach(pid: i32) -> c_int {
    Tracer::detach(pid)
}
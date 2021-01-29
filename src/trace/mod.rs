#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "linux")]
use linux::PTrace as Tracer;

///
/// A `ProcessTracer` provides functionality for tracing and inspecting a
/// running process.
///
trait ProcessTracer {
    ///
    /// Read 8 bytes from the process memory at the specified address
    ///
    fn peek(pid: i32, addr: u64) -> i64;

    ///
    /// Write 8 bytes to the process memory at the specified address
    ///
    fn poke(pid: i32, addr: u64, data: u64) -> i64;

    ///
    /// Continue execution of the process, until the next signal
    ///
    fn proceed(pid: i32) -> i64;
}

///
/// Read 8 bytes from the process memory at the specified address.
///
/// Uses a platform-specific tracer
///
pub fn peek(pid: i32, addr: u64) -> i64 {
    Tracer::peek(pid, addr)
}

///
/// Write 8 bytes to the process memory at the specified address.
///
/// Uses a platform-specific tracer
///
pub fn poke(pid: i32, addr: u64, data: u64) -> i64 {
    Tracer::poke(pid, addr, data)
}

///
/// Continue execution of the process, until the next signal
///
/// Uses a platform-specific tracer
///
pub fn proceed(pid: i32) -> i64 {
    Tracer::proceed(pid)
}

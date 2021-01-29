use crate::trace::ProcessTracer;

pub struct PTrace;

impl ProcessTracer for PTrace {
    fn peek(pid: i32, addr: u64) -> i64 {
        unsafe {
            libc::ptrace(libc::PTRACE_PEEKDATA, pid, addr, 0)
        }
    }

    fn poke(pid: i32, addr: u64, data: u64) -> i64 {
        unsafe {
            libc::ptrace(libc::PTRACE_POKEDATA, pid, addr, data)
        }
    }

    fn proceed(pid: i32) -> i64 {
        unsafe {
            libc::ptrace(libc::PTRACE_CONT, pid, 0, 0)
        }
    }
}
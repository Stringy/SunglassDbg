use crate::trace::ProcessTracer;
use std::os::raw::{c_char, c_long, c_void};
use core::ptr;

pub struct Tracer;

impl ProcessTracer for Tracer {
    fn read_text(pid: i32, addr: u64) -> c_long {
        unsafe {
            libc::ptrace(libc::PTRACE_PEEKTEXT, pid, addr as *mut c_char, 0)
        }
    }

    fn write_text(pid: i32, addr: u64, data: u64) -> c_long {
        unsafe {
            libc::ptrace(libc::PTRACE_POKETEXT, pid, addr as *mut c_char, data as c_long)
        }
    }

    fn read_data(pid: i32, addr: u64) -> c_long {
        unsafe {
            libc::ptrace(libc::PTRACE_PEEKDATA, pid, addr as *mut c_char, 0)
        }
    }

    fn write_data(pid: i32, addr: u64, data: u64) -> c_long {
        unsafe {
            libc::ptrace(libc::PTRACE_POKEDATA, pid, addr as *mut c_char, data as c_long)
        }
    }

    fn proceed(pid: i32) -> c_long {
        unsafe {
            libc::ptrace(libc::PTRACE_CONT, pid, ptr::null_mut::<*mut c_void>(), 0)
        }
    }

    fn trace_me() -> c_long {
        unsafe {
            libc::ptrace(libc::PTRACE_TRACEME, 0, ptr::null_mut::<*mut c_void>(), 0)
        }
    }

    fn step(pid: i32) -> c_long {
        unsafe {
            libc::ptrace(libc::PTRACE_SINGLESTEP, pid, ptr::null_mut::<*mut c_void>(), 0)
        }
    }

    fn attach(pid: i32) -> c_long {
        unsafe {
            libc::ptrace(libc::PTRACE_ATTACH, pid, ptr::null_mut::<*mut c_void>(), 0)
        }
    }

    fn detach(pid: i32) -> c_long {
        unsafe {
            libc::ptrace(libc::PTRACE_DETACH, pid, ptr::null_mut::<*mut c_void>(), 0)
        }
    }
}
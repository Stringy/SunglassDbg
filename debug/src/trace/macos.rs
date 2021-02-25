use crate::trace::ProcessTracer;
use std::ptr;
use std::os::raw::{c_long, c_char, c_int};

pub struct Tracer;

impl ProcessTracer for Tracer {
    fn read_text(pid: i32, addr: u64) -> c_long {
        unsafe {
            libc::ptrace(libc::PT_READ_I, pid, addr as *mut c_char, 0) as c_long
        }
    }

    fn write_text(pid: i32, addr: u64, data: u64) -> c_long {
        unsafe {
            libc::ptrace(libc::PT_WRITE_I, pid, addr as *mut c_char, data as c_int) as c_long
        }
    }

    fn read_data(pid: i32, addr: u64) -> c_long {
        unsafe {
            libc::ptrace(libc::PT_READ_D, pid, addr as *mut c_char, 0) as c_long
        }
    }

    fn write_data(pid: i32, addr: u64, data: u64) -> c_long {
        unsafe {
            libc::ptrace(libc::PT_WRITE_D, pid, addr as *mut c_char, data as c_int) as c_long
        }
    }

    fn proceed(pid: i32) -> c_long {
        unsafe {
            libc::ptrace(libc::PT_CONTINUE, pid, ptr::null_mut(), 0) as c_long
        }
    }

    fn trace_me() -> c_long {
        unsafe {
            libc::ptrace(libc::PT_TRACE_ME, 0, ptr::null_mut(), 0) as c_long
        }
    }

    fn step(pid: i32) -> c_long {
        unsafe {
            libc::ptrace(libc::PT_STEP, pid, ptr::null_mut(), 0) as c_long
        }
    }

    fn attach(pid: i32) -> c_long {
        unsafe {
            libc::ptrace(libc::PT_ATTACHEXC, pid, ptr::null_mut(), 0) as c_long
        }
    }

    fn detach(pid: i32) -> c_long {
        unsafe {
            libc::ptrace(libc::PT_DETACH, pid, ptr::null_mut(), 0) as c_long
        }
    }
}
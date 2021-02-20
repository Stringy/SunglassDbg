use crate::trace::ProcessTracer;
use std::ptr;
use std::os::raw::{c_int, c_char};

pub struct Tracer;

impl ProcessTracer for Tracer {
    fn read_text(pid: i32, addr: u64) -> c_int {
        unsafe {
            libc::ptrace(libc::PT_READ_I, pid, addr as *mut c_char, 0)
        }
    }

    fn write_text(pid: i32, addr: u64, data: u64) -> c_int {
        unsafe {
            libc::ptrace(libc::PT_WRITE_I, pid, addr as *mut c_char, data as c_int)
        }
    }

    fn read_data(pid: i32, addr: u64) -> c_int {
        unsafe {
            libc::ptrace(libc::PT_READ_D, pid, addr as *mut c_char, 0)
        }
    }

    fn write_data(pid: i32, addr: u64, data: u64) -> c_int {
        unsafe {
            libc::ptrace(libc::PT_WRITE_D, pid, addr as *mut c_char, data as c_int)
        }
    }

    fn proceed(pid: i32) -> c_int {
        unsafe {
            libc::ptrace(libc::PT_CONTINUE, pid, ptr::null_mut(), 0)
        }
    }

    fn trace_me() -> c_int {
        unsafe {
            libc::ptrace(libc::PT_TRACE_ME, 0, ptr::null_mut(), 0)
        }
    }

    fn step(pid: i32) -> c_int {
        unsafe {
            libc::ptrace(libc::PT_STEP, pid, ptr::null_mut(), 0)
        }
    }

    fn attach(pid: i32) -> c_int {
        unsafe {
            libc::ptrace(libc::PT_ATTACHEXC, pid, ptr::null_mut(), 0)
        }
    }

    fn detach(pid: i32) -> c_int {
        unsafe {
            libc::ptrace(libc::PT_DETACH, pid, ptr::null_mut(), 0)
        }
    }
}
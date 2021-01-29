use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::path::{Path, PathBuf};
use std::ptr;

use crate::trace;

pub type Result<T> = std::result::Result<T, ()>;

pub struct Process {
    pub pid: i32
}

impl Process {
    pub fn new(pid: i32) -> Self {
        Self {
            pid
        }
    }

    pub fn peek(&self, addr: u64) -> i64 {
        trace::peek(self.pid, addr)
    }

    pub fn poke(&self, addr: u64, data: u64) -> i64 {
        trace::poke(self.pid, addr, data)
    }

    pub fn proceed(&self) -> i64 {
        trace::proceed(self.pid)
    }
}

///
///
///
pub fn start<P: Into<PathBuf>>(path: P, args: Vec<String>, env: Option<Vec<String>>) -> Result<Process> {
    if cfg!(target_os="linux") {
        do_start(path.into(), args, env, || {
            unsafe {
                libc::ptrace(libc::PTRACE_TRACEME, 0, 0, 0)
            }
        })
    } else {
        unimplemented!();
    }
}

fn do_start<F>(path: PathBuf, args: Vec<String>, env: Option<Vec<String>>, pre_exec: F) -> Result<Process>
    where F: Fn() -> i64 {
    let pid = unsafe { libc::fork() };
    match pid {
        x if x < 0 => Err(()),
        x if x > 0 => {
            // parent
            Ok(Process::new(x))
        }
        x if x == 0 => {
            let path = path.into_os_string().into_string().unwrap();

            let mut args = args.clone();
            args.insert(0, path.clone());

            let path = string_to_ptr(path);
            let args = string_vec_to_ptr(args);

            let env = if env.is_some() {
                string_vec_to_ptr(env.unwrap())
            } else {
                ptr::null()
            };

            // child
            if pre_exec() < 0 {
                panic!("pre-exec routine failed!");
            }

            unsafe {
                libc::execve(path, args, env);
                let error = CString::from_raw(libc::strerror(*libc::__errno_location()));
                unreachable!("execve failed!: {}", error.into_string().unwrap());
            }
        }
        _ => unreachable!()
    }
}

fn string_to_ptr(s: String) -> *const c_char {
    let s = CString::new(s).unwrap();
    let p = s.as_ptr();
    std::mem::forget(s);
    p
}

fn string_vec_to_ptr(v: Vec<String>) -> *const *const c_char {
    let mut strings = Vec::new();
    for s in v {
        strings.push(string_to_ptr(s));
    }
    strings.push(ptr::null());

    let p = strings.as_ptr();
    std::mem::forget(strings);
    p
}
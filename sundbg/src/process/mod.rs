use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::path::{Path, PathBuf};
use std::ptr;

use crate::trace;

cfg_if! {
    if #[cfg(any(target_os = "linux"))] {
        use libc::__errno_location as errno_location;
    } else if #[cfg(any(target_os = "macos"))] {
        use libc::__error as errno_location;
    }
}

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

    pub fn start<P: Into<PathBuf>>(path: P, args: Vec<String>, env: Option<Vec<String>>) -> Result<Self> {
        if cfg!(any(target_os="linux", target_os="macos")) {
            Process::do_start(path.into(), args, env, || {
                trace::trace_me() as i64
            })
        } else {
            unimplemented!("Unknown platform!");
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
                    let error = CString::from_raw(libc::strerror(*errno_location()));
                    unreachable!("execve failed!: {}", error.into_string().unwrap());
                }
            }
            _ => unreachable!()
        }
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
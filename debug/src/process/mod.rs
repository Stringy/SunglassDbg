use std::ffi::CString;
use std::os::raw::c_char;
use std::path::PathBuf;
use std::ptr;

use crate::error::{DebugError, Result};
use crate::trace;
use nix::sys::wait::{waitpid, WaitStatus};
use nix::unistd::Pid;

use log::info;


cfg_if! {
    if #[cfg(any(target_os = "linux"))] {
        use libc::__errno_location as errno_location;
    } else if #[cfg(any(target_os = "macos"))] {
        use libc::__error as errno_location;
    }
}

#[derive(Default)]
pub struct Process {
    pub pid: i32
}

impl Process {
    pub fn new(pid: i32) -> Self {
        Self {
            pid
        }
    }

    pub fn attach(_pid: i32) -> Self {
        unimplemented!();
    }

    pub fn start<P: Into<PathBuf>>(path: P, args: Vec<String>, env: Option<Vec<String>>) -> Result<Self> {
        if cfg!(any(target_os="linux", target_os="macos")) {
            Process::do_start(path.into(), args, env, || {
                use trace::TraceError::*;
                if let Err(e) = trace::trace_me() {
                    match e {
                        Sys(n) => n.raw_os_error().unwrap_or(-1) as i64,
                    }
                } else {
                    0i64
                }
            })
        } else {
            unimplemented!("Unknown platform!");
        }
    }

    ///
    /// Continue executing until the next event, where that can be a breakpoint
    /// is hit (SIGTRAP), the process has exited, or other possible wait statuses.
    ///
    pub fn proceed(&self) -> Result<()> {
        trace::proceed(self.pid).map_err(DebugError::TraceFailure)?;
        match waitpid(Pid::from_raw(self.pid), None) {
            Ok(WaitStatus::Stopped(pid, sig)) => info!("process {} stopped with signal {}", pid, sig),
            Ok(WaitStatus::Exited(pid, status)) => info!("process {} exited (status: {})", pid, status),
            Ok(WaitStatus::Signaled(pid, sig, _b)) => info!("process {} received signal {}", pid, sig),
            Ok(WaitStatus::PtraceEvent(_, _, _)) => {
                todo!("ptrace event")
            }
            Ok(WaitStatus::PtraceSyscall(_)) => {
                todo!("ptrace syscall")
            }
            Ok(WaitStatus::Continued(_)) => {}
            Ok(WaitStatus::StillAlive) => {}
            Err(e) => return Err(e.into()),
        };
        Ok(())
    }

    ///
    /// Reads a single word from the given address in the process memory.
    ///
    pub fn read(&self, addr: u64) -> Result<i64> {
        trace::read_text(self.pid, addr).map_err(DebugError::TraceFailure)
    }

    ///
    /// Writes a single word to the given address in the process memory.
    ///
    pub fn write(&self, addr: u64, data: u64) -> Result<()> {
        trace::write_text(self.pid, addr, data).map_err(DebugError::TraceFailure)
    }

    fn do_start<F>(path: PathBuf, args: Vec<String>, env: Option<Vec<String>>, pre_exec: F) -> Result<Process>
        where F: Fn() -> i64 {
        let pid = unsafe { libc::fork() };
        match pid {
            x if x < 0 => Err(DebugError::Sys(std::io::Error::last_os_error())),
            x if x > 0 => {
                // parent
                Ok(Process::new(x))
            }
            x if x == 0 => {
                let path = path.into_os_string().into_string().unwrap();

                let mut args = args;
                args.insert(0, path.clone());

                let path = string_to_ptr(path);
                let args = string_vec_to_ptr(args);

                let env = if let Some(env) = env {
                    string_vec_to_ptr(env)
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

///
/// Helper method for converting a `String` into a raw c-style string
/// pointer.
///
fn string_to_ptr(s: String) -> *const c_char {
    let s = CString::new(s).unwrap();
    let p = s.as_ptr();
    std::mem::forget(s);
    p
}

///
/// Helper method for converting an entire list of `String`s into raw
/// c-style string array.
///
fn string_vec_to_ptr(v: Vec<String>) -> *const *const c_char {
    let mut strings = Vec::new();
    for s in v {
        strings.push(string_to_ptr(s));
    }
    strings.push(ptr::null());

    let p = strings.as_ptr();
    // important to forget them, because otherwise they are invalid by
    // the time they are used.
    // This is fine, however, since we're doing this just before
    // exec'ing in the new process.
    std::mem::forget(strings);
    p
}
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fmt;
use nix::Error::Sys;
use crate::trace::TraceError;

pub type Result<T> = std::result::Result<T, DebugError>;

#[derive(Debug)]
pub enum DebugError {
    InvalidOperation(Reason),
    TraceFailure(TraceError),
    Sys(std::io::Error),
    Unknown(Box::<dyn Error>),
}

#[derive(Debug)]
pub enum Reason {
    NoFile,
    NoProcess,
    ProcessExists,
    NoSuchBreakpoint,
}

impl Display for Reason {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use Reason::*;
        match self {
            NoFile => write!(f, "no file"),
            NoProcess => write!(f, "no process"),
            ProcessExists => write!(f, "process already exists"),
            NoSuchBreakpoint => write!(f, "no such breakpoint")
        }
    }
}

impl Display for DebugError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use DebugError::*;
        match self {
            InvalidOperation(r) => write!(f, "Invalid operation: {}", r),
            Sys(inner) => write!(f, "Encountered IO error: {}", inner),
            TraceFailure(t) => write!(f, "tracing failed: {}", t),
            Unknown(n) => write!(f, "unknown error: {}", n)
        }
    }
}

impl Error for DebugError {}

impl From<nix::Error> for DebugError {
    fn from(e: nix::Error) -> Self {
        match e {
            Sys(n) => DebugError::Sys(std::io::Error::from_raw_os_error(n as i32)),
            unkn => DebugError::Unknown(Box::new(unkn))
        }
    }
}
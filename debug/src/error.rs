use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fmt;

pub type Result<T> = std::result::Result<T, DebugError>;

#[derive(Debug)]
pub enum DebugError {
    CannotStart(Reason),
    IoError(std::io::Error),
}

#[derive(Debug)]
pub enum Reason {
    NoFile,
    NoProcess,
    ProcessExists,
}

impl Display for Reason {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use Reason::*;
        match self {
            NoFile => write!(f, "no file"),
            NoProcess => write!(f, "no process"),
            ProcessExists => write!(f, "process already exists")
        }
    }
}

impl Display for DebugError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use DebugError::*;
        match self {
            CannotStart(r) => write!(f, "Unable to start: {}", r),
            IoError(inner) => write!(f, "Encountered IO error: {}", inner),
        }
    }
}

impl Error for DebugError {}
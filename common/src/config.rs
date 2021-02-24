use crate::cli::CommandLine;

pub struct Config {
    pub should_attach: bool,
    pub await_start: bool,
    pub pid: Option<i32>,
    pub file: Option<String>,
}

impl From<CommandLine> for Config {
    fn from(c: CommandLine) -> Self {
        Self {
            should_attach: c.pid.is_some(),
            await_start: c.file.is_some() && c.pid.is_none(),
            file: c.file,
            pid: c.pid,
        }
    }
}
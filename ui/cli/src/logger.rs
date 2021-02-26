use log::{Record, Metadata, SetLoggerError, LevelFilter};
use std::io::Write;

static LOGGER: StdoutLogger = StdoutLogger;

pub fn init_logger() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Info))
}

struct StdoutLogger;

impl log::Log for StdoutLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {
        std::io::stdout().flush().unwrap()
    }
}


use log::{Record, Level, Metadata, SetLoggerError};

static LOGGER: StdoutLogger = StdoutLogger;

pub fn init_logger() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER)
}

struct StdoutLogger;

impl log::Log for StdoutLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        return true;
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {
        unimplemented!()
    }
}


use log::{Log, Level, Metadata, Record, SetLoggerError, LevelFilter};
use crate::event_bus::Bus;

pub struct UILogger;

impl Log for UILogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let bus = Bus::instance();
            let msg = format!("{}", record.args());
            bus.post_message(&msg);
        }
    }

    fn flush(&self) {}
}

static LOGGER: UILogger = UILogger;

pub fn init_logging() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Info))
}
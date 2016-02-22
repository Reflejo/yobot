extern crate log;

use log::{LogMetadata, LogLevel, LogRecord};

pub struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= LogLevel::Info
    }

    fn log(&self, record: &LogRecord) {
        println!("{} - {}", record.level(), record.args());
    }
}

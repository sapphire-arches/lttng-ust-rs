//! Publishes Rust stdlog logging to a lttng-ust channel.
//! To get started, all you need to do is add this crate as a dependency and
//! register it as the current logging facility using [`init()`](::init).
//!
extern crate log;
#[macro_use]
extern crate lttng_ust;

use log::{Record, Metadata, SetLoggerError};

import_tracepoints!(concat!(env!("OUT_DIR"), "/logging_tracepoints.rs"), tracepoints);

struct LTTNGLogger;

static LOGGER: LTTNGLogger = LTTNGLogger;

impl log::Log for LTTNGLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        // TODO: implementing env_logger esq filtering
        true
    }

    fn log(&self, record: &Record) {
        use tracepoints::rust_logging::*;
        use log::Level;

        if !self.enabled(record.metadata()) {
            return;
        }

        let file = record.file().unwrap_or("<unknown>");
        let line = record.line().unwrap_or(0);
        let module_path = record.module_path().unwrap_or("<unknown>");
        let target = record.target();
        let msg = format!("{}", record.args());

        match record.level() {
            Level::Error => error(file, line, module_path, target, &msg),
            Level::Warn => warn(file, line, module_path, target, &msg),
            Level::Info => info(file, line, module_path, target, &msg),
            Level::Debug => debug(file, line, module_path, target, &msg),
            Level::Trace => trace(file, line, module_path, target, &msg),
        }
    }

    fn flush(&self) {}
}

/// Try to set lttng-ust as the logging facility, reporting an error if the
/// operation fails.
pub fn try_init() -> Result<(), SetLoggerError> {
    use log::LevelFilter;
    log::set_max_level(LevelFilter::Trace);
    log::set_logger(&LOGGER)
}

/// Initialize a default logger, panicking if the operation fails.
pub fn init() {
    try_init().unwrap();
}

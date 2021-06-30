#![allow(unused)]

use log::{Level, Log, Metadata, Record, SetLoggerError};

struct EmbLogger {
    level: Level,
}

impl Log for EmbLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let level = record.level();

            let target = if !record.target().is_empty() {
                record.target()
            }
            else {
                record.module_path().unwrap_or_default()
            };

            // if false
            //|| target.starts_with("delta::block")
            //|| target.starts_with("m3broxy::bpm")
            //|| target.starts_with("m3broxy::task_bpm")
            {
                println!("{:<5} [{}] {}", level, target, record.args());

                // let tick = crate::DWT::get_cycle_count();
                // println!("{} {:<5} [{}] {}", tick, level, target, record.args());
            }
        }
    }

    fn flush(&self) {}
}

#[no_mangle]
static mut EMB_LOGGER: Option<EmbLogger> = None;

pub fn init_with_level(level: Level) -> Result<(), SetLoggerError> {
    static mut EMB_LOGGER: Option<EmbLogger> = None;

    // Unsafely init static logger
    let logger = unsafe {
        EMB_LOGGER = Some(EmbLogger { level });
        EMB_LOGGER.as_ref().unwrap()
    };

    log::set_logger(logger)?;
    log::set_max_level(level.to_level_filter());
    Ok(())
}

pub fn init() -> Result<(), SetLoggerError> {
    init_with_level(Level::Trace)
}

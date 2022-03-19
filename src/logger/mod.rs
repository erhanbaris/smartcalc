/*
 * smartcalc v1.0.7
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

#[cfg(all(not(target_arch = "wasm32"), not(test)))]
use libc_print::*;

use log::*;

pub struct SimpleLogger;
pub static LOGGER: SimpleLogger = SimpleLogger;

impl Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Debug
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            #[cfg(all(not(target_arch = "wasm32"), not(test)))]
            libc_println!("[{}] {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

pub fn initialize_logger() {
    if log::set_logger(&LOGGER).is_ok() {
        if cfg!(debug_assertions) {
            log::set_max_level(log::LevelFilter::Debug)
        } else {
            log::set_max_level(log::LevelFilter::Info)
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
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
            #[cfg(not(target_arch = "wasm32"))]
            libc_println!("[{}] {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}
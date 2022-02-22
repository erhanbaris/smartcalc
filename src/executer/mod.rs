/*
 * smartcalc v1.0.1
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

use crate::logger::{LOGGER};

pub fn initialize() {
    if log::set_logger(&LOGGER).is_ok() {
        if cfg!(debug_assertions) {
            log::set_max_level(log::LevelFilter::Debug)
        } else {
            log::set_max_level(log::LevelFilter::Info)
        }
    }
}

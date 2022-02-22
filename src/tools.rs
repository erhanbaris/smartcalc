/*
 * smartcalc v1.0.1
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

pub fn do_divition(left: f64, right: f64) -> f64 {
    let mut calculation = left / right;
    if calculation.is_infinite() || calculation.is_nan() {
        calculation = 0.0;
    }
    calculation
}

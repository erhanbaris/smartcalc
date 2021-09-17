use crate::{config::SmartCalcConfig, types::Money};

pub fn convert_currency(config: &SmartCalcConfig, left: &Money, right: &Money) -> f64 {
    let as_usd = match config.currency_rate.get(&left.get_currency()) {
        Some(l_rate) => left.get_price() / l_rate,
        _ => 0.0
    };

    match config.currency_rate.get(&right.get_currency()) {
        Some(r_rate) => as_usd * r_rate,
        _ => 0.0
    }
}

pub fn do_divition(left: f64, right: f64) -> f64 {
    let mut calculation = left / right;
    if calculation.is_infinite() || calculation.is_nan() {
        calculation = 0.0;
    }
    calculation
}

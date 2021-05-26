use crate::config::SmartCalcConfig;

pub fn convert_currency(config: &SmartCalcConfig, l_price: f64, l_currency: &str,  r_currency: &str) -> f64 {
    let as_usd = match config.currency_rate.get(l_currency) {
        Some(l_rate) => l_price / l_rate,
        _ => 0.0
    };

    match config.currency_rate.get(r_currency) {
        Some(r_rate) => as_usd * r_rate,
        _ => 0.0
    }
}
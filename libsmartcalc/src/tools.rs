use alloc::string::String;

pub fn convert_currency(l_price: f64, l_currency: &String,  r_currency: &String) -> f64 {
    let as_usd = match CURRENCY_RATES.read().unwrap().get(l_currency) {
        Some(l_rate) => l_price / l_rate,
        _ => 0.0
    };

    match CURRENCY_RATES.read().unwrap().get(r_currency) {
        Some(r_rate) => as_usd * r_rate,
        _ => 0.0
    }
}
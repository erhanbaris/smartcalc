pub fn do_divition(left: f64, right: f64) -> f64 {
    let mut calculation = left / right;
    if calculation.is_infinite() || calculation.is_nan() {
        calculation = 0.0;
    }
    calculation
}

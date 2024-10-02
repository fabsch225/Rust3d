pub fn fast_square_root(num: f64) -> f64 {
    if num < 0.0_f64 {
        return f64::NAN;
    }

    let mut root = 1.0_f64;

    while (root * root - num).abs() > 1e-5_f64 {
        root -= (root * root - num) / (2.0_f64 * root);
    }

    root
}
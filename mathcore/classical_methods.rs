use std::f64::consts::PI;
use std::collections::HashMap;

// Function to calculate the bearing capacity factors using Terzaghi's method
fn terzaghi(phi: f64) -> HashMap<String, f64> {
    let mut factors = HashMap::new();

    let exponent = (PI * (0.75 - (phi / 360.0)) * (phi * PI / 180.0).tan()).exp();
    let numerator_q = exponent.powi(2);
    let denominator_q = 2.0 * ((45.0 + (phi / 2.0)) * PI / 180.0).cos().powi(2);
    let nq = numerator_q / denominator_q;

    let nc = if phi > 0.0 {
        (nq - 1.0) * (1.0 / (phi * PI / 180.0).tan())
    } else {
        5.71
    };

    let numerator_g = 2.0 * (nq + 1.0) * (phi * PI / 180.0).tan();
    let denominator_g = 1.0 + (0.4 * (4.0 * phi * PI / 180.0).sin());
    let ngamma = numerator_g / denominator_g;

    factors.insert("Nc".to_string(), nc);
    factors.insert("Nq".to_string(), nq);
    factors.insert("Ngamma".to_string(), ngamma);

    factors
}

// Function to calculate the bearing capacity factors using Meyerhof's method
fn meyerhof(phi: f64) -> HashMap<String, f64> {
    let mut factors = HashMap::new();

    let nq = (PI * (phi * PI / 180.0).tan()).exp() * ((45.0 + (phi / 2.0)) * PI / 180.0).tan().powi(2);

    let nc = if phi > 0.0 {
        (nq - 1.0) * (1.0 / (phi * PI / 180.0).tan())
    } else {
        5.14
    };

    let ngamma = (nq - 1.0) * (phi * 1.4 * PI / 180.0).tan();

    factors.insert("Nc".to_string(), nc);
    factors.insert("Nq".to_string(), nq);
    factors.insert("Ngamma".to_string(), ngamma);

    factors
}

// Function to calculate the bearing capacity factors using Vesic's method
fn vesic(phi: f64) -> HashMap<String, f64> {
    let mut factors = HashMap::new();

    let nq = (PI * (phi * PI / 180.0).tan()).exp() * ((45.0 + (phi / 2.0)) * PI / 180.0).tan().powi(2);

    let nc = if phi > 0.0 {
        (nq - 1.0) * (1.0 / (phi * PI / 180.0).tan())
    } else {
        5.14
    };

    let ngamma = 2.0 * (nq + 1.0) * (phi * PI / 180.0).tan();

    factors.insert("Nc".to_string(), nc);
    factors.insert("Nq".to_string(), nq);
    factors.insert("Ngamma".to_string(), ngamma);

    factors
}

// Function to calculate the bearing capacity factors using Hansen's method
fn hansen(phi: f64) -> HashMap<String, f64> {
    let mut factors = HashMap::new();

    let nq = (PI * (phi * PI / 180.0).tan()).exp() * ((45.0 + (phi / 2.0)) * PI / 180.0).tan().powi(2);

    let nc = if phi > 0.0 {
        (nq - 1.0) * (1.0 / (phi * PI / 180.0).tan())
    } else {
        5.14
    };

    let ngamma = 1.5 * (nq - 1.0) * (phi * PI / 180.0).tan();

    factors.insert("Nc".to_string(), nc);
    factors.insert("Nq".to_string(), nq);
    factors.insert("Ngamma".to_string(), ngamma);

    factors
}

// Function to calculate the bearing capacity factors following Eurocode 7 (EC7)
fn ec7(phi: f64) -> HashMap<String, f64> {
    let mut factors = HashMap::new();

    let nq = (PI * (phi * PI / 180.0).tan()).exp() * ((45.0 + (phi / 2.0)) * PI / 180.0).tan().powi(2);

    let nc = if phi > 0.0 {
        (nq - 1.0) * (1.0 / (phi * PI / 180.0).tan())
    } else {
        5.14
    };

    let ngamma = 2.0 * (nq - 1.0) * (phi * PI / 180.0).tan();

    factors.insert("Nc".to_string(), nc);
    factors.insert("Nq".to_string(), nq);
    factors.insert("Ngamma".to_string(), ngamma);

    factors
}

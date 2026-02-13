//! WASM bindings for em-antennas.

use wasm_bindgen::prelude::*;
use em_antennas::{dipole, arrays, link_budget};

#[wasm_bindgen]
pub fn hertzian_dipole(length: f64, current: f64, frequency: f64, num_points: usize) -> JsValue {
    let d = dipole::HertzianDipole::new(length, current, frequency);
    let (thetas, pattern) = d.sample_pattern(num_points);
    let result = serde_json::json!({
        "thetas_deg": thetas.iter().map(|t| t.to_degrees()).collect::<Vec<_>>(),
        "pattern": pattern,
        "radiation_resistance": d.radiation_resistance(),
        "directivity": d.directivity(),
        "directivity_dbi": d.directivity_dbi(),
        "radiated_power": d.radiated_power(),
        "effective_area": d.effective_area(),
    });
    serde_wasm_bindgen::to_value(&result).unwrap()
}

#[wasm_bindgen]
pub fn half_wave_dipole(frequency: f64, current: f64, num_points: usize) -> JsValue {
    let d = dipole::HalfWaveDipole::new(frequency, current);
    let (thetas, pattern) = d.sample_pattern(num_points);
    let (r_in, x_in) = d.input_impedance();
    let result = serde_json::json!({
        "thetas_deg": thetas.iter().map(|t| t.to_degrees()).collect::<Vec<_>>(),
        "pattern": pattern,
        "length": d.length(),
        "radiation_resistance": d.radiation_resistance(),
        "input_impedance_re": r_in,
        "input_impedance_im": x_in,
        "directivity": d.directivity(),
        "directivity_dbi": d.directivity_dbi(),
        "radiated_power": d.radiated_power(),
        "effective_area": d.effective_area(),
    });
    serde_wasm_bindgen::to_value(&result).unwrap()
}

#[wasm_bindgen]
pub fn antenna_array(num_elements: usize, spacing: f64, beta_deg: f64, num_points: usize) -> JsValue {
    let beta = beta_deg.to_radians();
    let arr = arrays::UniformLinearArray::new(num_elements, spacing, beta);
    let (thetas, af) = arr.sample_pattern(num_points);
    let (_, total) = arr.sample_total_pattern(num_points);
    let result = serde_json::json!({
        "thetas_deg": thetas.iter().map(|t| t.to_degrees()).collect::<Vec<_>>(),
        "array_factor": af,
        "total_pattern": total,
        "beamwidth_deg": arr.first_null_beamwidth().to_degrees(),
        "directivity_approx": arr.directivity_approx(),
    });
    serde_wasm_bindgen::to_value(&result).unwrap()
}

#[wasm_bindgen]
pub fn friis_link(p_tx_w: f64, g_tx_db: f64, g_rx_db: f64, frequency: f64, distance: f64) -> JsValue {
    let g_tx = link_budget::from_db(g_tx_db);
    let g_rx = link_budget::from_db(g_rx_db);
    let link = link_budget::FriisLink::new(p_tx_w, g_tx, g_rx, frequency, distance);
    let result = serde_json::json!({
        "received_power_dbm": link.received_power_dbm(),
        "path_loss_db": link.path_loss_db(),
        "eirp_dbw": link.eirp_dbw(),
        "power_density_w_m2": link.power_density(),
    });
    serde_wasm_bindgen::to_value(&result).unwrap()
}

#[wasm_bindgen]
pub fn link_vs_distance(p_tx_w: f64, g_tx_db: f64, g_rx_db: f64, frequency: f64, r_min: f64, r_max: f64, num_points: usize) -> JsValue {
    let g_tx = link_budget::from_db(g_tx_db);
    let g_rx = link_budget::from_db(g_rx_db);
    let link = link_budget::FriisLink::new(p_tx_w, g_tx, g_rx, frequency, r_min);
    let (ds, ps) = link.sample_vs_distance(r_min, r_max, num_points);
    let result = serde_json::json!({ "distances": ds, "power_dbm": ps });
    serde_wasm_bindgen::to_value(&result).unwrap()
}

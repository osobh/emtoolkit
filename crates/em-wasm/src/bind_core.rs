//! WASM bindings for em-core.

use wasm_bindgen::prelude::*;
use num_complex::Complex64;
use em_core::{complex, constants, coordinates};

#[wasm_bindgen]
pub fn get_constants() -> JsValue {
    let map = serde_json::json!({
        "c0": constants::C_0,
        "mu0": constants::MU_0,
        "epsilon0": constants::EPSILON_0,
        "eta0": constants::ETA_0,
    });
    serde_wasm_bindgen::to_value(&map).unwrap()
}

#[wasm_bindgen]
pub fn reflection_coefficient(zl_re: f64, zl_im: f64, z0: f64) -> JsValue {
    let zl = Complex64::new(zl_re, zl_im);
    let z0c = Complex64::new(z0, 0.0);
    let gamma = complex::reflection_coefficient(zl, z0c);
    let result = serde_json::json!({
        "re": gamma.re,
        "im": gamma.im,
        "magnitude": gamma.norm(),
        "phase_deg": gamma.arg().to_degrees(),
        "vswr": complex::vswr(gamma),
    });
    serde_wasm_bindgen::to_value(&result).unwrap()
}

#[wasm_bindgen]
pub fn input_impedance_lossless(zl_re: f64, zl_im: f64, z0: f64, beta_l: f64) -> JsValue {
    let zl = Complex64::new(zl_re, zl_im);
    let zin = complex::input_impedance_lossless(z0, zl, beta_l);
    let result = serde_json::json!({ "re": zin.re, "im": zin.im });
    serde_wasm_bindgen::to_value(&result).unwrap()
}

#[wasm_bindgen]
pub fn power_to_db(linear: f64) -> f64 {
    em_core::units::power_to_db(linear)
}

#[wasm_bindgen]
pub fn db_to_power(db: f64) -> f64 {
    em_core::units::db_to_power(db)
}

#[wasm_bindgen]
pub fn wavelength(frequency_hz: f64) -> f64 {
    constants::wavelength(frequency_hz)
}

#[wasm_bindgen]
pub fn wavenumber(frequency_hz: f64) -> f64 {
    constants::wavenumber(frequency_hz)
}

#[wasm_bindgen]
pub fn cartesian_to_spherical(x: f64, y: f64, z: f64) -> JsValue {
    let c = coordinates::Cartesian::new(x, y, z);
    let s = c.to_spherical();
    let result = serde_json::json!({ "r": s.r, "theta": s.theta, "phi": s.phi });
    serde_wasm_bindgen::to_value(&result).unwrap()
}

#[wasm_bindgen]
pub fn spherical_to_cartesian(r: f64, theta: f64, phi: f64) -> JsValue {
    let s = coordinates::Spherical::new(r, theta, phi).expect("valid spherical coords");
    let c = s.to_cartesian();
    let result = serde_json::json!({ "x": c.x, "y": c.y, "z": c.z });
    serde_wasm_bindgen::to_value(&result).unwrap()
}

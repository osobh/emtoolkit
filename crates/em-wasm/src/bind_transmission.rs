//! WASM bindings for em-transmission.

use wasm_bindgen::prelude::*;
use num_complex::Complex64;
use em_transmission::{smith_chart, line_types, standing_waves, matching, stub_tuning};

#[wasm_bindgen]
pub fn smith_chart_point(zl_re: f64, zl_im: f64) -> JsValue {
    let z = Complex64::new(zl_re, zl_im);
    let sp = smith_chart::SmithPoint::from_impedance(z);
    let result = serde_json::json!({
        "gamma_re": sp.gamma.re,
        "gamma_im": sp.gamma.im,
        "gamma_mag": sp.gamma.norm(),
        "gamma_phase_deg": sp.gamma.arg().to_degrees(),
        "r": sp.r(),
        "x": sp.x(),
        "vswr": sp.vswr(),
        "return_loss_db": sp.return_loss_db(),
    });
    serde_wasm_bindgen::to_value(&result).unwrap()
}

#[wasm_bindgen]
pub fn smith_chart_swr_circle(gamma_mag: f64, num_points: usize) -> JsValue {
    let pts = smith_chart::swr_circle_points(gamma_mag, num_points);
    let xs: Vec<f64> = pts.iter().map(|p| p.0).collect();
    let ys: Vec<f64> = pts.iter().map(|p| p.1).collect();
    let result = serde_json::json!({ "x": xs, "y": ys });
    serde_wasm_bindgen::to_value(&result).unwrap()
}

#[wasm_bindgen]
pub fn smith_chart_trace(zl_re: f64, zl_im: f64, electrical_length: f64, num_points: usize) -> JsValue {
    let sp = smith_chart::SmithPoint::from_impedance(Complex64::new(zl_re, zl_im));
    let trace = smith_chart::trace_toward_generator(&sp, num_points, electrical_length);
    let xs: Vec<f64> = trace.iter().map(|p| p.gamma.re).collect();
    let ys: Vec<f64> = trace.iter().map(|p| p.gamma.im).collect();
    let result = serde_json::json!({ "gamma_re": xs, "gamma_im": ys });
    serde_wasm_bindgen::to_value(&result).unwrap()
}

#[wasm_bindgen]
pub fn coaxial_line_params(inner_radius: f64, outer_radius: f64, epsilon_r: f64, frequency: f64) -> JsValue {
    let coax = line_types::CoaxialLine::lossless(inner_radius, outer_radius, epsilon_r);
    let params = coax.parameters(frequency);
    let result = serde_json::json!({
        "r_per_m": params.r_per_m,
        "l_per_m": params.l_per_m,
        "g_per_m": params.g_per_m,
        "c_per_m": params.c_per_m,
        "z0_lossless": params.z0_lossless(),
    });
    serde_wasm_bindgen::to_value(&result).unwrap()
}

#[wasm_bindgen]
pub fn standing_wave_pattern(z0: f64, zl_re: f64, zl_im: f64, frequency: f64, length: f64, num_points: usize) -> JsValue {
    let zl = Complex64::new(zl_re, zl_im);
    let sw = standing_waves::StandingWaveParams::in_free_space(z0, zl, frequency, length);
    let (dv, v) = sw.sample_voltage(num_points);
    let (_, i) = sw.sample_current(num_points);
    let result = serde_json::json!({
        "positions": dv,
        "voltage": v,
        "current": i,
        "vswr": sw.vswr(),
    });
    serde_wasm_bindgen::to_value(&result).unwrap()
}

#[wasm_bindgen]
pub fn quarter_wave_match(z_load: f64, z_line: f64, frequency: f64) -> JsValue {
    use em_core::constants::C_0;
    let design = matching::quarter_wave_single(z_line, z_load, frequency, C_0, 2.0);
    let result = serde_json::json!({
        "z_transformer": design.z_transformer,
        "length": design.length,
        "bandwidth_fractional": design.bandwidth_fractional,
    });
    serde_wasm_bindgen::to_value(&result).unwrap()
}

#[wasm_bindgen]
pub fn single_stub_match(zl_re: f64, zl_im: f64, z0: f64, use_short: bool) -> JsValue {
    let zl = Complex64::new(zl_re, zl_im);
    let stub_type = if use_short {
        stub_tuning::StubType::Short
    } else {
        stub_tuning::StubType::Open
    };
    use em_core::constants::C_0;
    let solutions = stub_tuning::single_stub(z0, zl, 1e9, C_0, stub_type);
    let result: Vec<_> = solutions.iter().map(|s| {
        serde_json::json!({
            "stub_distance_wavelengths": s.stub_distance_wavelengths,
            "stub_length_wavelengths": s.stub_length_wavelengths,
        })
    }).collect();
    serde_wasm_bindgen::to_value(&result).unwrap()
}

//! WASM bindings for em-waves.

use wasm_bindgen::prelude::*;
use em_waves::{sinusoidal, traveling, phase};

#[wasm_bindgen]
pub fn sinusoidal_wave(amplitude: f64, frequency: f64, phase_rad: f64, damping: f64, t_end: f64, num_points: usize) -> JsValue {
    let w = sinusoidal::SinusoidalParams::damped(amplitude, frequency, phase_rad, damping);
    let (times, values) = w.sample(0.0, t_end, num_points);
    let result = serde_json::json!({ "times": times, "values": values });
    serde_wasm_bindgen::to_value(&result).unwrap()
}

#[wasm_bindgen]
pub fn traveling_wave_snapshot(amplitude: f64, frequency: f64, t: f64, x_max: f64, num_points: usize) -> JsValue {
    let wave = traveling::TravelingWaveParams::in_free_space(amplitude, frequency, 0.0, traveling::Direction::PositiveX);
    let dx = x_max / (num_points - 1) as f64;
    let positions: Vec<f64> = (0..num_points).map(|i| i as f64 * dx).collect();
    let values: Vec<f64> = positions.iter().map(|&x| wave.evaluate(x, t)).collect();
    let result = serde_json::json!({
        "positions": positions,
        "values": values,
    });
    serde_wasm_bindgen::to_value(&result).unwrap()
}

#[wasm_bindgen]
pub fn phase_comparison(a1: f64, f1: f64, p1: f64, a2: f64, f2: f64, p2: f64, t_end: f64, num_points: usize) -> JsValue {
    let w1 = phase::WaveformParams::new(a1, f1, p1);
    let w2 = phase::WaveformParams::new(a2, f2, p2);
    let comparison = phase::compare(&w1, &w2);
    let (t1, v1) = w1.sample(0.0, t_end, num_points);
    let (_, v2) = w2.sample(0.0, t_end, num_points);
    let result = serde_json::json!({
        "phase_difference_deg": comparison.phase_difference_deg,
        "relationship": format!("{:?}", comparison.relation),
        "times": t1,
        "wave1": v1,
        "wave2": v2,
    });
    serde_wasm_bindgen::to_value(&result).unwrap()
}

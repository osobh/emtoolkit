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

#[wasm_bindgen]
pub fn power_calculations(e0: f64, eta: f64, gamma_mag: f64) -> JsValue {
    use em_waves::power;
    let s_avg = power::poynting_average_magnitude(e0, eta);
    let p_delivered = power::power_delivered(s_avg, gamma_mag);
    let rl = if gamma_mag > 0.0 { power::return_loss_db(gamma_mag) } else { f64::INFINITY };
    let ml = power::mismatch_loss_db(gamma_mag);
    let result = serde_json::json!({
        "poynting_avg": s_avg,
        "power_delivered": p_delivered,
        "power_reflected": s_avg - p_delivered,
        "return_loss_db": rl,
        "mismatch_loss_db": ml,
        "gamma_mag": gamma_mag,
    });
    serde_wasm_bindgen::to_value(&result).unwrap()
}

#[wasm_bindgen]
pub fn unit_conversions(value: f64, from_unit: &str) -> JsValue {
    use em_waves::power;
    let result = match from_unit {
        "W" => serde_json::json!({
            "watts": value, "dbm": power::watts_to_dbm(value), "dbw": power::watts_to_dbw(value),
            "mw": value * 1e3, "uw": value * 1e6,
        }),
        "dBm" => {
            let w = power::dbm_to_watts(value);
            serde_json::json!({
                "watts": w, "dbm": value, "dbw": value - 30.0,
                "mw": w * 1e3, "uw": w * 1e6,
            })
        }
        "dBW" => {
            let w = power::dbw_to_watts(value);
            serde_json::json!({
                "watts": w, "dbm": value + 30.0, "dbw": value,
                "mw": w * 1e3, "uw": w * 1e6,
            })
        }
        _ => serde_json::json!({"error": "Unknown unit"}),
    };
    serde_wasm_bindgen::to_value(&result).unwrap()
}

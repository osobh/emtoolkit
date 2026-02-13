//! WASM bindings for em-timevarying.

use wasm_bindgen::prelude::*;
use em_timevarying::{faraday, displacement_current, charge_continuity};

#[wasm_bindgen]
pub fn sinusoidal_emf(b_peak: f64, area: f64, omega: f64, t_end: f64, num_points: usize) -> JsValue {
    let sf = faraday::SinusoidalFlux::new(b_peak, area, omega);
    let (ts, flux, emf) = sf.sample(t_end, num_points);
    let result = serde_json::json!({
        "times": ts, "flux": flux, "emf": emf,
        "emf_peak": sf.emf_peak(),
    });
    serde_wasm_bindgen::to_value(&result).unwrap()
}

#[wasm_bindgen]
pub fn ac_generator(turns: usize, b_field: f64, area: f64, rpm: f64) -> JsValue {
    let g = faraday::AcGenerator::from_rpm(turns, b_field, area, rpm);
    let result = serde_json::json!({
        "emf_peak": g.emf_peak(),
        "vrms": g.vrms(),
        "frequency": g.frequency(),
        "period": g.period(),
    });
    serde_wasm_bindgen::to_value(&result).unwrap()
}

#[wasm_bindgen]
pub fn transformer(n_primary: usize, n_secondary: usize, v_primary: f64, i_primary: f64) -> JsValue {
    let t = faraday::IdealTransformer::new(n_primary, n_secondary);
    let result = serde_json::json!({
        "turns_ratio": t.turns_ratio(),
        "v_secondary": t.v_secondary(v_primary),
        "i_secondary": t.i_secondary(i_primary),
        "is_step_up": t.is_step_up(),
    });
    serde_wasm_bindgen::to_value(&result).unwrap()
}

#[wasm_bindgen]
pub fn displacement_current_sim(area: f64, separation: f64, epsilon_r: f64, v_peak: f64, omega: f64, t_end: f64, num_points: usize) -> JsValue {
    let cap = displacement_current::ParallelPlateCapacitor::new(area, separation).with_dielectric(epsilon_r);
    let s = cap.sample(v_peak, omega, t_end, num_points);
    let result = serde_json::json!({
        "times": s.times,
        "voltage": s.voltage,
        "displacement_current": s.displacement_current,
        "conduction_current": s.conduction_current,
        "capacitance": cap.capacitance(),
        "id_peak": cap.displacement_current_peak(v_peak, omega),
    });
    serde_wasm_bindgen::to_value(&result).unwrap()
}

#[wasm_bindgen]
pub fn charge_relaxation(rho_0: f64, epsilon_r: f64, conductivity: f64, radius: f64, t_end: f64, num_points: usize) -> JsValue {
    let rc = charge_continuity::RelaxingCharge::from_material(rho_0, epsilon_r, conductivity, radius);
    let (ts, qs, is, dqs) = rc.sample(t_end, num_points);
    let result = serde_json::json!({
        "times": ts, "charge": qs, "current": is, "neg_dqdt": dqs,
        "tau": rc.time_constant(),
    });
    serde_wasm_bindgen::to_value(&result).unwrap()
}

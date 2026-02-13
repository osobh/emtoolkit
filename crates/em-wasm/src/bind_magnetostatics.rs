//! WASM bindings for em-magnetostatics.

use wasm_bindgen::prelude::*;
use em_magnetostatics::{biot_savart, current_loops, wire_forces, solenoid};

#[wasm_bindgen]
pub fn b_field_infinite_wire(current: f64, rho: f64) -> f64 {
    biot_savart::b_infinite_wire(current, rho)
}

#[wasm_bindgen]
pub fn b_field_wire_2d(current: f64, half_length: f64, num_segments: usize, x_min: f64, x_max: f64, y_min: f64, y_max: f64, nx: usize, ny: usize) -> JsValue {
    let segs = biot_savart::discretize_wire_z(current, half_length, num_segments);
    let (xs, ys, fields) = biot_savart::sample_b_field_2d(&segs, (x_min, x_max), (y_min, y_max), 0.0, nx, ny);
    let bx: Vec<f64> = fields.iter().map(|f| f.x).collect();
    let by: Vec<f64> = fields.iter().map(|f| f.y).collect();
    let mag: Vec<f64> = fields.iter().map(|f| f.magnitude()).collect();
    let result = serde_json::json!({ "x": xs, "y": ys, "bx": bx, "by": by, "magnitude": mag });
    serde_wasm_bindgen::to_value(&result).unwrap()
}

#[wasm_bindgen]
pub fn current_loop_on_axis(radius: f64, current: f64, z_min: f64, z_max: f64, num_points: usize) -> JsValue {
    let cl = current_loops::CurrentLoop::new(radius, current);
    let dz = (z_max - z_min) / (num_points - 1) as f64;
    let zs: Vec<f64> = (0..num_points).map(|i| z_min + i as f64 * dz).collect();
    let bz: Vec<f64> = zs.iter().map(|&z| cl.b_on_axis(z)).collect();
    let result = serde_json::json!({
        "z": zs, "bz": bz,
        "magnetic_moment": cl.magnetic_moment(),
    });
    serde_wasm_bindgen::to_value(&result).unwrap()
}

#[wasm_bindgen]
pub fn helmholtz_coil(radius: f64, current: f64, turns: usize, z_min: f64, z_max: f64, num_points: usize) -> JsValue {
    let hc = current_loops::HelmholtzCoil::new(radius, current, turns);
    let (zs, bs) = hc.sample_on_axis((z_min, z_max), num_points);
    let result = serde_json::json!({
        "z": zs, "b": bs,
        "b_center": hc.b_at_center(),
        "separation": hc.separation(),
    });
    serde_wasm_bindgen::to_value(&result).unwrap()
}

#[wasm_bindgen]
pub fn parallel_wire_force(i1: f64, i2: f64, separation: f64, length: f64) -> JsValue {
    let pw = wire_forces::ParallelWireForce::new(i1, i2, separation);
    let result = serde_json::json!({
        "force_per_length": pw.force_per_length(),
        "total_force": pw.total_force(length),
        "is_attractive": pw.is_attractive(),
    });
    serde_wasm_bindgen::to_value(&result).unwrap()
}

#[wasm_bindgen]
pub fn solenoid_params(turns: usize, length: f64, current: f64, radius: f64, mu_r: f64) -> JsValue {
    let s = solenoid::Solenoid::new(turns, length, current, radius).with_core(mu_r);
    let result = serde_json::json!({
        "b_interior": s.b_interior(),
        "inductance": s.inductance(),
        "stored_energy": s.stored_energy(),
        "energy_density": s.energy_density(),
    });
    serde_wasm_bindgen::to_value(&result).unwrap()
}

#[wasm_bindgen]
pub fn coaxial_cable_b(inner_r: f64, outer_inner_r: f64, outer_outer_r: f64, current: f64, r_max: f64, num_points: usize) -> JsValue {
    let c = solenoid::CoaxialCable::new(inner_r, outer_inner_r, outer_outer_r, current);
    let (rs, bs) = c.sample_b_vs_r(r_max, num_points);
    let result = serde_json::json!({
        "r": rs, "b": bs,
        "inductance_per_m": c.inductance_per_length(),
    });
    serde_wasm_bindgen::to_value(&result).unwrap()
}

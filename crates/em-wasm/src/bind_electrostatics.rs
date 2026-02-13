//! WASM bindings for em-electrostatics.

use wasm_bindgen::prelude::*;
use em_core::constants::EPSILON_0;
use em_electrostatics::{point_charges, method_of_images};

#[wasm_bindgen]
pub fn electric_field_2d(charges_json: &str, x_min: f64, x_max: f64, y_min: f64, y_max: f64, nx: usize, ny: usize) -> JsValue {
    let charges: Vec<(f64, f64, f64)> = serde_json::from_str(charges_json).unwrap_or_default();
    let pcs: Vec<point_charges::PointCharge> = charges
        .iter()
        .map(|&(x, y, q)| point_charges::PointCharge::new(x, y, 0.0, q))
        .collect();
    let (xs, ys, fields, potentials) = point_charges::sample_field_2d(&pcs, EPSILON_0, (x_min, x_max), (y_min, y_max), 0.0, nx, ny);
    let ex: Vec<f64> = fields.iter().map(|f| f.x).collect();
    let ey: Vec<f64> = fields.iter().map(|f| f.y).collect();
    let result = serde_json::json!({
        "x": xs, "y": ys,
        "ex": ex, "ey": ey,
        "potential": potentials,
    });
    serde_wasm_bindgen::to_value(&result).unwrap()
}

#[wasm_bindgen]
pub fn field_lines(charges_json: &str, source_idx: usize, num_lines: usize, num_steps: usize) -> JsValue {
    let charges: Vec<(f64, f64, f64)> = serde_json::from_str(charges_json).unwrap_or_default();
    let pcs: Vec<point_charges::PointCharge> = charges
        .iter()
        .map(|&(x, y, q)| point_charges::PointCharge::new(x, y, 0.0, q))
        .collect();
    let lines = point_charges::trace_field_lines(&pcs, source_idx, num_lines, num_steps, 0.005, EPSILON_0);
    let result: Vec<Vec<(f64, f64)>> = lines.iter().map(|line| {
        line.iter().map(|p| (p.x, p.y)).collect()
    }).collect();
    serde_wasm_bindgen::to_value(&result).unwrap()
}

#[wasm_bindgen]
pub fn charge_above_plane(charge: f64, height: f64, x_min: f64, x_max: f64, nx: usize) -> JsValue {
    let config = method_of_images::ChargeAbovePlane::new(charge, height);
    let dx = (x_max - x_min) / (nx - 1) as f64;
    let xs: Vec<f64> = (0..nx).map(|i| x_min + i as f64 * dx).collect();
    let sigma: Vec<f64> = xs.iter().map(|&x| config.surface_charge_density(x, 0.0)).collect();
    let force = config.force_on_charge();
    let result = serde_json::json!({
        "x": xs,
        "surface_charge_density": sigma,
        "force_z": force.z,
        "total_induced_charge": config.total_induced_charge(),
    });
    serde_wasm_bindgen::to_value(&result).unwrap()
}

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

#[wasm_bindgen]
pub fn inductance_calc(geometry: &str, params_json: &str) -> JsValue {
    use em_magnetostatics::inductance;
    let p: serde_json::Value = serde_json::from_str(params_json).unwrap_or_default();
    let mu_r = p["mu_r"].as_f64().unwrap_or(1.0);
    let current = p["current"].as_f64().unwrap_or(1.0);

    let ind = match geometry {
        "solenoid" => {
            let n = p["turns"].as_u64().unwrap_or(100) as usize;
            let l = p["length"].as_f64().unwrap_or(0.1);
            let r = p["radius"].as_f64().unwrap_or(0.02);
            inductance::solenoid(n, l, r, mu_r)
        }
        "toroid" => {
            let n = p["turns"].as_u64().unwrap_or(200) as usize;
            let a = p["inner_radius"].as_f64().unwrap_or(0.05);
            let b = p["outer_radius"].as_f64().unwrap_or(0.08);
            let h = p["height"].as_f64().unwrap_or(0.02);
            inductance::toroid(n, a, b, h, mu_r)
        }
        "coaxial" => {
            let a = p["inner_radius"].as_f64().unwrap_or(0.001);
            let b = p["outer_radius"].as_f64().unwrap_or(0.004);
            let l = p["length"].as_f64().unwrap_or(1.0);
            inductance::coaxial(a, b, mu_r, l)
        }
        "parallel_wires" => {
            let r = p["wire_radius"].as_f64().unwrap_or(0.001);
            let d = p["separation"].as_f64().unwrap_or(0.1);
            let l = p["length"].as_f64().unwrap_or(1.0);
            inductance::parallel_wires_per_length(r, d, mu_r) * l
        }
        _ => 0.0,
    };

    let w = inductance::energy(ind, current);
    let tau = if let Some(r) = p["resistance"].as_f64() {
        inductance::rl_time_constant(ind, r)
    } else { 0.0 };

    let result = serde_json::json!({
        "inductance": ind,
        "energy": w,
        "current": current,
        "time_constant": tau,
        "geometry": geometry,
    });
    serde_wasm_bindgen::to_value(&result).unwrap()
}

#[wasm_bindgen]
pub fn rl_step(voltage: f64, resistance: f64, inductance_val: f64, t_end: f64, num_points: usize) -> JsValue {
    use em_magnetostatics::inductance;
    let (ts, is) = inductance::rl_step_response(voltage, resistance, inductance_val, t_end, num_points);
    let tau = inductance::rl_time_constant(inductance_val, resistance);
    let result = serde_json::json!({
        "t": ts,
        "current": is,
        "time_constant": tau,
        "i_final": voltage / resistance,
    });
    serde_wasm_bindgen::to_value(&result).unwrap()
}

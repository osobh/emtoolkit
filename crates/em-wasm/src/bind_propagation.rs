//! WASM bindings for em-propagation.

use wasm_bindgen::prelude::*;
use em_propagation::{plane_wave, polarization, fresnel};

#[wasm_bindgen]
pub fn medium_properties(epsilon_r: f64, mu_r: f64, conductivity: f64, frequency: f64) -> JsValue {
    let m = plane_wave::Medium { epsilon_r, mu_r, conductivity };
    let omega = 2.0 * std::f64::consts::PI * frequency;
    let eta = m.intrinsic_impedance(omega);
    let result = serde_json::json!({
        "alpha": m.alpha(omega),
        "beta": m.beta(omega),
        "phase_velocity": m.phase_velocity(omega),
        "wavelength": m.wavelength(omega),
        "skin_depth": m.skin_depth(omega),
        "eta_re": eta.re,
        "eta_im": eta.im,
        "loss_tangent": m.loss_tangent(omega),
        "is_good_conductor": m.is_good_conductor(omega),
        "is_low_loss": m.is_low_loss(omega),
    });
    serde_wasm_bindgen::to_value(&result).unwrap()
}

#[wasm_bindgen]
pub fn polarization_state(ax: f64, ay: f64, delta_deg: f64, num_trace: usize) -> JsValue {
    let delta = delta_deg.to_radians();
    let p = polarization::PolarizationState::new(ax, ay, delta);
    let (ex, ey) = p.trace_ellipse(num_trace);
    let stokes = p.stokes_parameters();
    let poincare = p.poincare_point();
    let result = serde_json::json!({
        "type": format!("{:?}", p.polarization_type()),
        "rotation": format!("{:?}", p.rotation_sense()),
        "axial_ratio": p.axial_ratio(),
        "tilt_angle_deg": p.tilt_angle().to_degrees(),
        "stokes": stokes,
        "poincare": poincare,
        "trace_x": ex,
        "trace_y": ey,
    });
    serde_wasm_bindgen::to_value(&result).unwrap()
}

#[wasm_bindgen]
pub fn fresnel_normal(eta1: f64, eta2: f64) -> JsValue {
    let ni = fresnel::NormalIncidence::new(eta1, eta2);
    let result = serde_json::json!({
        "gamma": ni.gamma(),
        "tau": ni.tau(),
        "reflectance": ni.reflectance(),
        "transmittance": ni.transmittance(),
    });
    serde_wasm_bindgen::to_value(&result).unwrap()
}

#[wasm_bindgen]
pub fn fresnel_oblique(er1: f64, er2: f64, theta_i_deg: f64) -> JsValue {
    let theta_i = theta_i_deg.to_radians();
    let oi = fresnel::ObliqueIncidence::new(er1, er2, theta_i);
    let result = serde_json::json!({
        "theta_t_deg": oi.theta_t().map(|t| t.to_degrees()),
        "is_tir": oi.is_tir(),
        "critical_angle_deg": oi.critical_angle().map(|a| a.to_degrees()),
        "brewster_angle_deg": oi.brewster_angle().to_degrees(),
        "gamma_perp": oi.gamma_perp(),
        "gamma_par": oi.gamma_par(),
    });
    serde_wasm_bindgen::to_value(&result).unwrap()
}

#[wasm_bindgen]
pub fn fresnel_vs_angle(er1: f64, er2: f64, num_points: usize) -> JsValue {
    let sample = fresnel::ObliqueIncidence::sample_vs_angle(er1, er2, num_points);
    let angles_deg: Vec<f64> = sample.angles.iter().map(|a| a.to_degrees()).collect();
    let result = serde_json::json!({
        "angles_deg": angles_deg,
        "gamma_perp": sample.gamma_perp,
        "gamma_par": sample.gamma_par,
    });
    serde_wasm_bindgen::to_value(&result).unwrap()
}

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
pub fn skin_depth_vs_frequency(epsilon_r: f64, conductivity: f64, f_min: f64, f_max: f64, num_points: usize) -> JsValue {
    let m = plane_wave::Medium { epsilon_r, mu_r: 1.0, conductivity };
    let mut freqs = Vec::with_capacity(num_points);
    let mut depths = Vec::with_capacity(num_points);
    let mut alphas = Vec::with_capacity(num_points);
    let log_min = f_min.log10();
    let log_max = f_max.log10();
    for i in 0..num_points {
        let f = 10f64.powf(log_min + (log_max - log_min) * i as f64 / (num_points - 1) as f64);
        let omega = 2.0 * std::f64::consts::PI * f;
        freqs.push(f);
        depths.push(m.skin_depth(omega));
        alphas.push(m.alpha(omega));
    }
    let result = serde_json::json!({
        "frequencies": freqs,
        "skin_depths": depths,
        "alphas": alphas,
    });
    serde_wasm_bindgen::to_value(&result).unwrap()
}

#[wasm_bindgen]
pub fn attenuation_profile(epsilon_r: f64, conductivity: f64, frequency: f64, e0: f64, z_max: f64, num_points: usize) -> JsValue {
    let m = plane_wave::Medium { epsilon_r, mu_r: 1.0, conductivity };
    let omega = 2.0 * std::f64::consts::PI * frequency;
    let alpha = m.alpha(omega);
    let eta = m.intrinsic_impedance(omega);
    let mut z_vals = Vec::with_capacity(num_points);
    let mut e_vals = Vec::with_capacity(num_points);
    let mut s_vals = Vec::with_capacity(num_points);
    for i in 0..num_points {
        let z = z_max * i as f64 / (num_points - 1) as f64;
        z_vals.push(z);
        e_vals.push(plane_wave::e_field_magnitude(e0, alpha, z));
        s_vals.push(plane_wave::poynting_average(e0, alpha, eta, z));
    }
    let result = serde_json::json!({
        "z": z_vals,
        "e_magnitude": e_vals,
        "poynting": s_vals,
        "skin_depth": m.skin_depth(omega),
        "alpha": alpha,
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

#[wasm_bindgen]
pub fn waveguide_rect(a_mm: f64, b_mm: f64, epsilon_r: f64, frequency: f64) -> JsValue {
    use em_propagation::waveguide::RectWaveguide;
    let wg = RectWaveguide::new(a_mm / 1000.0, b_mm / 1000.0, epsilon_r, 1.0);
    let dominant = wg.mode_at_frequency(1, 0, frequency, "TE");
    let (f_single_min, f_single_max) = wg.single_mode_band();
    let modes = wg.modes_below(frequency * 1.5, 4);

    let mode_list: Vec<serde_json::Value> = modes.iter().map(|m| {
        serde_json::json!({
            "mode": format!("{}_{}{}", m.mode_type, m.m, m.n),
            "f_cutoff": m.f_cutoff,
            "propagates": m.propagates,
            "beta": m.beta,
            "lambda_g": m.lambda_g,
            "v_phase": m.v_phase,
            "v_group": m.v_group,
            "z_mode": m.z_mode,
        })
    }).collect();

    let result = serde_json::json!({
        "dominant_cutoff": dominant.f_cutoff,
        "dominant_propagates": dominant.propagates,
        "dominant_beta": dominant.beta,
        "dominant_lambda_g": dominant.lambda_g,
        "dominant_v_phase": dominant.v_phase,
        "dominant_v_group": dominant.v_group,
        "dominant_z_te": dominant.z_mode,
        "single_mode_min": f_single_min,
        "single_mode_max": f_single_max,
        "modes": mode_list,
    });
    serde_wasm_bindgen::to_value(&result).unwrap()
}

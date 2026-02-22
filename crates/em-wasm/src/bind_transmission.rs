//! WASM bindings for em-transmission.

use wasm_bindgen::prelude::*;
use num_complex::Complex64;
use em_transmission::{smith_chart, line_types, standing_waves, matching, stub_tuning, slotted_line, lossy_line};

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
    let data = standing_waves::pattern_from_load(z0, zl_re, zl_im, frequency, length, num_points);
    let result = serde_json::json!({
        "positions": data.positions,
        "voltage_mag": data.voltage_mag,
        "current_mag": data.current_mag,
        "vswr": data.vswr,
        "gamma_mag": data.gamma_mag,
        "gamma_angle_deg": data.gamma_angle.to_degrees(),
        "v_max": data.v_max,
        "v_min": data.v_min,
        "d_to_first_min": data.d_to_first_min,
        "d_to_first_max": data.d_to_first_max,
    });
    serde_wasm_bindgen::to_value(&result).unwrap()
}

#[wasm_bindgen]
pub fn standing_wave_from_vswr(z0: f64, vswr: f64, gamma_angle_deg: f64, frequency: f64, length: f64, num_points: usize) -> JsValue {
    let data = standing_waves::pattern_from_vswr(z0, vswr, gamma_angle_deg, frequency, length, num_points);
    let result = serde_json::json!({
        "positions": data.positions,
        "voltage_mag": data.voltage_mag,
        "current_mag": data.current_mag,
        "vswr": data.vswr,
        "gamma_mag": data.gamma_mag,
        "gamma_angle_deg": data.gamma_angle.to_degrees(),
        "v_max": data.v_max,
        "v_min": data.v_min,
        "d_to_first_min": data.d_to_first_min,
        "d_to_first_max": data.d_to_first_max,
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

#[wasm_bindgen]
pub fn double_stub_match(
    zl_re: f64,
    zl_im: f64,
    z0: f64,
    stub_separation: f64,
    use_short: bool,
) -> JsValue {
    let zl = Complex64::new(zl_re, zl_im);
    let stub_type = if use_short {
        stub_tuning::StubType::Short
    } else {
        stub_tuning::StubType::Open
    };
    use em_core::constants::C_0;
    
    match stub_tuning::double_stub(z0, zl, stub_separation, 1e9, C_0, stub_type) {
        Ok(solutions) => {
            let result: Vec<_> = solutions.iter().map(|s| {
                serde_json::json!({
                    "stub1_length_wavelengths": s.stub1_length_wavelengths,
                    "stub2_length_wavelengths": s.stub2_length_wavelengths,
                })
            }).collect();
            serde_wasm_bindgen::to_value(&serde_json::json!({
                "success": true,
                "solutions": result,
            })).unwrap()
        }
        Err(msg) => {
            serde_wasm_bindgen::to_value(&serde_json::json!({
                "success": false,
                "error": msg,
            })).unwrap()
        }
    }
}

#[wasm_bindgen]
pub fn series_stub_match(zl_re: f64, zl_im: f64, z0: f64, use_short: bool) -> JsValue {
    let zl = Complex64::new(zl_re, zl_im);
    let stub_type = if use_short {
        stub_tuning::StubType::Short
    } else {
        stub_tuning::StubType::Open
    };
    use em_core::constants::C_0;
    let solutions = stub_tuning::series_stub(z0, zl, 1e9, C_0, stub_type);
    let result: Vec<_> = solutions.iter().map(|s| {
        serde_json::json!({
            "stub_distance_wavelengths": s.stub_distance_wavelengths,
            "stub_length_wavelengths": s.stub_length_wavelengths,
        })
    }).collect();
    serde_wasm_bindgen::to_value(&result).unwrap()
}

#[wasm_bindgen]
pub fn l_network_match(z_source: f64, zl_re: f64, zl_im: f64, frequency: f64) -> JsValue {
    use em_transmission::l_network;
    let zl = Complex64::new(zl_re, zl_im);
    let solutions = l_network::l_network_match(z_source, zl, frequency);
    let result: Vec<_> = solutions.iter().map(|s| {
        serde_json::json!({
            "topology": format!("{}", s.topology),
            "series_value": s.series_value,
            "series_type": s.series_type,
            "shunt_value": s.shunt_value,
            "shunt_type": s.shunt_type,
            "q_factor": s.q_factor,
            "bandwidth_fractional": s.bandwidth_fractional,
            "description": s.description,
        })
    }).collect();
    serde_wasm_bindgen::to_value(&result).unwrap()
}

#[wasm_bindgen]
pub fn multisection_binomial(z0: f64, z_load: f64, num_sections: usize) -> JsValue {
    use em_transmission::multisection;
    let result = multisection::multisection_binomial(z0, z_load, num_sections);
    serde_wasm_bindgen::to_value(&serde_json::json!({
        "design_type": "Binomial",
        "num_sections": result.num_sections,
        "section_impedances": result.section_impedances,
        "junction_gammas": result.junction_gammas,
        "frequency_response": result.frequency_response,
    })).unwrap()
}

#[wasm_bindgen]
pub fn multisection_chebyshev(z0: f64, z_load: f64, num_sections: usize, max_gamma: f64) -> JsValue {
    use em_transmission::multisection;
    let result = multisection::multisection_chebyshev(z0, z_load, num_sections, max_gamma);
    serde_wasm_bindgen::to_value(&serde_json::json!({
        "design_type": "Chebyshev",
        "num_sections": result.num_sections,
        "section_impedances": result.section_impedances,
        "junction_gammas": result.junction_gammas,
        "fractional_bandwidth": result.fractional_bandwidth,
        "frequency_response": result.frequency_response,
    })).unwrap()
}

#[wasm_bindgen]
pub fn taper_design(z0: f64, z_load: f64, length_wavelengths: f64, taper_type_str: &str) -> JsValue {
    use em_transmission::taper::{self, TaperType};
    
    let taper_type = match taper_type_str.to_lowercase().as_str() {
        "exponential" => TaperType::Exponential,
        "triangular" => TaperType::Triangular,
        "klopfenstein" => TaperType::Klopfenstein,
        _ => TaperType::Exponential,
    };
    
    let result = taper::taper_design(z0, z_load, length_wavelengths, taper_type);
    serde_wasm_bindgen::to_value(&serde_json::json!({
        "taper_type": format!("{}", result.taper_type),
        "length_wavelengths": result.length_wavelengths,
        "impedance_profile": result.impedance_profile,
        "frequency_response": result.frequency_response,
        "max_ripple": result.max_ripple,
        "cutoff_beta_l": result.cutoff_beta_l,
    })).unwrap()
}

#[wasm_bindgen]
pub fn general_transformer_analysis(
    z0: f64,
    zl_re: f64,
    zl_im: f64,
    z_transformer: f64,
    length_wavelengths: f64,
    frequency: f64,
) -> JsValue {
    use em_transmission::general_transformer;
    let zl = Complex64::new(zl_re, zl_im);
    let result = general_transformer::general_transformer(z0, zl, z_transformer, length_wavelengths, frequency);
    serde_wasm_bindgen::to_value(&serde_json::json!({
        "z_transformer": result.z_transformer,
        "length_wavelengths": result.length_wavelengths,
        "z_in_re": result.z_in.re,
        "z_in_im": result.z_in.im,
        "gamma_re": result.gamma_in.re,
        "gamma_im": result.gamma_in.im,
        "gamma_mag": result.gamma_mag,
        "vswr": result.vswr,
        "frequency_response": result.frequency_response,
    })).unwrap()
}

#[wasm_bindgen]
pub fn two_line_transformer_analysis(
    z0: f64,
    zl_re: f64,
    zl_im: f64,
    z1: f64,
    l1_wavelengths: f64,
    z2: f64,
    l2_wavelengths: f64,
    frequency: f64,
) -> JsValue {
    use em_transmission::general_transformer;
    let zl = Complex64::new(zl_re, zl_im);
    let result = general_transformer::two_line_transformer(z0, zl, z1, l1_wavelengths, z2, l2_wavelengths, frequency);
    serde_wasm_bindgen::to_value(&serde_json::json!({
        "z1": result.z1,
        "l1_wavelengths": result.l1_wavelengths,
        "z2": result.z2,
        "l2_wavelengths": result.l2_wavelengths,
        "z_in_re": result.z_in.re,
        "z_in_im": result.z_in.im,
        "gamma_re": result.gamma_in.re,
        "gamma_im": result.gamma_in.im,
        "gamma_mag": result.gamma_mag,
        "vswr": result.vswr,
        "frequency_response": result.frequency_response,
    })).unwrap()
}

// ============================================================================
// Stripline parameters
// ============================================================================

#[wasm_bindgen]
pub fn stripline_params(width: f64, ground_spacing: f64, epsilon_r: f64) -> JsValue {
    let sl = line_types::StriplineLine::new(width, ground_spacing, epsilon_r);
    let params = sl.parameters();
    let result = serde_json::json!({
        "z0": sl.characteristic_impedance(),
        "epsilon_eff": sl.effective_epsilon_r(),
        "phase_velocity": sl.phase_velocity(),
        "l_per_m": params.l_per_m,
        "c_per_m": params.c_per_m,
    });
    serde_wasm_bindgen::to_value(&result).unwrap()
}

// ============================================================================
// Two-wire line parameters
// ============================================================================

#[wasm_bindgen]
pub fn two_wire_params(wire_radius: f64, separation: f64, epsilon_r: f64) -> JsValue {
    let line = line_types::TwoWireLine::lossless(wire_radius, separation, epsilon_r);
    let params = line.parameters(0.0); // frequency = 0 for lossless
    let result = serde_json::json!({
        "z0": params.z0_lossless(),
        "phase_velocity": params.phase_velocity_lossless(),
        "l_per_m": params.l_per_m,
        "c_per_m": params.c_per_m,
    });
    serde_wasm_bindgen::to_value(&result).unwrap()
}

// ============================================================================
// Lossy Smith Chart trace
// ============================================================================

#[wasm_bindgen]
pub fn smith_chart_lossy_trace(
    zl_re: f64,
    zl_im: f64,
    alpha: f64,
    beta: f64,
    length: f64,
    num_points: usize,
) -> JsValue {
    let sp = smith_chart::SmithPoint::from_impedance(Complex64::new(zl_re, zl_im));
    let trace = smith_chart::trace_lossy(&sp, alpha, beta, length, num_points);
    let xs: Vec<f64> = trace.iter().map(|p| p.0).collect();
    let ys: Vec<f64> = trace.iter().map(|p| p.1).collect();
    let result = serde_json::json!({ "gamma_re": xs, "gamma_im": ys });
    serde_wasm_bindgen::to_value(&result).unwrap()
}

// ============================================================================
// Slotted line measurement
// ============================================================================

#[wasm_bindgen]
pub fn slotted_line_measurement(
    v_max: f64,
    v_min: f64,
    d_min_from_load: f64,
    wavelength: f64,
    z0: f64,
) -> JsValue {
    let result = slotted_line::slotted_line_measurement(v_max, v_min, d_min_from_load, wavelength, z0);
    let json = serde_json::json!({
        "vswr": result.vswr,
        "gamma_magnitude": result.gamma_magnitude,
        "gamma_angle_deg": result.gamma_angle_deg,
        "gamma_re": result.gamma_re,
        "gamma_im": result.gamma_im,
        "zl_re": result.zl_re,
        "zl_im": result.zl_im,
        "zl_norm_re": result.zl_norm_re,
        "zl_norm_im": result.zl_norm_im,
    });
    serde_wasm_bindgen::to_value(&json).unwrap()
}

#[wasm_bindgen]
pub fn slotted_line_standing_wave(
    gamma_magnitude: f64,
    gamma_angle_rad: f64,
    num_wavelengths: f64,
    num_points: usize,
) -> JsValue {
    let (positions, voltages) = slotted_line::standing_wave_pattern(
        gamma_magnitude,
        gamma_angle_rad,
        num_wavelengths,
        num_points,
    );
    let result = serde_json::json!({
        "positions": positions,
        "voltages": voltages,
    });
    serde_wasm_bindgen::to_value(&result).unwrap()
}

// ============================================================================
// Lossy line analysis
// ============================================================================

#[wasm_bindgen]
pub fn lossy_line_analysis(
    r: f64,
    l: f64,
    g: f64,
    c: f64,
    frequency: f64,
    length: f64,
    z_load_re: f64,
    z_load_im: f64,
) -> JsValue {
    let result = lossy_line::lossy_line_analysis(r, l, g, c, frequency, length, z_load_re, z_load_im);
    let json = serde_json::json!({
        "alpha": result.alpha,
        "beta": result.beta,
        "gamma_re": result.gamma_re,
        "gamma_im": result.gamma_im,
        "z0_re": result.z0_re,
        "z0_im": result.z0_im,
        "zin_re": result.zin_re,
        "zin_im": result.zin_im,
        "gamma_in_mag": result.gamma_in_mag,
        "gamma_in_phase_deg": result.gamma_in_phase_deg,
        "power_loss_ratio": result.power_loss_ratio,
        "power_loss_db": result.power_loss_db,
        "phase_velocity": result.phase_velocity,
        "wavelength": result.wavelength,
    });
    serde_wasm_bindgen::to_value(&json).unwrap()
}

#[wasm_bindgen]
pub fn lossy_line_profile(
    z0_re: f64,
    z0_im: f64,
    gamma_re: f64,
    gamma_im: f64,
    z_load_re: f64,
    z_load_im: f64,
    length: f64,
    num_points: usize,
) -> JsValue {
    let (positions, voltages, currents) = lossy_line::lossy_line_profile(
        z0_re, z0_im, gamma_re, gamma_im, z_load_re, z_load_im, length, num_points,
    );
    let result = serde_json::json!({
        "positions": positions,
        "voltages": voltages,
        "currents": currents,
    });
    serde_wasm_bindgen::to_value(&result).unwrap()
}

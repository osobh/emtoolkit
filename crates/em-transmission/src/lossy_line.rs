//! General lossy transmission line analysis.
//!
//! Computes propagation constant, characteristic impedance, input impedance,
//! and power loss for transmission lines with distributed R, L, G, C parameters.

use num_complex::Complex64;
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

/// Result of lossy transmission line analysis.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LossyLineResult {
    /// Attenuation constant α (Np/m)
    pub alpha: f64,
    /// Phase constant β (rad/m)
    pub beta: f64,
    /// Complex propagation constant γ = α + jβ
    pub gamma_re: f64,
    pub gamma_im: f64,
    /// Complex characteristic impedance Z₀ (Ω)
    pub z0_re: f64,
    pub z0_im: f64,
    /// Complex input impedance Z_in (Ω)
    pub zin_re: f64,
    pub zin_im: f64,
    /// Input reflection coefficient magnitude
    pub gamma_in_mag: f64,
    /// Input reflection coefficient phase (degrees)
    pub gamma_in_phase_deg: f64,
    /// Power loss ratio (P_in - P_load) / P_in
    pub power_loss_ratio: f64,
    /// Power loss in dB
    pub power_loss_db: f64,
    /// Phase velocity (m/s)
    pub phase_velocity: f64,
    /// Wavelength on the line (m)
    pub wavelength: f64,
}

/// Analyze a general lossy transmission line.
///
/// # Arguments
/// * `r` - Resistance per unit length (Ω/m)
/// * `l` - Inductance per unit length (H/m)
/// * `g` - Conductance per unit length (S/m)
/// * `c` - Capacitance per unit length (F/m)
/// * `frequency` - Operating frequency (Hz)
/// * `length` - Line length (m)
/// * `z_load_re` - Real part of load impedance (Ω)
/// * `z_load_im` - Imaginary part of load impedance (Ω)
///
/// # Returns
/// Complete analysis including γ, Z₀, Z_in, and power loss.
///
/// # Theory
/// - γ = √((R + jωL)(G + jωC)) = α + jβ
/// - Z₀ = √((R + jωL)/(G + jωC))
/// - Z_in = Z₀ · (Z_L + Z₀·tanh(γℓ)) / (Z₀ + Z_L·tanh(γℓ))
pub fn lossy_line_analysis(
    r: f64,
    l: f64,
    g: f64,
    c: f64,
    frequency: f64,
    length: f64,
    z_load_re: f64,
    z_load_im: f64,
) -> LossyLineResult {
    let omega = 2.0 * PI * frequency;
    let z_load = Complex64::new(z_load_re, z_load_im);

    // Series impedance per unit length: Z = R + jωL
    let z_series = Complex64::new(r, omega * l);
    
    // Shunt admittance per unit length: Y = G + jωC
    let y_shunt = Complex64::new(g, omega * c);

    // Propagation constant: γ = √(ZY)
    let gamma = (z_series * y_shunt).sqrt();
    // Ensure α ≥ 0 (choose the root with positive real part)
    let gamma = if gamma.re < 0.0 { -gamma } else { gamma };
    let alpha = gamma.re;
    let beta = gamma.im;

    // Characteristic impedance: Z₀ = √(Z/Y)
    let z0 = (z_series / y_shunt).sqrt();
    // Choose root with positive real part (typical for passive lines)
    let z0 = if z0.re < 0.0 { -z0 } else { z0 };

    // Input impedance: Z_in = Z₀ · (Z_L + Z₀·tanh(γℓ)) / (Z₀ + Z_L·tanh(γℓ))
    let gamma_l = gamma * length;
    let tanh_gamma_l = gamma_l.tanh();
    let z_in = z0 * (z_load + z0 * tanh_gamma_l) / (z0 + z_load * tanh_gamma_l);

    // Input reflection coefficient: Γ_in = (Z_in - Z₀) / (Z_in + Z₀)
    let gamma_in = (z_in - z0) / (z_in + z0);
    let gamma_in_mag = gamma_in.norm();
    let gamma_in_phase_deg = gamma_in.arg().to_degrees();

    // Power loss calculation
    // For a matched source (Z_s = Z₀*), power delivered to line is maximized
    // Power at load: P_L = P_in · e^{-2αℓ} (for matched conditions)
    // Power loss ratio = 1 - e^{-2αℓ}
    let power_transmission = (-2.0 * alpha * length).exp();
    let power_loss_ratio = 1.0 - power_transmission;
    let power_loss_db = if power_transmission > 0.0 {
        -10.0 * power_transmission.log10()
    } else {
        f64::INFINITY
    };

    // Phase velocity and wavelength
    let phase_velocity = if beta > 0.0 { omega / beta } else { f64::INFINITY };
    let wavelength = if beta > 0.0 { 2.0 * PI / beta } else { f64::INFINITY };

    LossyLineResult {
        alpha,
        beta,
        gamma_re: gamma.re,
        gamma_im: gamma.im,
        z0_re: z0.re,
        z0_im: z0.im,
        zin_re: z_in.re,
        zin_im: z_in.im,
        gamma_in_mag,
        gamma_in_phase_deg,
        power_loss_ratio,
        power_loss_db,
        phase_velocity,
        wavelength,
    }
}

/// Compute voltage and current along a lossy line for visualization.
///
/// # Arguments
/// * `z0` - Complex characteristic impedance
/// * `gamma` - Complex propagation constant
/// * `z_load` - Complex load impedance
/// * `length` - Total line length (m)
/// * `num_points` - Number of sample points
///
/// # Returns
/// (positions, voltage_magnitudes, current_magnitudes) normalized to load values
pub fn lossy_line_profile(
    z0_re: f64,
    z0_im: f64,
    gamma_re: f64,
    gamma_im: f64,
    z_load_re: f64,
    z_load_im: f64,
    length: f64,
    num_points: usize,
) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    let z0 = Complex64::new(z0_re, z0_im);
    let gamma = Complex64::new(gamma_re, gamma_im);
    let z_load = Complex64::new(z_load_re, z_load_im);

    // Reflection coefficient at load
    let gamma_l = (z_load - z0) / (z_load + z0);

    let mut positions = Vec::with_capacity(num_points);
    let mut voltages = Vec::with_capacity(num_points);
    let mut currents = Vec::with_capacity(num_points);

    for i in 0..num_points {
        // z is distance from load (z=0 at load, z=length at source)
        let z = length * i as f64 / (num_points - 1).max(1) as f64;
        
        // V(z) = V₀⁺ · (e^{γz} + Γ_L·e^{-γz})
        // I(z) = (V₀⁺/Z₀) · (e^{γz} - Γ_L·e^{-γz})
        let exp_pos = (gamma * z).exp();
        let exp_neg = (-gamma * z).exp();
        
        let v = exp_pos + gamma_l * exp_neg;
        let i_val = (exp_pos - gamma_l * exp_neg) / z0;

        positions.push(z);
        voltages.push(v.norm());
        currents.push(i_val.norm());
    }

    // Normalize to values at z=0 (load)
    let v0 = voltages[0];
    let i0 = currents[0];
    for v in &mut voltages {
        *v /= v0;
    }
    for i in &mut currents {
        *i /= i0;
    }

    (positions, voltages, currents)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn lossless_line_has_zero_alpha() {
        // Lossless: R = 0, G = 0
        let result = lossy_line_analysis(
            0.0,      // R
            250e-9,   // L
            0.0,      // G
            100e-12,  // C
            1e9,      // frequency
            1.0,      // length
            50.0,     // Z_L real
            0.0,      // Z_L imag
        );
        assert_relative_eq!(result.alpha, 0.0, epsilon = 1e-15);
    }

    #[test]
    fn lossless_50_ohm_line() {
        // L/C ratio for 50Ω: Z₀ = √(L/C) = 50 → L/C = 2500
        let result = lossy_line_analysis(
            0.0,      // R
            250e-9,   // L = 250 nH/m
            0.0,      // G
            100e-12,  // C = 100 pF/m → √(L/C) = 50Ω
            1e9,
            1.0,
            50.0,
            0.0,
        );
        assert_relative_eq!(result.z0_re, 50.0, max_relative = 1e-6);
        assert_relative_eq!(result.z0_im, 0.0, epsilon = 1e-10);
    }

    #[test]
    fn matched_load_gives_zin_equals_z0() {
        let result = lossy_line_analysis(
            0.0, 250e-9, 0.0, 100e-12,
            1e9, 1.0,
            50.0, 0.0, // Z_L = Z₀ = 50Ω
        );
        assert_relative_eq!(result.zin_re, 50.0, max_relative = 1e-6);
        assert_relative_eq!(result.zin_im, 0.0, epsilon = 1e-6);
        assert_relative_eq!(result.gamma_in_mag, 0.0, epsilon = 1e-10);
    }

    #[test]
    fn lossy_line_has_positive_alpha() {
        let result = lossy_line_analysis(
            0.5,      // R = 0.5 Ω/m
            250e-9,
            1e-5,     // G = 10 µS/m
            100e-12,
            1e9,
            1.0,
            50.0,
            0.0,
        );
        assert!(result.alpha > 0.0, "α should be positive for lossy line");
    }

    #[test]
    fn lossy_line_z0_has_negative_imaginary() {
        // For series resistance dominant, Z₀ has negative imaginary part
        let result = lossy_line_analysis(
            10.0,     // Significant R
            250e-9,
            0.0,
            100e-12,
            1e9,
            1.0,
            50.0,
            0.0,
        );
        assert!(result.z0_im < 0.0, "Z₀ should have negative imaginary part for series loss");
    }

    #[test]
    fn power_loss_increases_with_length() {
        let short = lossy_line_analysis(
            1.0, 250e-9, 1e-5, 100e-12,
            1e9, 1.0,
            50.0, 0.0,
        );
        let long = lossy_line_analysis(
            1.0, 250e-9, 1e-5, 100e-12,
            1e9, 10.0,
            50.0, 0.0,
        );
        assert!(long.power_loss_ratio > short.power_loss_ratio);
        assert!(long.power_loss_db > short.power_loss_db);
    }

    #[test]
    fn lossless_line_no_power_loss() {
        let result = lossy_line_analysis(
            0.0, 250e-9, 0.0, 100e-12,
            1e9, 1.0,
            50.0, 0.0,
        );
        assert_relative_eq!(result.power_loss_ratio, 0.0, epsilon = 1e-15);
        assert_relative_eq!(result.power_loss_db, 0.0, epsilon = 1e-15);
    }

    #[test]
    fn phase_velocity_on_lossless_line() {
        // For lossless line: v_p = 1/√(LC)
        let l: f64 = 250e-9;
        let c: f64 = 100e-12;
        let expected_vp = 1.0 / (l * c).sqrt();
        
        let result = lossy_line_analysis(
            0.0, l, 0.0, c,
            1e9, 1.0,
            50.0, 0.0,
        );
        assert_relative_eq!(result.phase_velocity, expected_vp, max_relative = 1e-6);
    }

    #[test]
    fn half_wave_restores_load_impedance() {
        // Half-wave line: Z_in = Z_L
        let l_per_m: f64 = 250e-9;
        let c_per_m: f64 = 100e-12;
        let f: f64 = 1e9;
        let vp = 1.0 / (l_per_m * c_per_m).sqrt();
        let wavelength = vp / f;
        let half_wave = wavelength / 2.0;
        
        let result = lossy_line_analysis(
            0.0, l_per_m, 0.0, c_per_m,
            f, half_wave,
            75.0, 25.0, // Z_L = 75 + j25 Ω
        );
        // Z_in should equal Z_L for half-wave lossless line
        assert_relative_eq!(result.zin_re, 75.0, max_relative = 0.01);
        assert_relative_eq!(result.zin_im, 25.0, max_relative = 0.01);
    }

    #[test]
    fn profile_starts_at_load_normalized_to_1() {
        let (pos, v, i) = lossy_line_profile(
            50.0, 0.0,    // Z₀
            0.01, 20.0,   // γ
            100.0, 0.0,   // Z_L
            1.0,          // length
            100,
        );
        assert_eq!(pos.len(), 100);
        assert_relative_eq!(v[0], 1.0, epsilon = 1e-10);
        assert_relative_eq!(i[0], 1.0, epsilon = 1e-10);
    }

    #[test]
    fn profile_voltage_changes_along_lossy_line() {
        let (_, v, _) = lossy_line_profile(
            50.0, 0.0,
            0.1, 20.0,  // Significant attenuation
            100.0, 0.0,
            1.0,
            100,
        );
        // Voltage should change from load to source
        let v_at_source = v[v.len() - 1];
        assert!((v_at_source - 1.0).abs() > 0.01, "voltage should change along lossy line");
    }
}

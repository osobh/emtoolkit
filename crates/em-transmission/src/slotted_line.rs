//! Slotted line measurement analysis.
//!
//! Computes load impedance from slotted line voltage measurements.
//! Classic transmission line measurement technique using VSWR and
//! the position of voltage minima.

use num_complex::Complex64;
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

/// Result of a slotted line measurement analysis.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SlottedLineResult {
    /// Voltage Standing Wave Ratio
    pub vswr: f64,
    /// Magnitude of reflection coefficient |Γ|
    pub gamma_magnitude: f64,
    /// Phase angle of reflection coefficient (radians)
    pub gamma_angle_rad: f64,
    /// Phase angle of reflection coefficient (degrees)
    pub gamma_angle_deg: f64,
    /// Complex reflection coefficient
    pub gamma_re: f64,
    pub gamma_im: f64,
    /// Complex load impedance (Ω)
    pub zl_re: f64,
    pub zl_im: f64,
    /// Normalized load impedance (z_L = Z_L / Z₀)
    pub zl_norm_re: f64,
    pub zl_norm_im: f64,
}

/// Perform slotted line measurement analysis.
///
/// Given voltage maximum and minimum readings, plus the distance from
/// the load to the first voltage minimum, computes the load impedance.
///
/// # Arguments
/// * `v_max` - Maximum voltage reading on the slotted line
/// * `v_min` - Minimum voltage reading on the slotted line
/// * `d_min_from_load` - Distance from load to first voltage minimum (m)
/// * `wavelength` - Operating wavelength (m)
/// * `z0` - Characteristic impedance of the line (Ω)
///
/// # Returns
/// Complete slotted line analysis result including VSWR, Γ, and Z_L.
///
/// # Theory
/// - VSWR = V_max / V_min
/// - |Γ| = (VSWR - 1) / (VSWR + 1)
/// - θ_Γ = π - 2βd_min where β = 2π/λ and d_min is distance to first minimum
/// - Z_L = Z₀ · (1 + Γ) / (1 - Γ)
pub fn slotted_line_measurement(
    v_max: f64,
    v_min: f64,
    d_min_from_load: f64,
    wavelength: f64,
    z0: f64,
) -> SlottedLineResult {
    // Compute VSWR
    let vswr = v_max / v_min;

    // Compute |Γ| from VSWR
    let gamma_magnitude = (vswr - 1.0) / (vswr + 1.0);

    // Compute phase of Γ
    // At a voltage minimum, the incident and reflected waves are 180° out of phase
    // The minimum closest to the load gives: θ_Γ = π - 2βd_min
    let beta = 2.0 * PI / wavelength;
    let gamma_angle_rad = PI - 2.0 * beta * d_min_from_load;
    
    // Normalize angle to [-π, π]
    let gamma_angle_normalized = normalize_angle(gamma_angle_rad);
    let gamma_angle_deg = gamma_angle_normalized.to_degrees();

    // Complex reflection coefficient
    let gamma = Complex64::from_polar(gamma_magnitude, gamma_angle_normalized);

    // Compute load impedance: Z_L = Z₀ · (1 + Γ) / (1 - Γ)
    let one = Complex64::new(1.0, 0.0);
    let z_norm = (one + gamma) / (one - gamma);
    let z_load = z_norm * z0;

    SlottedLineResult {
        vswr,
        gamma_magnitude,
        gamma_angle_rad: gamma_angle_normalized,
        gamma_angle_deg,
        gamma_re: gamma.re,
        gamma_im: gamma.im,
        zl_re: z_load.re,
        zl_im: z_load.im,
        zl_norm_re: z_norm.re,
        zl_norm_im: z_norm.im,
    }
}

/// Generate standing wave pattern data for visualization.
///
/// # Arguments
/// * `gamma_magnitude` - |Γ| of the load
/// * `gamma_angle_rad` - Phase of Γ in radians
/// * `num_wavelengths` - Number of wavelengths to plot
/// * `num_points` - Number of sample points
///
/// # Returns
/// Tuple of (positions in wavelengths from load, normalized voltage magnitudes)
pub fn standing_wave_pattern(
    gamma_magnitude: f64,
    gamma_angle_rad: f64,
    num_wavelengths: f64,
    num_points: usize,
) -> (Vec<f64>, Vec<f64>) {
    let mut positions = Vec::with_capacity(num_points);
    let mut voltages = Vec::with_capacity(num_points);

    for i in 0..num_points {
        let z_wavelengths = num_wavelengths * i as f64 / (num_points - 1).max(1) as f64;
        let beta_z = 2.0 * PI * z_wavelengths;
        
        // V(z) = V⁺ · (1 + Γ·e^{-j2βz}) where z is distance from load
        // Moving away from load means we go toward generator, so rotation is -2βz
        // |V(z)| = |V⁺| · |1 + |Γ|·e^{j(θ_Γ - 2βz)}|
        let phase = gamma_angle_rad - 2.0 * beta_z;
        let v_mag = (1.0 + gamma_magnitude * phase.cos()).hypot(gamma_magnitude * phase.sin());
        
        positions.push(z_wavelengths);
        voltages.push(v_mag);
    }

    (positions, voltages)
}

/// Normalize angle to [-π, π] range.
fn normalize_angle(angle: f64) -> f64 {
    let mut a = angle % (2.0 * PI);
    if a > PI {
        a -= 2.0 * PI;
    } else if a < -PI {
        a += 2.0 * PI;
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn matched_load_vswr_is_1() {
        // For a matched load, Vmax = Vmin, VSWR = 1
        let result = slotted_line_measurement(1.0, 1.0, 0.0, 0.1, 50.0);
        assert_relative_eq!(result.vswr, 1.0, epsilon = 1e-12);
        assert_relative_eq!(result.gamma_magnitude, 0.0, epsilon = 1e-12);
    }

    #[test]
    fn vswr_2_gives_gamma_one_third() {
        // VSWR = 2 → |Γ| = (2-1)/(2+1) = 1/3
        let result = slotted_line_measurement(2.0, 1.0, 0.0, 0.1, 50.0);
        assert_relative_eq!(result.vswr, 2.0, epsilon = 1e-12);
        assert_relative_eq!(result.gamma_magnitude, 1.0 / 3.0, epsilon = 1e-12);
    }

    #[test]
    fn vswr_3_gives_gamma_half() {
        // VSWR = 3 → |Γ| = (3-1)/(3+1) = 0.5
        let result = slotted_line_measurement(3.0, 1.0, 0.0, 0.1, 50.0);
        assert_relative_eq!(result.vswr, 3.0, epsilon = 1e-12);
        assert_relative_eq!(result.gamma_magnitude, 0.5, epsilon = 1e-12);
    }

    #[test]
    fn minimum_at_load_gives_gamma_angle_pi() {
        // d_min = 0 → θ_Γ = π (short circuit like behavior)
        let result = slotted_line_measurement(2.0, 1.0, 0.0, 0.1, 50.0);
        assert_relative_eq!(result.gamma_angle_rad, PI, epsilon = 1e-12);
    }

    #[test]
    fn minimum_at_quarter_wavelength() {
        // d_min = λ/4 → θ_Γ = π - 2·(2π/λ)·(λ/4) = π - π = 0
        let wavelength = 0.1;
        let result = slotted_line_measurement(2.0, 1.0, wavelength / 4.0, wavelength, 50.0);
        assert_relative_eq!(result.gamma_angle_rad, 0.0, epsilon = 1e-10);
    }

    #[test]
    fn open_circuit_detection() {
        // Open circuit: very high VSWR, Γ ≈ +1∠0°
        // Minimum at λ/4 from load indicates open circuit
        let wavelength = 0.1;
        let result = slotted_line_measurement(1000.0, 1.0, wavelength / 4.0, wavelength, 50.0);
        assert!(result.gamma_magnitude > 0.99);
        assert!(result.gamma_angle_rad.abs() < 0.01); // ≈ 0°
        // Z_L should be very high (open circuit)
        assert!(result.zl_re > 1000.0);
    }

    #[test]
    fn short_circuit_detection() {
        // Short circuit: very high VSWR, Γ ≈ -1 (180°)
        // Minimum right at load indicates short circuit
        let wavelength = 0.1;
        let result = slotted_line_measurement(1000.0, 1.0, 0.0, wavelength, 50.0);
        assert!(result.gamma_magnitude > 0.99);
        assert_relative_eq!(result.gamma_angle_rad, PI, epsilon = 0.01);
        // Z_L should be very small (short circuit)
        assert!(result.zl_re.abs() < 0.1);
    }

    #[test]
    fn purely_resistive_load_2z0() {
        // Z_L = 2·Z₀ = 100Ω with Z₀ = 50Ω
        // Γ = (2-1)/(2+1) = 1/3, angle = 0° (minimum at λ/4)
        let wavelength = 0.1;
        let z0 = 50.0;
        let result = slotted_line_measurement(2.0, 1.0, wavelength / 4.0, wavelength, z0);
        // z_norm = 1+Γ)/(1-Γ) with Γ = 1/3 → z_norm = (4/3)/(2/3) = 2
        assert_relative_eq!(result.zl_norm_re, 2.0, max_relative = 0.01);
        assert_relative_eq!(result.zl_norm_im, 0.0, epsilon = 0.01);
        assert_relative_eq!(result.zl_re, 100.0, max_relative = 0.01);
    }

    #[test]
    fn standing_wave_has_correct_vswr_ratio() {
        let gamma_mag = 0.5; // VSWR = 3
        let (_, voltages) = standing_wave_pattern(gamma_mag, 0.0, 1.0, 1000);
        let v_max = voltages.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let v_min = voltages.iter().cloned().fold(f64::INFINITY, f64::min);
        let measured_vswr = v_max / v_min;
        // VSWR = (1+|Γ|)/(1-|Γ|) = 1.5/0.5 = 3
        assert_relative_eq!(measured_vswr, 3.0, max_relative = 0.01);
    }

    #[test]
    fn standing_wave_period_is_half_wavelength() {
        let (positions, voltages) = standing_wave_pattern(0.5, 0.0, 2.0, 1000);
        // Find minima
        let mut minima_positions = Vec::new();
        for i in 1..voltages.len() - 1 {
            if voltages[i] < voltages[i - 1] && voltages[i] < voltages[i + 1] {
                minima_positions.push(positions[i]);
            }
        }
        // Should have ~4 minima in 2 wavelengths
        assert!(minima_positions.len() >= 3);
        // Spacing should be λ/2
        if minima_positions.len() >= 2 {
            let spacing = minima_positions[1] - minima_positions[0];
            assert_relative_eq!(spacing, 0.5, max_relative = 0.02);
        }
    }
}

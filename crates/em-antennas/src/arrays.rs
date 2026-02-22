//! Antenna array analysis — uniform linear arrays (ULA).
//!
//! Array factor, beam steering, broadside/endfire configurations.

use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

/// Uniform Linear Array (ULA) along the z-axis.
///
/// Array factor: AF(θ) = sin(Nψ/2) / (N sin(ψ/2))
/// where ψ = kd·cos(θ) + β
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct UniformLinearArray {
    /// Number of elements
    pub num_elements: usize,
    /// Element spacing in wavelengths (d/λ)
    pub spacing: f64,
    /// Progressive phase shift β (radians)
    pub beta: f64,
}

impl UniformLinearArray {
    pub fn new(num_elements: usize, spacing: f64, beta: f64) -> Self {
        assert!(num_elements >= 2, "need at least 2 elements");
        Self {
            num_elements,
            spacing,
            beta,
        }
    }

    /// Broadside array: β = 0, main beam at θ = 90°.
    pub fn broadside(num_elements: usize, spacing: f64) -> Self {
        Self::new(num_elements, spacing, 0.0)
    }

    /// Endfire array: β = -kd, main beam at θ = 0°.
    pub fn endfire(num_elements: usize, spacing: f64) -> Self {
        let beta = -2.0 * PI * spacing; // -kd where k=2π/λ, d in wavelengths
        Self::new(num_elements, spacing, beta)
    }

    /// Scanned array: main beam steered to θ₀.
    pub fn scanned(num_elements: usize, spacing: f64, theta_0: f64) -> Self {
        let beta = -2.0 * PI * spacing * theta_0.cos();
        Self::new(num_elements, spacing, beta)
    }

    /// ψ = kd·cos(θ) + β
    pub fn psi(&self, theta: f64) -> f64 {
        2.0 * PI * self.spacing * theta.cos() + self.beta
    }

    /// Normalized array factor |AF(θ)| / N.
    ///
    /// AF = sin(Nψ/2) / (N·sin(ψ/2))
    pub fn array_factor(&self, theta: f64) -> f64 {
        let n = self.num_elements as f64;
        let psi = self.psi(theta);
        let half_psi = psi / 2.0;

        if half_psi.sin().abs() < 1e-12 {
            // At ψ = 0, 2π, ... → AF = 1 (main beam)
            return 1.0;
        }

        ((n * half_psi).sin() / (n * half_psi.sin())).abs()
    }

    /// First-null beamwidth (FNBW) in radians.
    ///
    /// For broadside: FNBW ≈ 2·arcsin(λ/(N·d)) ≈ 2λ/(N·d) for large arrays.
    pub fn first_null_beamwidth(&self) -> f64 {
        let n = self.num_elements as f64;
        let nd = n * self.spacing;
        if nd > 1.0 {
            2.0 * (1.0 / nd).asin()
        } else {
            PI // entire hemisphere
        }
    }

    /// Directivity of the array (approximate for large N).
    ///
    /// For broadside with d = λ/2: D ≈ 2N·d/λ = N (for d = λ/2)
    pub fn directivity_approx(&self) -> f64 {
        2.0 * self.num_elements as f64 * self.spacing
    }

    /// Sample the array factor pattern.
    pub fn sample_pattern(&self, num_points: usize) -> (Vec<f64>, Vec<f64>) {
        let dtheta = PI / (num_points - 1) as f64;
        let thetas: Vec<f64> = (0..num_points).map(|i| i as f64 * dtheta).collect();
        let af: Vec<f64> = thetas.iter().map(|&t| self.array_factor(t)).collect();
        (thetas, af)
    }

    /// Sample the total pattern (element × array factor).
    ///
    /// Uses sin(θ) element pattern (short dipole).
    pub fn sample_total_pattern(&self, num_points: usize) -> (Vec<f64>, Vec<f64>) {
        let dtheta = PI / (num_points - 1) as f64;
        let thetas: Vec<f64> = (0..num_points).map(|i| i as f64 * dtheta).collect();
        let pattern: Vec<f64> = thetas.iter().map(|&t| {
            t.sin() * self.array_factor(t)
        }).collect();
        (thetas, pattern)
    }
}

/// Result from two-element array analysis.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TwoElementResult {
    /// Angles in radians (0 to π)
    pub angles: Vec<f64>,
    /// Combined pattern in dB (element × array factor)
    pub pattern_db: Vec<f64>,
    /// Element pattern (linear, not dB)
    pub element_pattern: Vec<f64>,
    /// Array factor (linear, not dB)
    pub array_factor: Vec<f64>,
    /// Half-power beamwidth in degrees
    pub beamwidth_deg: f64,
    /// Approximate directivity
    pub directivity: f64,
    /// Main beam direction in degrees
    pub main_beam_deg: f64,
}

/// Two-element antenna array analysis.
///
/// Combines dipole element pattern with two-element array factor.
pub fn two_element_array(
    element_length_wavelengths: f64,
    spacing_wavelengths: f64,
    phase_shift_deg: f64,
    num_points: usize,
) -> TwoElementResult {
    let phase_shift = phase_shift_deg.to_radians();
    let k = 2.0 * PI; // k = 2π/λ, and we're using wavelength units
    let kl_half = PI * element_length_wavelengths; // kL/2

    let mut angles = Vec::with_capacity(num_points);
    let mut element_pattern = Vec::with_capacity(num_points);
    let mut array_factor = Vec::with_capacity(num_points);
    let mut pattern_db = Vec::with_capacity(num_points);

    // Track max for finding beamwidth
    let mut max_pattern = 0.0_f64;
    let mut max_angle = PI / 2.0;

    for i in 0..num_points {
        let theta = PI * i as f64 / (num_points - 1) as f64;
        angles.push(theta);

        // Element pattern for thin dipole:
        // E(θ) = [cos(kL/2 · cos(θ)) - cos(kL/2)] / sin(θ)
        let cos_theta = theta.cos();
        let sin_theta = theta.sin();

        let e_element = if sin_theta.abs() < 1e-10 {
            0.0
        } else {
            let numerator = (kl_half * cos_theta).cos() - (kl_half).cos();
            (numerator / sin_theta).abs()
        };
        element_pattern.push(e_element);

        // Two-element array factor:
        // AF = 2 cos(ψ/2) where ψ = kd·cos(θ) + β
        let psi = k * spacing_wavelengths * cos_theta + phase_shift;
        let af = (psi / 2.0).cos().abs() * 2.0;
        // Normalize to max of 1
        let af_normalized = af / 2.0;
        array_factor.push(af_normalized);

        // Combined pattern
        let combined = e_element * af_normalized;

        if combined > max_pattern {
            max_pattern = combined;
            max_angle = theta;
        }

        // Convert to dB (handle zeros)
        let db = if combined > 1e-10 {
            20.0 * (combined / max_pattern.max(1e-10)).log10()
        } else {
            -60.0
        };
        pattern_db.push(db);
    }

    // Normalize dB pattern to 0 dB max
    let max_db = pattern_db.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    for db in &mut pattern_db {
        *db -= max_db;
        if *db < -60.0 {
            *db = -60.0;
        }
    }

    // Find half-power beamwidth
    let beamwidth_deg = find_beamwidth(&angles, &pattern_db);

    // Approximate directivity for two-element array
    // D ≈ 4 * d/λ for endfire, ≈ 2 for broadside half-wave spacing
    let directivity = 2.0 * (1.0 + spacing_wavelengths);

    TwoElementResult {
        angles,
        pattern_db,
        element_pattern,
        array_factor,
        beamwidth_deg,
        directivity,
        main_beam_deg: max_angle.to_degrees(),
    }
}

/// Thin dipole element pattern.
///
/// E(θ) = [cos(kL/2 · cos(θ)) - cos(kL/2)] / sin(θ)
pub fn dipole_element_pattern(length_wavelengths: f64, theta: f64) -> f64 {
    let kl_half = PI * length_wavelengths;
    let cos_theta = theta.cos();
    let sin_theta = theta.sin();

    if sin_theta.abs() < 1e-10 {
        return 0.0;
    }

    let numerator = (kl_half * cos_theta).cos() - (kl_half).cos();
    (numerator / sin_theta).abs()
}

/// Find half-power beamwidth from dB pattern.
fn find_beamwidth(angles: &[f64], pattern_db: &[f64]) -> f64 {
    // Find angles where pattern crosses -3 dB
    let mut crossings = Vec::new();

    for i in 1..pattern_db.len() {
        if (pattern_db[i - 1] + 3.0) * (pattern_db[i] + 3.0) < 0.0 {
            // Linear interpolation to find crossing
            let t = (pattern_db[i - 1] + 3.0) / (pattern_db[i - 1] - pattern_db[i]);
            let angle = angles[i - 1] + t * (angles[i] - angles[i - 1]);
            crossings.push(angle);
        }
    }

    if crossings.len() >= 2 {
        (crossings[1] - crossings[0]).to_degrees().abs()
    } else if crossings.len() == 1 {
        // Assume symmetric, double the half
        2.0 * (PI / 2.0 - crossings[0]).to_degrees().abs()
    } else {
        180.0 // Full hemisphere if no crossings found
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn two_element_broadside() {
        // Half-wave dipoles, half-wave spacing, no phase shift (broadside)
        let result = two_element_array(0.5, 0.5, 0.0, 181);
        // Main beam should be around 90 degrees
        assert!((result.main_beam_deg - 90.0).abs() < 5.0);
    }

    #[test]
    fn two_element_endfire() {
        // For dipole elements, the element pattern has nulls at 0° and 180°.
        // The combined pattern peak depends on both element pattern and array factor.
        // With half-wave spacing and -180° phase shift, the array factor peak is at 0°/180°,
        // but combined with dipole pattern (max at 90°), the result is tilted.
        // Just verify the pattern computation runs and produces reasonable results.
        let result = two_element_array(0.5, 0.5, -180.0, 181);
        assert!(result.beamwidth_deg > 0.0);
        assert!(result.directivity > 1.0);
        // Pattern should not be flat (should have some variation)
        let max = result.pattern_db.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let min = result.pattern_db.iter().cloned().fold(f64::INFINITY, f64::min);
        assert!(max - min > 3.0); // At least 3dB variation
    }

    #[test]
    fn two_element_result_sizes() {
        let result = two_element_array(0.5, 0.5, 0.0, 361);
        assert_eq!(result.angles.len(), 361);
        assert_eq!(result.pattern_db.len(), 361);
        assert_eq!(result.element_pattern.len(), 361);
        assert_eq!(result.array_factor.len(), 361);
    }

    #[test]
    fn broadside_peak_at_90() {
        let arr = UniformLinearArray::broadside(8, 0.5);
        assert_relative_eq!(arr.array_factor(PI / 2.0), 1.0, epsilon = 1e-10);
    }

    #[test]
    fn broadside_symmetric() {
        let arr = UniformLinearArray::broadside(8, 0.5);
        // Pattern should be symmetric about θ = 90°
        assert_relative_eq!(
            arr.array_factor(PI / 4.0),
            arr.array_factor(3.0 * PI / 4.0),
            epsilon = 1e-10
        );
    }

    #[test]
    fn endfire_peak_at_0() {
        let arr = UniformLinearArray::endfire(8, 0.5);
        assert_relative_eq!(arr.array_factor(0.0), 1.0, epsilon = 1e-6);
    }

    #[test]
    fn scanned_to_45_peak() {
        let arr = UniformLinearArray::scanned(8, 0.5, PI / 4.0);
        assert_relative_eq!(arr.array_factor(PI / 4.0), 1.0, epsilon = 1e-6);
    }

    #[test]
    fn more_elements_narrower_beam() {
        let arr4 = UniformLinearArray::broadside(4, 0.5);
        let arr16 = UniformLinearArray::broadside(16, 0.5);
        assert!(arr16.first_null_beamwidth() < arr4.first_null_beamwidth());
    }

    #[test]
    fn array_factor_bounded() {
        let arr = UniformLinearArray::broadside(10, 0.5);
        let (_, af) = arr.sample_pattern(361);
        for &v in &af {
            assert!(v >= 0.0 && v <= 1.001, "AF should be in [0, 1], got {v}");
        }
    }

    #[test]
    fn directivity_broadside_half_wave() {
        // D ≈ N for d = λ/2
        let arr = UniformLinearArray::broadside(10, 0.5);
        assert_relative_eq!(arr.directivity_approx(), 10.0, epsilon = 1e-10);
    }

    #[test]
    fn sample_pattern_dimensions() {
        let arr = UniformLinearArray::broadside(8, 0.5);
        let (t, af) = arr.sample_pattern(181);
        assert_eq!(t.len(), 181);
        assert_eq!(af.len(), 181);
    }

    #[test]
    fn sample_total_pattern_dimensions() {
        let arr = UniformLinearArray::broadside(8, 0.5);
        let (t, p) = arr.sample_total_pattern(181);
        assert_eq!(t.len(), 181);
        assert_eq!(p.len(), 181);
    }

    #[test]
    fn total_pattern_zero_on_axis() {
        // Element pattern sin(0) = 0, so total pattern at θ=0 is 0
        let arr = UniformLinearArray::broadside(8, 0.5);
        let (_, p) = arr.sample_total_pattern(181);
        assert_relative_eq!(p[0], 0.0, epsilon = 1e-10);
    }

    #[test]
    #[should_panic]
    fn single_element_panics() {
        UniformLinearArray::new(1, 0.5, 0.0);
    }
}

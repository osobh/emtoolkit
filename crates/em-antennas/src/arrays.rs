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

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

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

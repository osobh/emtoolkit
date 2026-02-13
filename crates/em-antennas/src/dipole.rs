//! Dipole antenna radiation patterns and parameters.
//!
//! Hertzian (infinitesimal) dipole and half-wave dipole.

use em_core::constants::C_0;
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

/// Hertzian (infinitesimal) dipole antenna along the z-axis.
///
/// Far-field pattern: E_θ ∝ sin(θ)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct HertzianDipole {
    /// Dipole length (m) — must be << λ
    pub length: f64,
    /// Current amplitude (A)
    pub current: f64,
    /// Operating frequency (Hz)
    pub frequency: f64,
}

impl HertzianDipole {
    pub fn new(length: f64, current: f64, frequency: f64) -> Self {
        Self {
            length,
            current,
            frequency,
        }
    }

    /// Wavelength λ = c/f
    pub fn wavelength(&self) -> f64 {
        C_0 / self.frequency
    }

    /// Wave number k = 2π/λ
    pub fn k(&self) -> f64 {
        2.0 * PI * self.frequency / C_0
    }

    /// Normalized radiation pattern F(θ) = sin(θ).
    pub fn pattern(&self, theta: f64) -> f64 {
        theta.sin().abs()
    }

    /// Radiation resistance: R_rad = 80π²(dl/λ)²
    pub fn radiation_resistance(&self) -> f64 {
        let ratio = self.length / self.wavelength();
        80.0 * PI * PI * ratio * ratio
    }

    /// Directivity: D = 1.5 (3/2) for Hertzian dipole.
    pub fn directivity(&self) -> f64 {
        1.5
    }

    /// Directivity in dBi.
    pub fn directivity_dbi(&self) -> f64 {
        10.0 * 1.5_f64.log10()
    }

    /// Maximum effective area: A_e = λ²·D/(4π)
    pub fn effective_area(&self) -> f64 {
        let lambda = self.wavelength();
        lambda * lambda * self.directivity() / (4.0 * PI)
    }

    /// Radiated power: P_rad = ½ I² R_rad
    pub fn radiated_power(&self) -> f64 {
        0.5 * self.current * self.current * self.radiation_resistance()
    }

    /// Sample radiation pattern in E-plane (φ=0, vary θ).
    pub fn sample_pattern(&self, num_points: usize) -> (Vec<f64>, Vec<f64>) {
        let dtheta = PI / (num_points - 1) as f64;
        let thetas: Vec<f64> = (0..num_points).map(|i| i as f64 * dtheta).collect();
        let pattern: Vec<f64> = thetas.iter().map(|&t| self.pattern(t)).collect();
        (thetas, pattern)
    }
}

/// Half-wave dipole antenna along the z-axis.
///
/// Far-field pattern: F(θ) = cos(π/2 · cos θ) / sin θ
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct HalfWaveDipole {
    /// Operating frequency (Hz)
    pub frequency: f64,
    /// Current amplitude at feed point (A)
    pub current: f64,
}

impl HalfWaveDipole {
    pub fn new(frequency: f64, current: f64) -> Self {
        Self { frequency, current }
    }

    pub fn wavelength(&self) -> f64 {
        C_0 / self.frequency
    }

    /// Physical length = λ/2
    pub fn length(&self) -> f64 {
        self.wavelength() / 2.0
    }

    /// Normalized radiation pattern.
    ///
    /// F(θ) = cos(π/2 · cos θ) / sin θ
    pub fn pattern(&self, theta: f64) -> f64 {
        let sin_t = theta.sin();
        if sin_t.abs() < 1e-15 {
            return 0.0;
        }
        ((PI / 2.0 * theta.cos()).cos() / sin_t).abs()
    }

    /// Radiation resistance ≈ 73.1 Ω
    pub fn radiation_resistance(&self) -> f64 {
        73.1
    }

    /// Input impedance ≈ 73.1 + j42.5 Ω (for exact λ/2)
    pub fn input_impedance(&self) -> (f64, f64) {
        (73.1, 42.5)
    }

    /// Directivity ≈ 1.643 (2.15 dBi)
    pub fn directivity(&self) -> f64 {
        1.643
    }

    /// Directivity in dBi.
    pub fn directivity_dbi(&self) -> f64 {
        10.0 * self.directivity().log10()
    }

    /// Maximum effective area.
    pub fn effective_area(&self) -> f64 {
        let lambda = self.wavelength();
        lambda * lambda * self.directivity() / (4.0 * PI)
    }

    /// Radiated power.
    pub fn radiated_power(&self) -> f64 {
        0.5 * self.current * self.current * self.radiation_resistance()
    }

    /// Sample radiation pattern.
    pub fn sample_pattern(&self, num_points: usize) -> (Vec<f64>, Vec<f64>) {
        let dtheta = PI / (num_points - 1) as f64;
        let thetas: Vec<f64> = (0..num_points).map(|i| i as f64 * dtheta).collect();
        let pattern: Vec<f64> = thetas.iter().map(|&t| self.pattern(t)).collect();
        (thetas, pattern)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    // Hertzian dipole

    #[test]
    fn hertzian_pattern_max_at_90() {
        let d = HertzianDipole::new(0.01, 1.0, 1e9);
        assert_relative_eq!(d.pattern(PI / 2.0), 1.0, epsilon = 1e-12);
    }

    #[test]
    fn hertzian_pattern_zero_on_axis() {
        let d = HertzianDipole::new(0.01, 1.0, 1e9);
        assert_relative_eq!(d.pattern(0.0), 0.0, epsilon = 1e-12);
        assert_relative_eq!(d.pattern(PI), 0.0, epsilon = 1e-12);
    }

    #[test]
    fn hertzian_directivity() {
        let d = HertzianDipole::new(0.01, 1.0, 1e9);
        assert_relative_eq!(d.directivity(), 1.5, epsilon = 1e-12);
    }

    #[test]
    fn hertzian_radiation_resistance() {
        // For dl/λ = 0.01: R = 80π²(0.01)² ≈ 0.0789 Ω
        let lambda = C_0 / 1e9;
        let dl = 0.01 * lambda;
        let d = HertzianDipole::new(dl, 1.0, 1e9);
        let expected = 80.0 * PI * PI * 0.01 * 0.01;
        assert_relative_eq!(d.radiation_resistance(), expected, max_relative = 1e-6);
    }

    #[test]
    fn hertzian_directivity_dbi() {
        let d = HertzianDipole::new(0.01, 1.0, 1e9);
        assert_relative_eq!(d.directivity_dbi(), 10.0 * 1.5_f64.log10(), max_relative = 1e-10);
    }

    #[test]
    fn hertzian_sample_pattern_dims() {
        let d = HertzianDipole::new(0.01, 1.0, 1e9);
        let (t, p) = d.sample_pattern(181);
        assert_eq!(t.len(), 181);
        assert_eq!(p.len(), 181);
    }

    // Half-wave dipole

    #[test]
    fn halfwave_pattern_max_at_90() {
        let d = HalfWaveDipole::new(1e9, 1.0);
        assert_relative_eq!(d.pattern(PI / 2.0), 1.0, epsilon = 1e-10);
    }

    #[test]
    fn halfwave_pattern_zero_on_axis() {
        let d = HalfWaveDipole::new(1e9, 1.0);
        assert_relative_eq!(d.pattern(0.0), 0.0, epsilon = 1e-10);
    }

    #[test]
    fn halfwave_radiation_resistance() {
        let d = HalfWaveDipole::new(1e9, 1.0);
        assert_relative_eq!(d.radiation_resistance(), 73.1, epsilon = 0.1);
    }

    #[test]
    fn halfwave_directivity() {
        let d = HalfWaveDipole::new(1e9, 1.0);
        assert_relative_eq!(d.directivity(), 1.643, epsilon = 0.001);
    }

    #[test]
    fn halfwave_directivity_dbi() {
        let d = HalfWaveDipole::new(1e9, 1.0);
        assert_relative_eq!(d.directivity_dbi(), 2.15, max_relative = 0.01);
    }

    #[test]
    fn halfwave_length_is_half_lambda() {
        let d = HalfWaveDipole::new(1e9, 1.0);
        assert_relative_eq!(d.length(), d.wavelength() / 2.0, epsilon = 1e-12);
    }

    #[test]
    fn halfwave_sample_pattern_dims() {
        let d = HalfWaveDipole::new(1e9, 1.0);
        let (t, p) = d.sample_pattern(91);
        assert_eq!(t.len(), 91);
        assert_eq!(p.len(), 91);
    }
}

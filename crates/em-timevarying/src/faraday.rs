//! Faraday's law of electromagnetic induction.
//!
//! EMF = -dΦ_B/dt where Φ_B = ∫∫ B · dA
//!
//! Covers: stationary loops in time-varying B, moving conductors in static B,
//! and transformers/generators.

use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

/// Magnetic flux through a surface.
///
/// Φ = B · A · cos(θ)
pub fn magnetic_flux(b_magnitude: f64, area: f64, angle: f64) -> f64 {
    b_magnitude * area * angle.cos()
}

/// EMF from a time-varying sinusoidal B-field through a stationary loop.
///
/// If B(t) = B₀ cos(ωt + φ), then Φ = B₀·A·cos(ωt + φ)
/// and EMF = B₀·A·ω·sin(ωt + φ)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SinusoidalFlux {
    /// Peak B-field magnitude (T)
    pub b_peak: f64,
    /// Loop area (m²)
    pub area: f64,
    /// Angular frequency (rad/s)
    pub omega: f64,
    /// Phase offset (rad)
    pub phase: f64,
}

impl SinusoidalFlux {
    pub fn new(b_peak: f64, area: f64, omega: f64) -> Self {
        Self {
            b_peak,
            area,
            omega,
            phase: 0.0,
        }
    }

    pub fn with_phase(mut self, phase: f64) -> Self {
        self.phase = phase;
        self
    }

    /// Magnetic flux at time t.
    pub fn flux_at(&self, t: f64) -> f64 {
        self.b_peak * self.area * (self.omega * t + self.phase).cos()
    }

    /// Induced EMF at time t: EMF = -dΦ/dt = B₀·A·ω·sin(ωt + φ)
    pub fn emf_at(&self, t: f64) -> f64 {
        self.b_peak * self.area * self.omega * (self.omega * t + self.phase).sin()
    }

    /// Peak EMF magnitude.
    pub fn emf_peak(&self) -> f64 {
        self.b_peak * self.area * self.omega
    }

    /// Sample flux and EMF over time for visualization.
    pub fn sample(&self, t_end: f64, num_points: usize) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
        assert!(num_points >= 2);
        let dt = t_end / (num_points - 1) as f64;
        let times: Vec<f64> = (0..num_points).map(|i| i as f64 * dt).collect();
        let flux: Vec<f64> = times.iter().map(|&t| self.flux_at(t)).collect();
        let emf: Vec<f64> = times.iter().map(|&t| self.emf_at(t)).collect();
        (times, flux, emf)
    }
}

/// A simple AC generator (rotating loop in uniform B-field).
///
/// EMF = N·B·A·ω·sin(ωt)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct AcGenerator {
    /// Number of turns
    pub turns: usize,
    /// B-field magnitude (T)
    pub b_field: f64,
    /// Loop area (m²)
    pub area: f64,
    /// Rotation speed (rad/s)
    pub omega: f64,
}

impl AcGenerator {
    pub fn new(turns: usize, b_field: f64, area: f64, omega: f64) -> Self {
        Self {
            turns,
            b_field,
            area,
            omega,
        }
    }

    /// From RPM rotation speed.
    pub fn from_rpm(turns: usize, b_field: f64, area: f64, rpm: f64) -> Self {
        Self::new(turns, b_field, area, rpm * 2.0 * PI / 60.0)
    }

    /// Peak EMF.
    pub fn emf_peak(&self) -> f64 {
        self.turns as f64 * self.b_field * self.area * self.omega
    }

    /// EMF at time t.
    pub fn emf_at(&self, t: f64) -> f64 {
        self.emf_peak() * (self.omega * t).sin()
    }

    /// RMS voltage.
    pub fn vrms(&self) -> f64 {
        self.emf_peak() / 2.0_f64.sqrt()
    }

    /// Frequency in Hz.
    pub fn frequency(&self) -> f64 {
        self.omega / (2.0 * PI)
    }

    /// Period in seconds.
    pub fn period(&self) -> f64 {
        1.0 / self.frequency()
    }
}

/// A simple transformer (ideal).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct IdealTransformer {
    /// Primary turns
    pub n_primary: usize,
    /// Secondary turns
    pub n_secondary: usize,
}

impl IdealTransformer {
    pub fn new(n_primary: usize, n_secondary: usize) -> Self {
        Self {
            n_primary,
            n_secondary,
        }
    }

    /// Turns ratio: n = N₂/N₁
    pub fn turns_ratio(&self) -> f64 {
        self.n_secondary as f64 / self.n_primary as f64
    }

    /// Secondary voltage from primary voltage.
    pub fn v_secondary(&self, v_primary: f64) -> f64 {
        v_primary * self.turns_ratio()
    }

    /// Secondary current from primary current (ideal: P₁ = P₂).
    pub fn i_secondary(&self, i_primary: f64) -> f64 {
        i_primary / self.turns_ratio()
    }

    /// Impedance transformation: Z₂' = Z₂/n²
    pub fn impedance_reflected(&self, z_secondary: f64) -> f64 {
        z_secondary / (self.turns_ratio() * self.turns_ratio())
    }

    /// Is step-up transformer?
    pub fn is_step_up(&self) -> bool {
        self.n_secondary > self.n_primary
    }
}

/// Motional EMF for a conductor moving in a magnetic field.
///
/// EMF = ∫ (v × B) · dl
///
/// For a straight conductor of length L moving at velocity v perpendicular to B:
/// EMF = v · B · L
pub fn motional_emf(velocity: f64, b_field: f64, length: f64) -> f64 {
    velocity * b_field * length
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    // ================================================================
    // Magnetic flux
    // ================================================================

    #[test]
    fn flux_perpendicular() {
        assert_relative_eq!(magnetic_flux(1.0, 0.01, 0.0), 0.01, epsilon = 1e-12);
    }

    #[test]
    fn flux_parallel_is_zero() {
        assert_relative_eq!(magnetic_flux(1.0, 0.01, PI / 2.0), 0.0, epsilon = 1e-15);
    }

    // ================================================================
    // Sinusoidal flux
    // ================================================================

    #[test]
    fn sinusoidal_flux_at_t0() {
        let sf = SinusoidalFlux::new(0.5, 0.01, 100.0);
        assert_relative_eq!(sf.flux_at(0.0), 0.005, epsilon = 1e-12);
    }

    #[test]
    fn sinusoidal_emf_at_t0_is_zero() {
        let sf = SinusoidalFlux::new(0.5, 0.01, 100.0);
        assert_relative_eq!(sf.emf_at(0.0), 0.0, epsilon = 1e-12);
    }

    #[test]
    fn sinusoidal_emf_peak() {
        let sf = SinusoidalFlux::new(0.5, 0.01, 100.0);
        assert_relative_eq!(sf.emf_peak(), 0.5, epsilon = 1e-12);
    }

    #[test]
    fn sinusoidal_flux_and_emf_90_degrees_apart() {
        let sf = SinusoidalFlux::new(0.5, 0.01, 100.0);
        // When flux is max (t=0), EMF is zero; when flux is zero, EMF is max
        let quarter_period = PI / (2.0 * 100.0);
        assert_relative_eq!(sf.flux_at(quarter_period), 0.0, epsilon = 1e-12);
        assert_relative_eq!(sf.emf_at(quarter_period), sf.emf_peak(), max_relative = 1e-10);
    }

    #[test]
    fn sinusoidal_sample_dimensions() {
        let sf = SinusoidalFlux::new(0.5, 0.01, 100.0);
        let (ts, flux, emf) = sf.sample(1.0, 100);
        assert_eq!(ts.len(), 100);
        assert_eq!(flux.len(), 100);
        assert_eq!(emf.len(), 100);
    }

    // ================================================================
    // AC Generator
    // ================================================================

    #[test]
    fn generator_emf_peak() {
        let g = AcGenerator::new(100, 0.5, 0.04, 120.0 * PI);
        let expected = 100.0 * 0.5 * 0.04 * 120.0 * PI;
        assert_relative_eq!(g.emf_peak(), expected, max_relative = 1e-10);
    }

    #[test]
    fn generator_from_rpm() {
        let g = AcGenerator::from_rpm(1, 1.0, 1.0, 60.0);
        assert_relative_eq!(g.omega, 2.0 * PI, max_relative = 1e-10);
    }

    #[test]
    fn generator_vrms() {
        let g = AcGenerator::new(1, 1.0, 1.0, 2.0 * PI);
        assert_relative_eq!(g.vrms(), g.emf_peak() / 2.0_f64.sqrt(), max_relative = 1e-10);
    }

    #[test]
    fn generator_frequency() {
        let g = AcGenerator::new(1, 1.0, 1.0, 2.0 * PI * 60.0);
        assert_relative_eq!(g.frequency(), 60.0, max_relative = 1e-10);
    }

    #[test]
    fn generator_period() {
        let g = AcGenerator::new(1, 1.0, 1.0, 2.0 * PI * 50.0);
        assert_relative_eq!(g.period(), 0.02, max_relative = 1e-10);
    }

    // ================================================================
    // Transformer
    // ================================================================

    #[test]
    fn transformer_step_up() {
        let t = IdealTransformer::new(100, 500);
        assert!(t.is_step_up());
        assert_relative_eq!(t.turns_ratio(), 5.0, epsilon = 1e-12);
    }

    #[test]
    fn transformer_voltage() {
        let t = IdealTransformer::new(100, 500);
        assert_relative_eq!(t.v_secondary(120.0), 600.0, epsilon = 1e-12);
    }

    #[test]
    fn transformer_current_conservation() {
        let t = IdealTransformer::new(100, 500);
        // Power conservation: V₁I₁ = V₂I₂
        let v1 = 120.0;
        let i1 = 10.0;
        let v2 = t.v_secondary(v1);
        let i2 = t.i_secondary(i1);
        assert_relative_eq!(v1 * i1, v2 * i2, max_relative = 1e-10);
    }

    #[test]
    fn transformer_impedance_reflection() {
        let t = IdealTransformer::new(100, 200);
        // Z reflected = Z₂/n²
        let z2 = 100.0;
        let z_ref = t.impedance_reflected(z2);
        assert_relative_eq!(z_ref, 25.0, epsilon = 1e-12);
    }

    // ================================================================
    // Motional EMF
    // ================================================================

    #[test]
    fn motional_emf_basic() {
        assert_relative_eq!(motional_emf(10.0, 0.5, 0.1), 0.5, epsilon = 1e-12);
    }

    #[test]
    fn motional_emf_proportional_to_velocity() {
        let e1 = motional_emf(1.0, 1.0, 1.0);
        let e2 = motional_emf(3.0, 1.0, 1.0);
        assert_relative_eq!(e2 / e1, 3.0, max_relative = 1e-10);
    }
}

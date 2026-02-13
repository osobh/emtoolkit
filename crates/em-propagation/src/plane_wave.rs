//! Plane wave propagation in lossless and lossy media.
//!
//! Covers propagation constant γ = α + jβ, intrinsic impedance η,
//! phase velocity, wavelength, and skin depth.

use em_core::constants::{C_0, EPSILON_0, MU_0};
use num_complex::Complex64;
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

/// Material properties for wave propagation.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Medium {
    /// Relative permittivity εᵣ
    pub epsilon_r: f64,
    /// Relative permeability μᵣ
    pub mu_r: f64,
    /// Conductivity σ (S/m)
    pub conductivity: f64,
}

impl Medium {
    /// Lossless dielectric.
    pub fn lossless(epsilon_r: f64) -> Self {
        Self {
            epsilon_r,
            mu_r: 1.0,
            conductivity: 0.0,
        }
    }

    /// Lossy medium with conductivity.
    pub fn lossy(epsilon_r: f64, conductivity: f64) -> Self {
        Self {
            epsilon_r,
            mu_r: 1.0,
            conductivity,
        }
    }

    /// Free space.
    pub fn free_space() -> Self {
        Self::lossless(1.0)
    }

    /// Good conductor (σ >> ωε).
    pub fn conductor(conductivity: f64) -> Self {
        Self {
            epsilon_r: 1.0,
            mu_r: 1.0,
            conductivity,
        }
    }

    /// Absolute permittivity.
    pub fn epsilon(&self) -> f64 {
        EPSILON_0 * self.epsilon_r
    }

    /// Absolute permeability.
    pub fn mu(&self) -> f64 {
        MU_0 * self.mu_r
    }

    /// Complex permittivity: ε_c = ε' - jε'' = ε(1 - jσ/(ωε))
    pub fn complex_permittivity(&self, omega: f64) -> Complex64 {
        let eps = self.epsilon();
        Complex64::new(eps, -self.conductivity / omega)
    }

    /// Loss tangent: tan(δ) = σ/(ωε)
    pub fn loss_tangent(&self, omega: f64) -> f64 {
        self.conductivity / (omega * self.epsilon())
    }

    /// Is this a good conductor at frequency f? (σ >> ωε)
    pub fn is_good_conductor(&self, omega: f64) -> bool {
        self.loss_tangent(omega) > 100.0
    }

    /// Is this a low-loss dielectric? (σ << ωε)
    pub fn is_low_loss(&self, omega: f64) -> bool {
        self.loss_tangent(omega) < 0.01
    }

    /// Propagation constant γ = α + jβ = jω√(μ·ε_c)
    pub fn propagation_constant(&self, omega: f64) -> Complex64 {
        let mu = Complex64::new(self.mu(), 0.0);
        let eps_c = self.complex_permittivity(omega);
        let jw = Complex64::new(0.0, omega);
        (jw * jw * mu * eps_c).sqrt() // γ = √(-ω²με_c) but we want jω√(με_c)
        // Actually: γ² = jωμ(σ + jωε) = -ω²με + jωμσ
        // Let's compute correctly:
        // γ = sqrt(jωμ(σ + jωε))
    }

    /// Attenuation constant α (Np/m).
    pub fn alpha(&self, omega: f64) -> f64 {
        self.propagation_constant(omega).re
    }

    /// Phase constant β (rad/m).
    pub fn beta(&self, omega: f64) -> f64 {
        self.propagation_constant(omega).im
    }

    /// Intrinsic impedance η = √(jωμ/(σ + jωε))
    pub fn intrinsic_impedance(&self, omega: f64) -> Complex64 {
        let jwmu = Complex64::new(0.0, omega * self.mu());
        let sigma_plus_jwe = Complex64::new(self.conductivity, omega * self.epsilon());
        (jwmu / sigma_plus_jwe).sqrt()
    }

    /// Phase velocity: v_p = ω/β
    pub fn phase_velocity(&self, omega: f64) -> f64 {
        let b = self.beta(omega);
        if b.abs() < 1e-30 { C_0 } else { omega / b }
    }

    /// Wavelength in the medium: λ = 2π/β
    pub fn wavelength(&self, omega: f64) -> f64 {
        let b = self.beta(omega);
        if b.abs() < 1e-30 {
            C_0 * 2.0 * PI / omega
        } else {
            2.0 * PI / b
        }
    }

    /// Skin depth: δ = 1/α
    pub fn skin_depth(&self, omega: f64) -> f64 {
        let a = self.alpha(omega);
        if a.abs() < 1e-30 { f64::INFINITY } else { 1.0 / a }
    }
}

/// Compute plane wave E and H field magnitudes at distance z from source.
///
/// E(z) = E₀ · e^(-αz) · cos(ωt - βz)
pub fn e_field_magnitude(e0: f64, alpha: f64, z: f64) -> f64 {
    e0 * (-alpha * z).exp()
}

/// Time-averaged Poynting vector (power density) at distance z.
///
/// S_avg = |E₀|² e^(-2αz) / (2|η|) · cos(θ_η)
pub fn poynting_average(e0: f64, alpha: f64, eta: Complex64, z: f64) -> f64 {
    let e_z = e0 * (-alpha * z).exp();
    e_z * e_z * eta.re / (2.0 * eta.norm_sqr())
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn free_space_phase_velocity() {
        let m = Medium::free_space();
        let omega = 2.0 * PI * 1e9;
        assert_relative_eq!(m.phase_velocity(omega), C_0, max_relative = 0.01);
    }

    #[test]
    fn free_space_impedance_377_ohms() {
        let m = Medium::free_space();
        let omega = 2.0 * PI * 1e9;
        let eta = m.intrinsic_impedance(omega);
        let eta_0 = (MU_0 / EPSILON_0).sqrt(); // ≈ 377 Ω
        assert_relative_eq!(eta.re, eta_0, max_relative = 0.01);
        assert_relative_eq!(eta.im, 0.0, epsilon = 1.0);
    }

    #[test]
    fn free_space_no_attenuation() {
        let m = Medium::free_space();
        let omega = 2.0 * PI * 1e9;
        assert_relative_eq!(m.alpha(omega), 0.0, epsilon = 1e-10);
    }

    #[test]
    fn lossless_dielectric_slower() {
        let m = Medium::lossless(4.0);
        let omega = 2.0 * PI * 1e9;
        assert_relative_eq!(m.phase_velocity(omega), C_0 / 2.0, max_relative = 0.01);
    }

    #[test]
    fn lossless_wavelength_shorter() {
        let m = Medium::lossless(4.0);
        let omega = 2.0 * PI * 1e9;
        let lambda_0 = C_0 / 1e9;
        assert_relative_eq!(m.wavelength(omega), lambda_0 / 2.0, max_relative = 0.01);
    }

    #[test]
    fn lossy_medium_has_attenuation() {
        let m = Medium::lossy(1.0, 1.0); // moderate conductor
        let omega = 2.0 * PI * 1e6;
        assert!(m.alpha(omega) > 0.0);
    }

    #[test]
    fn good_conductor_skin_depth() {
        // δ = √(2/(ωμσ)) for good conductors
        let sigma = 5.8e7; // copper
        let m = Medium::conductor(sigma);
        let f = 1e6;
        let omega = 2.0 * PI * f;
        let delta = m.skin_depth(omega);
        let expected = (2.0 / (omega * MU_0 * sigma)).sqrt();
        assert_relative_eq!(delta, expected, max_relative = 0.05);
    }

    #[test]
    fn copper_is_good_conductor_at_1mhz() {
        let m = Medium::conductor(5.8e7);
        let omega = 2.0 * PI * 1e6;
        assert!(m.is_good_conductor(omega));
    }

    #[test]
    fn glass_is_low_loss_dielectric() {
        let m = Medium::lossy(4.0, 1e-10);
        let omega = 2.0 * PI * 1e9;
        assert!(m.is_low_loss(omega));
    }

    #[test]
    fn loss_tangent_value() {
        let m = Medium::lossy(1.0, 1.0);
        let omega = 2.0 * PI * 1e9;
        let expected = 1.0 / (omega * EPSILON_0);
        assert_relative_eq!(m.loss_tangent(omega), expected, max_relative = 1e-10);
    }

    #[test]
    fn e_field_decays_exponentially() {
        let e1 = e_field_magnitude(1.0, 0.1, 10.0);
        let e2 = e_field_magnitude(1.0, 0.1, 20.0);
        assert_relative_eq!(e2 / e1, (-1.0_f64).exp(), max_relative = 1e-10);
    }

    #[test]
    fn lossless_no_decay() {
        assert_relative_eq!(e_field_magnitude(5.0, 0.0, 100.0), 5.0, epsilon = 1e-12);
    }

    #[test]
    fn poynting_positive_in_free_space() {
        let m = Medium::free_space();
        let omega = 2.0 * PI * 1e9;
        let eta = m.intrinsic_impedance(omega);
        let s = poynting_average(1.0, 0.0, eta, 0.0);
        assert!(s > 0.0);
    }
}

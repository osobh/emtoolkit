//! Fresnel coefficients and Snell's law for wave reflection/transmission.
//!
//! Handles normal and oblique incidence at planar boundaries between
//! lossless dielectric media.

use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

/// Normal incidence reflection and transmission coefficients.
///
/// Γ = (η₂ - η₁)/(η₂ + η₁)
/// τ = 2η₂/(η₂ + η₁)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct NormalIncidence {
    /// Intrinsic impedance of medium 1 (Ω)
    pub eta1: f64,
    /// Intrinsic impedance of medium 2 (Ω)
    pub eta2: f64,
}

impl NormalIncidence {
    pub fn new(eta1: f64, eta2: f64) -> Self {
        Self { eta1, eta2 }
    }

    /// From relative permittivities (lossless, μᵣ=1).
    /// η = η₀/√εᵣ
    pub fn from_epsilon_r(er1: f64, er2: f64) -> Self {
        let eta0 = 377.0; // approximate
        Self {
            eta1: eta0 / er1.sqrt(),
            eta2: eta0 / er2.sqrt(),
        }
    }

    /// Reflection coefficient Γ.
    pub fn gamma(&self) -> f64 {
        (self.eta2 - self.eta1) / (self.eta2 + self.eta1)
    }

    /// Transmission coefficient τ.
    pub fn tau(&self) -> f64 {
        2.0 * self.eta2 / (self.eta2 + self.eta1)
    }

    /// Power reflectance |Γ|².
    pub fn reflectance(&self) -> f64 {
        self.gamma() * self.gamma()
    }

    /// Power transmittance 1 - |Γ|².
    pub fn transmittance(&self) -> f64 {
        1.0 - self.reflectance()
    }
}

/// Oblique incidence at a planar boundary between two lossless dielectrics.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ObliqueIncidence {
    /// Relative permittivity of medium 1
    pub er1: f64,
    /// Relative permittivity of medium 2
    pub er2: f64,
    /// Angle of incidence (radians)
    pub theta_i: f64,
}

impl ObliqueIncidence {
    pub fn new(er1: f64, er2: f64, theta_i: f64) -> Self {
        Self { er1, er2, theta_i }
    }

    /// Refractive indices.
    pub fn n1(&self) -> f64 {
        self.er1.sqrt()
    }

    pub fn n2(&self) -> f64 {
        self.er2.sqrt()
    }

    /// Snell's law: sin(θ_t) = (n₁/n₂)·sin(θ_i)
    ///
    /// Returns None if total internal reflection occurs.
    pub fn theta_t(&self) -> Option<f64> {
        let sin_t = self.n1() / self.n2() * self.theta_i.sin();
        if sin_t.abs() > 1.0 {
            None // TIR
        } else {
            Some(sin_t.asin())
        }
    }

    /// Critical angle for total internal reflection.
    ///
    /// θ_c = arcsin(n₂/n₁), only exists when n₁ > n₂.
    pub fn critical_angle(&self) -> Option<f64> {
        if self.n1() > self.n2() {
            Some((self.n2() / self.n1()).asin())
        } else {
            None
        }
    }

    /// Brewster angle (for parallel/TM polarization).
    ///
    /// θ_B = arctan(n₂/n₁)
    pub fn brewster_angle(&self) -> f64 {
        (self.n2() / self.n1()).atan()
    }

    /// Is total internal reflection occurring?
    pub fn is_tir(&self) -> bool {
        self.theta_t().is_none()
    }

    /// Perpendicular (TE/s) polarization reflection coefficient.
    ///
    /// Γ_⊥ = (η₂cosθᵢ - η₁cosθₜ)/(η₂cosθᵢ + η₁cosθₜ)
    pub fn gamma_perp(&self) -> Option<f64> {
        let theta_t = self.theta_t()?;
        let eta1 = 1.0 / self.n1(); // proportional
        let eta2 = 1.0 / self.n2();
        let num = eta2 * self.theta_i.cos() - eta1 * theta_t.cos();
        let den = eta2 * self.theta_i.cos() + eta1 * theta_t.cos();
        Some(num / den)
    }

    /// Parallel (TM/p) polarization reflection coefficient.
    ///
    /// Γ_∥ = (η₂cosθₜ - η₁cosθᵢ)/(η₂cosθₜ + η₁cosθᵢ)
    pub fn gamma_par(&self) -> Option<f64> {
        let theta_t = self.theta_t()?;
        let eta1 = 1.0 / self.n1();
        let eta2 = 1.0 / self.n2();
        let num = eta2 * theta_t.cos() - eta1 * self.theta_i.cos();
        let den = eta2 * theta_t.cos() + eta1 * self.theta_i.cos();
        Some(num / den)
    }

    /// Sample reflection coefficients vs angle for visualization.
    pub fn sample_vs_angle(
        er1: f64,
        er2: f64,
        num_points: usize,
    ) -> FresnelSample {
        let angles: Vec<f64> = (0..num_points)
            .map(|i| i as f64 * (PI / 2.0) / (num_points - 1) as f64)
            .collect();

        let mut gamma_perp = Vec::with_capacity(num_points);
        let mut gamma_par = Vec::with_capacity(num_points);

        for &theta in &angles {
            let oi = ObliqueIncidence::new(er1, er2, theta);
            gamma_perp.push(oi.gamma_perp().unwrap_or(1.0));
            gamma_par.push(oi.gamma_par().unwrap_or(1.0));
        }

        FresnelSample {
            angles,
            gamma_perp,
            gamma_par,
        }
    }
}

/// Sampled Fresnel coefficients for plotting.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FresnelSample {
    pub angles: Vec<f64>,
    pub gamma_perp: Vec<f64>,
    pub gamma_par: Vec<f64>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    // ================================================================
    // Normal incidence
    // ================================================================

    #[test]
    fn normal_matched_no_reflection() {
        let ni = NormalIncidence::new(377.0, 377.0);
        assert_relative_eq!(ni.gamma(), 0.0, epsilon = 1e-12);
        assert_relative_eq!(ni.tau(), 1.0, epsilon = 1e-12);
    }

    #[test]
    fn normal_power_conservation() {
        let ni = NormalIncidence::new(377.0, 200.0);
        assert_relative_eq!(ni.reflectance() + ni.transmittance(), 1.0, epsilon = 1e-12);
    }

    #[test]
    fn normal_open_circuit_gamma_plus_one() {
        // η₂ → ∞ (PEC boundary from other side)
        let ni = NormalIncidence::new(377.0, 1e10);
        assert_relative_eq!(ni.gamma(), 1.0, max_relative = 1e-6);
    }

    #[test]
    fn normal_short_circuit_gamma_minus_one() {
        // η₂ → 0 (PEC)
        let ni = NormalIncidence::new(377.0, 1e-10);
        assert_relative_eq!(ni.gamma(), -1.0, max_relative = 1e-6);
    }

    #[test]
    fn normal_from_epsilon_r() {
        let ni = NormalIncidence::from_epsilon_r(1.0, 4.0);
        // Γ = (η₀/2 - η₀)/(η₀/2 + η₀) = -1/3
        assert_relative_eq!(ni.gamma(), -1.0 / 3.0, max_relative = 1e-6);
    }

    // ================================================================
    // Oblique incidence - Snell's law
    // ================================================================

    #[test]
    fn snell_normal_incidence() {
        let oi = ObliqueIncidence::new(1.0, 4.0, 0.0);
        assert_relative_eq!(oi.theta_t().unwrap(), 0.0, epsilon = 1e-12);
    }

    #[test]
    fn snell_air_to_glass() {
        // n₁=1, n₂=1.5, θ_i=30° → sin(θ_t) = sin(30°)/1.5 = 1/3
        let oi = ObliqueIncidence::new(1.0, 2.25, PI / 6.0); // n₂ = √2.25 = 1.5
        let theta_t = oi.theta_t().unwrap();
        let expected = (1.0 / 3.0_f64).asin();
        assert_relative_eq!(theta_t, expected, max_relative = 1e-10);
    }

    #[test]
    fn snell_glass_to_air_tir() {
        // n₁=1.5, n₂=1.0, θ_i > θ_c → TIR
        let oi = ObliqueIncidence::new(2.25, 1.0, PI / 3.0); // 60° > θ_c ≈ 41.8°
        assert!(oi.is_tir());
    }

    #[test]
    fn critical_angle_glass_air() {
        let oi = ObliqueIncidence::new(2.25, 1.0, 0.0);
        let theta_c = oi.critical_angle().unwrap();
        let expected = (1.0 / 1.5_f64).asin();
        assert_relative_eq!(theta_c, expected, max_relative = 1e-10);
    }

    #[test]
    fn no_critical_angle_air_to_glass() {
        let oi = ObliqueIncidence::new(1.0, 2.25, 0.0);
        assert!(oi.critical_angle().is_none());
    }

    #[test]
    fn brewster_angle_air_glass() {
        let oi = ObliqueIncidence::new(1.0, 2.25, 0.0);
        let theta_b = oi.brewster_angle();
        let expected = (1.5_f64).atan();
        assert_relative_eq!(theta_b, expected, max_relative = 1e-10);
    }

    #[test]
    fn gamma_par_zero_at_brewster() {
        let er1 = 1.0;
        let er2 = 2.25;
        let ratio: f64 = er2 / er1;
        let theta_b = ratio.sqrt().atan();
        let oi = ObliqueIncidence::new(er1, er2, theta_b);
        assert_relative_eq!(oi.gamma_par().unwrap(), 0.0, epsilon = 1e-6);
    }

    #[test]
    fn gamma_perp_at_normal_matches_normal_incidence() {
        let er1 = 1.0;
        let er2 = 4.0;
        let oi = ObliqueIncidence::new(er1, er2, 0.0);
        let ni = NormalIncidence::from_epsilon_r(er1, er2);
        assert_relative_eq!(oi.gamma_perp().unwrap(), ni.gamma(), max_relative = 1e-6);
    }

    #[test]
    fn sample_vs_angle_dimensions() {
        let s = ObliqueIncidence::sample_vs_angle(1.0, 4.0, 50);
        assert_eq!(s.angles.len(), 50);
        assert_eq!(s.gamma_perp.len(), 50);
        assert_eq!(s.gamma_par.len(), 50);
    }
}

//! Charge-current continuity equation.
//!
//! ∇·J + ∂ρ/∂t = 0 (conservation of charge)
//!
//! Demonstrates that current flow in/out of a volume must equal
//! the rate of change of charge within that volume.

use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

/// A spherically symmetric charge distribution that decays exponentially.
///
/// ρ(r, t) = ρ₀ · e^(-t/τ) · f(r)
/// where f(r) is the spatial distribution and τ is the relaxation time.
///
/// τ = ε/σ (dielectric relaxation time)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct RelaxingCharge {
    /// Initial charge density (C/m³)
    pub rho_0: f64,
    /// Relaxation time constant: τ = ε/σ (seconds)
    pub tau: f64,
    /// Radius of charge distribution (m)
    pub radius: f64,
}

impl RelaxingCharge {
    pub fn new(rho_0: f64, tau: f64, radius: f64) -> Self {
        Self { rho_0, tau, radius }
    }

    /// From material properties: τ = ε₀εᵣ/σ
    pub fn from_material(rho_0: f64, epsilon_r: f64, conductivity: f64, radius: f64) -> Self {
        let tau = em_core::constants::EPSILON_0 * epsilon_r / conductivity;
        Self { rho_0, tau, radius }
    }

    /// Charge density at time t (uniform sphere).
    pub fn rho_at(&self, t: f64) -> f64 {
        self.rho_0 * (-t / self.tau).exp()
    }

    /// Total charge at time t: Q = ρ(t) · V
    pub fn total_charge_at(&self, t: f64) -> f64 {
        let volume = 4.0 / 3.0 * PI * self.radius.powi(3);
        self.rho_at(t) * volume
    }

    /// Current density magnitude at the surface (outward flow).
    ///
    /// From continuity: J_r(R) = -∂ρ/∂t · R/3 = (ρ₀/τ)·e^(-t/τ)·R/3
    pub fn surface_current_density(&self, t: f64) -> f64 {
        self.rho_0 * self.radius * (-t / self.tau).exp() / (3.0 * self.tau)
    }

    /// Total current leaving the sphere surface.
    ///
    /// I = J_r · 4πR²
    pub fn total_current_out(&self, t: f64) -> f64 {
        self.surface_current_density(t) * 4.0 * PI * self.radius * self.radius
    }

    /// Verify continuity: I_out should equal -dQ/dt.
    ///
    /// -dQ/dt = (ρ₀·V/τ)·e^(-t/τ) = Q₀/τ · e^(-t/τ)
    pub fn minus_dq_dt(&self, t: f64) -> f64 {
        let volume = 4.0 / 3.0 * PI * self.radius.powi(3);
        self.rho_0 * volume * (-t / self.tau).exp() / self.tau
    }

    /// Time for charge to decay to 1/e of initial value.
    pub fn time_constant(&self) -> f64 {
        self.tau
    }

    /// Sample charge and current vs time.
    pub fn sample(
        &self,
        t_end: f64,
        num_points: usize,
    ) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) {
        assert!(num_points >= 2);
        let dt = t_end / (num_points - 1) as f64;
        let times: Vec<f64> = (0..num_points).map(|i| i as f64 * dt).collect();
        let charge: Vec<f64> = times.iter().map(|&t| self.total_charge_at(t)).collect();
        let current: Vec<f64> = times.iter().map(|&t| self.total_current_out(t)).collect();
        let neg_dqdt: Vec<f64> = times.iter().map(|&t| self.minus_dq_dt(t)).collect();
        (times, charge, current, neg_dqdt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn initial_charge_density() {
        let rc = RelaxingCharge::new(1e-6, 1e-3, 0.01);
        assert_relative_eq!(rc.rho_at(0.0), 1e-6, epsilon = 1e-20);
    }

    #[test]
    fn charge_decays_exponentially() {
        let rc = RelaxingCharge::new(1e-6, 1e-3, 0.01);
        let rho_tau = rc.rho_at(1e-3); // at t = τ
        assert_relative_eq!(rho_tau, 1e-6 / std::f64::consts::E, max_relative = 1e-10);
    }

    #[test]
    fn total_charge_proportional_to_volume() {
        let rc1 = RelaxingCharge::new(1e-6, 1e-3, 0.01);
        let rc2 = RelaxingCharge::new(1e-6, 1e-3, 0.02);
        let ratio = rc2.total_charge_at(0.0) / rc1.total_charge_at(0.0);
        assert_relative_eq!(ratio, 8.0, max_relative = 1e-10); // (2r)³/r³ = 8
    }

    #[test]
    fn continuity_equation_holds() {
        let rc = RelaxingCharge::new(1e-6, 1e-3, 0.01);
        // I_out = -dQ/dt at all times
        for t in [0.0, 0.5e-3, 1e-3, 2e-3, 5e-3] {
            let i_out = rc.total_current_out(t);
            let neg_dqdt = rc.minus_dq_dt(t);
            assert_relative_eq!(i_out, neg_dqdt, max_relative = 1e-10);
        }
    }

    #[test]
    fn from_material_tau() {
        // Copper: σ ≈ 5.8e7 S/m, εᵣ = 1
        let rc = RelaxingCharge::from_material(1.0, 1.0, 5.8e7, 0.01);
        let expected_tau = em_core::constants::EPSILON_0 / 5.8e7;
        assert_relative_eq!(rc.tau, expected_tau, max_relative = 1e-10);
    }

    #[test]
    fn copper_relaxation_very_fast() {
        let rc = RelaxingCharge::from_material(1.0, 1.0, 5.8e7, 0.01);
        assert!(rc.tau < 1e-15, "copper relaxation should be sub-femtosecond");
    }

    #[test]
    fn current_at_t0_nonzero() {
        let rc = RelaxingCharge::new(1e-6, 1e-3, 0.01);
        assert!(rc.total_current_out(0.0) > 0.0);
    }

    #[test]
    fn current_decays_with_charge() {
        let rc = RelaxingCharge::new(1e-6, 1e-3, 0.01);
        let i0 = rc.total_current_out(0.0);
        let i1 = rc.total_current_out(1e-3);
        assert!(i1 < i0);
    }

    #[test]
    fn sample_dimensions() {
        let rc = RelaxingCharge::new(1e-6, 1e-3, 0.01);
        let (ts, qs, is, dqs) = rc.sample(5e-3, 50);
        assert_eq!(ts.len(), 50);
        assert_eq!(qs.len(), 50);
        assert_eq!(is.len(), 50);
        assert_eq!(dqs.len(), 50);
    }

    #[test]
    fn sample_continuity_all_points() {
        let rc = RelaxingCharge::new(1e-6, 1e-3, 0.01);
        let (_, _, is, dqs) = rc.sample(5e-3, 100);
        for i in 0..is.len() {
            assert_relative_eq!(is[i], dqs[i], max_relative = 1e-10);
        }
    }
}

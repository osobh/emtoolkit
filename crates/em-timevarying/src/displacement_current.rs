//! Displacement current and the generalized Ampère's law.
//!
//! J_d = ε ∂E/∂t — Maxwell's correction to Ampère's law.
//! In a capacitor, the displacement current equals the conduction current.

use em_core::constants::EPSILON_0;
use serde::{Deserialize, Serialize};

/// A parallel-plate capacitor for displacement current analysis.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ParallelPlateCapacitor {
    /// Plate area (m²)
    pub area: f64,
    /// Plate separation (m)
    pub separation: f64,
    /// Relative permittivity of dielectric
    pub epsilon_r: f64,
}

impl ParallelPlateCapacitor {
    pub fn new(area: f64, separation: f64) -> Self {
        Self {
            area,
            separation,
            epsilon_r: 1.0,
        }
    }

    pub fn with_dielectric(mut self, epsilon_r: f64) -> Self {
        self.epsilon_r = epsilon_r;
        self
    }

    /// Capacitance: C = ε₀ εᵣ A / d
    pub fn capacitance(&self) -> f64 {
        EPSILON_0 * self.epsilon_r * self.area / self.separation
    }

    /// Electric field in the gap for a given voltage.
    pub fn electric_field(&self, voltage: f64) -> f64 {
        voltage / self.separation
    }

    /// Displacement current density for a sinusoidal voltage.
    ///
    /// V(t) = V₀ cos(ωt) → E = V₀ cos(ωt)/d
    /// J_d = ε₀ εᵣ ∂E/∂t = -ε₀ εᵣ V₀ ω sin(ωt) / d
    pub fn displacement_current_density(&self, v_peak: f64, omega: f64, t: f64) -> f64 {
        -EPSILON_0 * self.epsilon_r * v_peak * omega * (omega * t).sin() / self.separation
    }

    /// Total displacement current: I_d = J_d · A
    pub fn displacement_current(&self, v_peak: f64, omega: f64, t: f64) -> f64 {
        self.displacement_current_density(v_peak, omega, t) * self.area
    }

    /// Peak displacement current magnitude.
    ///
    /// I_d_peak = C · V₀ · ω
    pub fn displacement_current_peak(&self, v_peak: f64, omega: f64) -> f64 {
        self.capacitance() * v_peak * omega
    }

    /// Verify displacement current equals conduction current.
    ///
    /// For V(t) = V₀ cos(ωt), I_cond = C dV/dt = -C V₀ ω sin(ωt)
    /// I_d = ε A/d · dE/dt = ε A/d · (-V₀ω/d) sin(ωt) = -C V₀ ω sin(ωt)
    pub fn conduction_current(&self, v_peak: f64, omega: f64, t: f64) -> f64 {
        -self.capacitance() * v_peak * omega * (omega * t).sin()
    }

    /// Energy stored in the capacitor.
    pub fn stored_energy(&self, voltage: f64) -> f64 {
        0.5 * self.capacitance() * voltage * voltage
    }

    /// Sample displacement current and voltage over time.
    pub fn sample(
        &self,
        v_peak: f64,
        omega: f64,
        t_end: f64,
        num_points: usize,
    ) -> DisplacementCurrentSample {
        assert!(num_points >= 2);
        let dt = t_end / (num_points - 1) as f64;
        let times: Vec<f64> = (0..num_points).map(|i| i as f64 * dt).collect();
        let voltage: Vec<f64> = times.iter().map(|&t| v_peak * (omega * t).cos()).collect();
        let i_disp: Vec<f64> = times
            .iter()
            .map(|&t| self.displacement_current(v_peak, omega, t))
            .collect();
        let i_cond: Vec<f64> = times
            .iter()
            .map(|&t| self.conduction_current(v_peak, omega, t))
            .collect();

        DisplacementCurrentSample {
            times,
            voltage,
            displacement_current: i_disp,
            conduction_current: i_cond,
        }
    }
}

/// Sampled displacement current data for visualization.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DisplacementCurrentSample {
    pub times: Vec<f64>,
    pub voltage: Vec<f64>,
    pub displacement_current: Vec<f64>,
    pub conduction_current: Vec<f64>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use std::f64::consts::PI;

    #[test]
    fn capacitance_formula() {
        let cap = ParallelPlateCapacitor::new(0.01, 0.001);
        let expected = EPSILON_0 * 0.01 / 0.001;
        assert_relative_eq!(cap.capacitance(), expected, max_relative = 1e-10);
    }

    #[test]
    fn capacitance_with_dielectric() {
        let cap = ParallelPlateCapacitor::new(0.01, 0.001).with_dielectric(4.0);
        let cap_air = ParallelPlateCapacitor::new(0.01, 0.001);
        assert_relative_eq!(cap.capacitance() / cap_air.capacitance(), 4.0, max_relative = 1e-10);
    }

    #[test]
    fn electric_field_value() {
        let cap = ParallelPlateCapacitor::new(0.01, 0.001);
        assert_relative_eq!(cap.electric_field(100.0), 100_000.0, epsilon = 1e-6);
    }

    #[test]
    fn displacement_current_equals_conduction_current() {
        let cap = ParallelPlateCapacitor::new(0.01, 0.001);
        let v0 = 10.0;
        let omega = 2.0 * PI * 1e6;
        for t in [0.0, 1e-7, 2.5e-7, 5e-7] {
            let id = cap.displacement_current(v0, omega, t);
            let ic = cap.conduction_current(v0, omega, t);
            assert_relative_eq!(id, ic, max_relative = 1e-10);
        }
    }

    #[test]
    fn displacement_current_peak_value() {
        let cap = ParallelPlateCapacitor::new(0.01, 0.001);
        let v0 = 10.0;
        let omega = 2.0 * PI * 1e6;
        let peak = cap.displacement_current_peak(v0, omega);
        let expected = cap.capacitance() * v0 * omega;
        assert_relative_eq!(peak, expected, max_relative = 1e-10);
    }

    #[test]
    fn displacement_current_at_t0_is_zero() {
        let cap = ParallelPlateCapacitor::new(0.01, 0.001);
        let id = cap.displacement_current(10.0, 1000.0, 0.0);
        assert_relative_eq!(id, 0.0, epsilon = 1e-20);
    }

    #[test]
    fn stored_energy() {
        let cap = ParallelPlateCapacitor::new(0.01, 0.001);
        let expected = 0.5 * cap.capacitance() * 100.0;
        assert_relative_eq!(cap.stored_energy(10.0), expected, max_relative = 1e-10);
    }

    #[test]
    fn sample_dimensions() {
        let cap = ParallelPlateCapacitor::new(0.01, 0.001);
        let s = cap.sample(10.0, 1000.0, 0.01, 50);
        assert_eq!(s.times.len(), 50);
        assert_eq!(s.voltage.len(), 50);
        assert_eq!(s.displacement_current.len(), 50);
        assert_eq!(s.conduction_current.len(), 50);
    }

    #[test]
    fn sample_id_equals_ic() {
        let cap = ParallelPlateCapacitor::new(0.01, 0.001);
        let s = cap.sample(10.0, 2.0 * PI * 1e3, 0.001, 20);
        for i in 0..s.times.len() {
            assert_relative_eq!(
                s.displacement_current[i],
                s.conduction_current[i],
                max_relative = 1e-10
            );
        }
    }
}

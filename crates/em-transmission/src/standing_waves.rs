//! Voltage and current standing wave patterns along a transmission line.
//!
//! Computes |V(d)|, |I(d)|, Z(d) as a function of distance d from the load
//! for both lossless and lossy lines.

use em_core::complex::input_impedance_lossless;
use num_complex::Complex64;
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

/// Parameters for standing wave computation on a lossless line.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct StandingWaveParams {
    /// Characteristic impedance Z₀ (Ω)
    pub z0: f64,
    /// Load impedance (complex, Ω)
    pub z_load: Complex64,
    /// Operating frequency (Hz)
    pub frequency: f64,
    /// Phase constant β (rad/m)
    pub beta: f64,
    /// Line length (m)
    pub length: f64,
}

impl StandingWaveParams {
    /// Create with explicit beta.
    pub fn new(z0: f64, z_load: Complex64, frequency: f64, beta: f64, length: f64) -> Self {
        Self {
            z0,
            z_load,
            frequency,
            beta,
            length,
        }
    }

    /// Create for a line in free space.
    pub fn in_free_space(z0: f64, z_load: Complex64, frequency: f64, length: f64) -> Self {
        let beta = 2.0 * PI * frequency / em_core::constants::C_0;
        Self {
            z0,
            z_load,
            frequency,
            beta,
            length,
        }
    }

    /// Reflection coefficient at the load.
    pub fn gamma_load(&self) -> Complex64 {
        em_core::complex::reflection_coefficient(self.z_load, Complex64::new(self.z0, 0.0))
    }

    /// VSWR on the line.
    pub fn vswr(&self) -> f64 {
        em_core::complex::vswr(self.gamma_load())
    }

    /// Voltage magnitude |V(d)| at distance d from the load (normalized to V⁺ = 1).
    ///
    /// |V(d)| = |1 + Γ_L · e^(-j2βd)|
    pub fn voltage_magnitude(&self, d: f64) -> f64 {
        let gamma_l = self.gamma_load();
        let one = Complex64::new(1.0, 0.0);
        let phase = Complex64::from_polar(1.0, -2.0 * self.beta * d);
        (one + gamma_l * phase).norm()
    }

    /// Current magnitude |I(d)| at distance d from the load (normalized to V⁺/Z₀ = 1).
    ///
    /// |I(d)| = |1 - Γ_L · e^(-j2βd)| / Z₀
    pub fn current_magnitude(&self, d: f64) -> f64 {
        let gamma_l = self.gamma_load();
        let one = Complex64::new(1.0, 0.0);
        let phase = Complex64::from_polar(1.0, -2.0 * self.beta * d);
        (one - gamma_l * phase).norm()
    }

    /// Input impedance at distance d from the load.
    pub fn impedance_at(&self, d: f64) -> Complex64 {
        input_impedance_lossless(self.z0, self.z_load, self.beta * d)
    }

    /// Sample voltage standing wave pattern.
    ///
    /// # Returns
    /// (distances_from_load, voltage_magnitudes)
    pub fn sample_voltage(&self, num_points: usize) -> (Vec<f64>, Vec<f64>) {
        assert!(num_points >= 2);
        let dd = self.length / (num_points - 1) as f64;
        let ds: Vec<f64> = (0..num_points).map(|i| i as f64 * dd).collect();
        let vs: Vec<f64> = ds.iter().map(|&d| self.voltage_magnitude(d)).collect();
        (ds, vs)
    }

    /// Sample current standing wave pattern.
    pub fn sample_current(&self, num_points: usize) -> (Vec<f64>, Vec<f64>) {
        assert!(num_points >= 2);
        let dd = self.length / (num_points - 1) as f64;
        let ds: Vec<f64> = (0..num_points).map(|i| i as f64 * dd).collect();
        let is: Vec<f64> = ds.iter().map(|&d| self.current_magnitude(d)).collect();
        (ds, is)
    }

    /// Sample impedance along the line.
    ///
    /// # Returns
    /// (distances, real_parts, imaginary_parts)
    pub fn sample_impedance(&self, num_points: usize) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
        assert!(num_points >= 2);
        let dd = self.length / (num_points - 1) as f64;
        let ds: Vec<f64> = (0..num_points).map(|i| i as f64 * dd).collect();
        let mut re = Vec::with_capacity(num_points);
        let mut im = Vec::with_capacity(num_points);
        for &d in &ds {
            let z = self.impedance_at(d);
            re.push(z.re);
            im.push(z.im);
        }
        (ds, re, im)
    }

    /// Wavelength on the line λ = 2π/β.
    pub fn wavelength(&self) -> f64 {
        2.0 * PI / self.beta
    }

    /// Distance from load to first voltage minimum.
    ///
    /// V_min occurs where Γ_L · e^(-j2βd) = -|Γ_L| (i.e., phase = π + ∠Γ_L)
    /// d_min = (π - ∠Γ_L) / (2β), adjusted to be positive.
    pub fn first_voltage_minimum(&self) -> f64 {
        let angle = self.gamma_load().arg();
        let d = (PI - angle) / (2.0 * self.beta);
        if d < 0.0 {
            d + self.wavelength() / 2.0
        } else {
            d
        }
    }

    /// Distance from load to first voltage maximum.
    ///
    /// V_max occurs where Γ_L · e^(-j2βd) = +|Γ_L|
    /// d_max = -∠Γ_L / (2β), adjusted to be positive.
    pub fn first_voltage_maximum(&self) -> f64 {
        let angle = self.gamma_load().arg();
        let d = -angle / (2.0 * self.beta);
        if d < 0.0 {
            d + self.wavelength() / 2.0
        } else if d == 0.0 && angle == 0.0 {
            0.0
        } else {
            d
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    fn make_test_line() -> StandingWaveParams {
        // 50Ω line, 100Ω resistive load, 1 GHz, 1m length
        StandingWaveParams::in_free_space(
            50.0,
            Complex64::new(100.0, 0.0),
            1e9,
            1.0,
        )
    }

    #[test]
    fn matched_load_no_standing_wave() {
        let sw = StandingWaveParams::in_free_space(50.0, Complex64::new(50.0, 0.0), 1e9, 1.0);
        let (_, vs) = sw.sample_voltage(100);
        // All voltage magnitudes should be 1.0 (flat)
        for v in &vs {
            assert_relative_eq!(*v, 1.0, epsilon = 1e-10);
        }
    }

    #[test]
    fn matched_load_vswr_is_1() {
        let sw = StandingWaveParams::in_free_space(50.0, Complex64::new(50.0, 0.0), 1e9, 1.0);
        assert_relative_eq!(sw.vswr(), 1.0, epsilon = 1e-12);
    }

    #[test]
    fn resistive_mismatch_vswr() {
        let sw = make_test_line(); // ZL/Z0 = 2
        assert_relative_eq!(sw.vswr(), 2.0, epsilon = 1e-10);
    }

    #[test]
    fn voltage_at_load_matches_formula() {
        let sw = make_test_line();
        // At d=0: |V(0)| = |1 + Γ_L|
        let gamma_l = sw.gamma_load();
        let expected = (Complex64::new(1.0, 0.0) + gamma_l).norm();
        assert_relative_eq!(sw.voltage_magnitude(0.0), expected, epsilon = 1e-12);
    }

    #[test]
    fn voltage_max_equals_1_plus_gamma() {
        let sw = make_test_line();
        let gamma_mag = sw.gamma_load().norm();
        let (_, vs) = sw.sample_voltage(10000);
        let v_max: f64 = vs.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        assert_relative_eq!(v_max, 1.0 + gamma_mag, max_relative = 1e-3);
    }

    #[test]
    fn voltage_min_equals_1_minus_gamma() {
        let sw = make_test_line();
        let gamma_mag = sw.gamma_load().norm();
        let (_, vs) = sw.sample_voltage(10000);
        let v_min: f64 = vs.iter().cloned().fold(f64::INFINITY, f64::min);
        assert_relative_eq!(v_min, 1.0 - gamma_mag, max_relative = 1e-3);
    }

    #[test]
    fn standing_wave_pattern_periodic_with_half_wavelength() {
        let sw = make_test_line();
        let lambda = sw.wavelength();
        // V(d) should repeat every λ/2
        let d1 = 0.1;
        let d2 = d1 + lambda / 2.0;
        assert_relative_eq!(
            sw.voltage_magnitude(d1),
            sw.voltage_magnitude(d2),
            epsilon = 1e-10
        );
    }

    #[test]
    fn voltage_and_current_minima_offset_by_quarter_wave() {
        let sw = make_test_line();
        let d_vmin = sw.first_voltage_minimum();
        let lambda = sw.wavelength();
        // Current minimum should be at d_vmin ± λ/4
        // (current max is at voltage min and vice versa)
        let v_at_vmin = sw.voltage_magnitude(d_vmin);
        let i_at_vmin = sw.current_magnitude(d_vmin);
        // At voltage minimum, current should be at maximum
        let (_, is) = sw.sample_current(10000);
        let i_max: f64 = is.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        assert_relative_eq!(i_at_vmin, i_max, max_relative = 1e-3);
    }

    #[test]
    fn impedance_at_load_is_z_load() {
        let sw = make_test_line();
        let z = sw.impedance_at(0.0);
        assert_relative_eq!(z.re, 100.0, epsilon = 1e-8);
        assert_relative_eq!(z.im, 0.0, epsilon = 1e-8);
    }

    #[test]
    fn impedance_at_quarter_wave_is_z0_squared_over_zl() {
        let sw = make_test_line();
        let lambda = sw.wavelength();
        let z = sw.impedance_at(lambda / 4.0);
        // Zin = Z0²/ZL = 2500/100 = 25
        assert_relative_eq!(z.re, 25.0, epsilon = 1e-6);
        assert_relative_eq!(z.im, 0.0, epsilon = 1e-6);
    }

    #[test]
    fn short_circuit_load_first_vmax_at_quarter_wave() {
        let sw = StandingWaveParams::in_free_space(
            50.0,
            Complex64::new(0.0, 0.0),
            1e9,
            1.0,
        );
        let lambda = sw.wavelength();
        let d_max = sw.first_voltage_maximum();
        assert_relative_eq!(d_max, lambda / 4.0, max_relative = 1e-6);
    }

    #[test]
    fn sample_returns_correct_length() {
        let sw = make_test_line();
        let (d, v) = sw.sample_voltage(200);
        assert_eq!(d.len(), 200);
        assert_eq!(v.len(), 200);
        let (d, r, x) = sw.sample_impedance(150);
        assert_eq!(d.len(), 150);
        assert_eq!(r.len(), 150);
        assert_eq!(x.len(), 150);
    }
}

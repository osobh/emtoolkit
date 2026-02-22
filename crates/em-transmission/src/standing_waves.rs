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

/// Standing wave result data structure for WASM bindings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StandingWaveData {
    /// Positions along line (m)
    pub positions: Vec<f64>,
    /// Voltage magnitude at each position
    pub voltage_mag: Vec<f64>,
    /// Current magnitude at each position
    pub current_mag: Vec<f64>,
    /// VSWR value
    pub vswr: f64,
    /// Reflection coefficient magnitude
    pub gamma_mag: f64,
    /// Reflection coefficient angle (radians)
    pub gamma_angle: f64,
    /// Voltage maximum
    pub v_max: f64,
    /// Voltage minimum
    pub v_min: f64,
    /// Distance to first voltage minimum
    pub d_to_first_min: f64,
    /// Distance to first voltage maximum
    pub d_to_first_max: f64,
}

/// Compute standing wave pattern from VSWR and reflection coefficient angle.
///
/// This allows users to specify VSWR directly instead of load impedance.
///
/// # Arguments
/// * `_z0` - Characteristic impedance (Ω) - reserved for future use
/// * `vswr` - Voltage standing wave ratio (≥ 1.0)
/// * `gamma_angle_deg` - Reflection coefficient angle in degrees
/// * `frequency` - Operating frequency (Hz)
/// * `line_length` - Length of line to analyze (m)
/// * `num_points` - Number of sample points
pub fn pattern_from_vswr(
    _z0: f64,
    vswr: f64,
    gamma_angle_deg: f64,
    frequency: f64,
    line_length: f64,
    num_points: usize,
) -> StandingWaveData {
    assert!(vswr >= 1.0, "VSWR must be >= 1.0");
    assert!(num_points >= 2);

    // Convert VSWR to |Γ|
    let gamma_mag = (vswr - 1.0) / (vswr + 1.0);
    let gamma_angle = gamma_angle_deg.to_radians();

    // Construct Γ_L
    let gamma_l = Complex64::from_polar(gamma_mag, gamma_angle);

    // Calculate β = ω/v = 2πf/c (assuming free space or specify v)
    let beta = 2.0 * PI * frequency / em_core::constants::C_0;

    let mut positions = Vec::with_capacity(num_points);
    let mut voltage_mag = Vec::with_capacity(num_points);
    let mut current_mag = Vec::with_capacity(num_points);

    let one = Complex64::new(1.0, 0.0);
    let dd = line_length / (num_points - 1) as f64;

    for i in 0..num_points {
        let d = i as f64 * dd;
        positions.push(d);

        // |V(d)| = |1 + Γ_L · e^(-j2βd)|
        let phase = Complex64::from_polar(1.0, -2.0 * beta * d);
        let v = (one + gamma_l * phase).norm();
        voltage_mag.push(v);

        // |I(d)| = |1 - Γ_L · e^(-j2βd)|
        let i_mag = (one - gamma_l * phase).norm();
        current_mag.push(i_mag);
    }

    // Calculate V_max and V_min
    let v_max = 1.0 + gamma_mag;
    let v_min = 1.0 - gamma_mag;

    // Distance to first voltage minimum
    // V_min occurs where phase of Γ_L·e^{-j2βd} = π (destructive)
    // γ - 2βd = π → d = (γ - π)/(2β)
    let d_to_first_min = (gamma_angle - PI) / (-2.0 * beta);
    let d_to_first_min = if d_to_first_min < 0.0 {
        d_to_first_min + PI / beta // Add λ/2
    } else {
        d_to_first_min
    };

    // Distance to first voltage maximum
    // V_max occurs where phase = 0 (constructive)
    // γ - 2βd = 0 → d = γ/(2β)
    let d_to_first_max = gamma_angle / (2.0 * beta);
    let d_to_first_max = if d_to_first_max < 0.0 {
        d_to_first_max + PI / beta
    } else if d_to_first_max == 0.0 && gamma_angle == 0.0 {
        0.0
    } else {
        d_to_first_max
    };

    StandingWaveData {
        positions,
        voltage_mag,
        current_mag,
        vswr,
        gamma_mag,
        gamma_angle,
        v_max,
        v_min,
        d_to_first_min,
        d_to_first_max,
    }
}

/// Get standing wave data from load impedance (wrapper for WASM).
pub fn pattern_from_load(
    z0: f64,
    zl_re: f64,
    zl_im: f64,
    frequency: f64,
    line_length: f64,
    num_points: usize,
) -> StandingWaveData {
    let zl = Complex64::new(zl_re, zl_im);
    let sw = StandingWaveParams::in_free_space(z0, zl, frequency, line_length);

    let (positions, voltage_mag) = sw.sample_voltage(num_points);
    let (_, current_mag) = sw.sample_current(num_points);
    let gamma_l = sw.gamma_load();

    StandingWaveData {
        positions,
        voltage_mag,
        current_mag,
        vswr: sw.vswr(),
        gamma_mag: gamma_l.norm(),
        gamma_angle: gamma_l.arg(),
        v_max: 1.0 + gamma_l.norm(),
        v_min: 1.0 - gamma_l.norm(),
        d_to_first_min: sw.first_voltage_minimum(),
        d_to_first_max: sw.first_voltage_maximum(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_vswr_pattern_matched() {
        let data = pattern_from_vswr(50.0, 1.0, 0.0, 1e9, 1.0, 100);
        assert_relative_eq!(data.gamma_mag, 0.0, epsilon = 1e-12);
        for v in &data.voltage_mag {
            assert_relative_eq!(*v, 1.0, epsilon = 1e-10);
        }
    }

    #[test]
    fn test_vswr_to_gamma() {
        let data = pattern_from_vswr(50.0, 3.0, 0.0, 1e9, 1.0, 100);
        // VSWR = 3 → |Γ| = (3-1)/(3+1) = 0.5
        assert_relative_eq!(data.gamma_mag, 0.5, epsilon = 1e-10);
    }

    #[test]
    fn test_vswr_pattern_minmax() {
        let data = pattern_from_vswr(50.0, 2.0, 0.0, 1e9, 1.0, 1000);
        // |Γ| = (2-1)/(2+1) = 1/3
        // V_max = 1 + 1/3 = 4/3, V_min = 1 - 1/3 = 2/3
        assert_relative_eq!(data.v_max, 4.0 / 3.0, epsilon = 1e-10);
        assert_relative_eq!(data.v_min, 2.0 / 3.0, epsilon = 1e-10);
    }

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

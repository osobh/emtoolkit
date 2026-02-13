//! Interactive Smith Chart computation engine.
//!
//! Provides all mathematical operations needed for the Smith chart module:
//! - Impedance ↔ reflection coefficient mapping
//! - Normalized impedance/admittance conversion
//! - Constant resistance and reactance circle geometry
//! - SWR circle computation
//! - Moving along the transmission line (rotation on Smith chart)
//! - Q circle computation

use em_core::complex::vswr;
use num_complex::Complex64;
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

/// A point on the Smith chart with both impedance and reflection coefficient representations.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SmithPoint {
    /// Normalized impedance z = Z/Z₀ = r + jx
    pub z_normalized: Complex64,
    /// Reflection coefficient Γ = Γᵣ + jΓᵢ
    pub gamma: Complex64,
    /// Normalized admittance y = Y·Z₀ = g + jb
    pub y_normalized: Complex64,
}

impl SmithPoint {
    /// Create a Smith chart point from normalized impedance z = r + jx.
    pub fn from_impedance(z_normalized: Complex64) -> Self {
        let one = Complex64::new(1.0, 0.0);
        let gamma = (z_normalized - one) / (z_normalized + one);
        let y_normalized = one / z_normalized;
        Self {
            z_normalized,
            gamma,
            y_normalized,
        }
    }

    /// Create a Smith chart point from reflection coefficient Γ.
    pub fn from_gamma(gamma: Complex64) -> Self {
        let one = Complex64::new(1.0, 0.0);
        let z_normalized = (one + gamma) / (one - gamma);
        let y_normalized = one / z_normalized;
        Self {
            z_normalized,
            gamma,
            y_normalized,
        }
    }

    /// Create from actual impedance and characteristic impedance.
    pub fn from_impedance_and_z0(z: Complex64, z0: f64) -> Self {
        Self::from_impedance(z / z0)
    }

    /// Get the actual impedance given Z₀.
    pub fn impedance(&self, z0: f64) -> Complex64 {
        self.z_normalized * z0
    }

    /// Get the actual admittance given Z₀.
    pub fn admittance(&self, z0: f64) -> Complex64 {
        self.y_normalized / z0
    }

    /// Resistance component r of normalized impedance.
    pub fn r(&self) -> f64 {
        self.z_normalized.re
    }

    /// Reactance component x of normalized impedance.
    pub fn x(&self) -> f64 {
        self.z_normalized.im
    }

    /// Conductance component g of normalized admittance.
    pub fn g(&self) -> f64 {
        self.y_normalized.re
    }

    /// Susceptance component b of normalized admittance.
    pub fn b(&self) -> f64 {
        self.y_normalized.im
    }

    /// |Γ| — magnitude of reflection coefficient.
    pub fn gamma_magnitude(&self) -> f64 {
        self.gamma.norm()
    }

    /// ∠Γ in radians.
    pub fn gamma_angle_rad(&self) -> f64 {
        self.gamma.arg()
    }

    /// ∠Γ in degrees.
    pub fn gamma_angle_deg(&self) -> f64 {
        self.gamma.arg().to_degrees()
    }

    /// VSWR at this point.
    pub fn vswr(&self) -> f64 {
        vswr(self.gamma)
    }

    /// Return loss in dB: RL = -20·log₁₀(|Γ|).
    pub fn return_loss_db(&self) -> f64 {
        -20.0 * self.gamma_magnitude().log10()
    }

    /// Mismatch loss in dB: ML = -10·log₁₀(1 - |Γ|²).
    pub fn mismatch_loss_db(&self) -> f64 {
        let mag_sq = self.gamma_magnitude().powi(2);
        -10.0 * (1.0 - mag_sq).log10()
    }

    /// Move along a lossless transmission line by electrical length βl (radians).
    ///
    /// Moving toward the generator rotates Γ clockwise by 2βl on the Smith chart.
    pub fn move_toward_generator(&self, beta_l: f64) -> Self {
        let rotated = self.gamma * Complex64::from_polar(1.0, -2.0 * beta_l);
        Self::from_gamma(rotated)
    }

    /// Move toward the load by electrical length βl (radians).
    pub fn move_toward_load(&self, beta_l: f64) -> Self {
        let rotated = self.gamma * Complex64::from_polar(1.0, 2.0 * beta_l);
        Self::from_gamma(rotated)
    }
}

/// Geometry of a constant-resistance circle on the Smith chart.
///
/// The circle for normalized resistance r has:
/// - Center at (r/(1+r), 0) in the Γ plane
/// - Radius 1/(1+r)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ConstantRCircle {
    pub r: f64,
    pub center_x: f64,
    pub center_y: f64,
    pub radius: f64,
}

/// Compute the constant-resistance circle for normalized resistance r.
pub fn constant_r_circle(r: f64) -> ConstantRCircle {
    ConstantRCircle {
        r,
        center_x: r / (1.0 + r),
        center_y: 0.0,
        radius: 1.0 / (1.0 + r),
    }
}

/// Geometry of a constant-reactance arc on the Smith chart.
///
/// The circle for normalized reactance x has:
/// - Center at (1, 1/x) in the Γ plane
/// - Radius |1/x|
///
/// Only the arc inside the unit circle (|Γ| ≤ 1) is drawn.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ConstantXCircle {
    pub x: f64,
    pub center_x: f64,
    pub center_y: f64,
    pub radius: f64,
}

/// Compute the constant-reactance circle for normalized reactance x.
///
/// x = 0 corresponds to the real axis (degenerate case).
pub fn constant_x_circle(x: f64) -> ConstantXCircle {
    if x == 0.0 {
        // Real axis — degenerate circle with infinite radius
        return ConstantXCircle {
            x: 0.0,
            center_x: 1.0,
            center_y: f64::INFINITY,
            radius: f64::INFINITY,
        };
    }
    ConstantXCircle {
        x,
        center_x: 1.0,
        center_y: 1.0 / x,
        radius: (1.0 / x).abs(),
    }
}

/// Geometry of the SWR circle (constant |Γ| circle centered at origin).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SwrCircle {
    pub vswr: f64,
    pub gamma_magnitude: f64,
    pub radius: f64,
}

/// Compute the SWR circle for a given VSWR value.
pub fn swr_circle(vswr_val: f64) -> SwrCircle {
    let gamma_mag = (vswr_val - 1.0) / (vswr_val + 1.0);
    SwrCircle {
        vswr: vswr_val,
        gamma_magnitude: gamma_mag,
        radius: gamma_mag,
    }
}

/// Compute the SWR circle from a reflection coefficient.
pub fn swr_circle_from_gamma(gamma: Complex64) -> SwrCircle {
    let gamma_mag = gamma.norm();
    let vswr_val = (1.0 + gamma_mag) / (1.0 - gamma_mag);
    SwrCircle {
        vswr: vswr_val,
        gamma_magnitude: gamma_mag,
        radius: gamma_mag,
    }
}

/// Generate points along an SWR circle for rendering.
///
/// # Arguments
/// * `gamma_magnitude` - |Γ| (radius of the circle)
/// * `num_points` - Number of points to generate
///
/// # Returns
/// Vector of (Γᵣ, Γᵢ) coordinates on the SWR circle.
pub fn swr_circle_points(gamma_magnitude: f64, num_points: usize) -> Vec<(f64, f64)> {
    (0..num_points)
        .map(|i| {
            let angle = 2.0 * PI * i as f64 / num_points as f64;
            (
                gamma_magnitude * angle.cos(),
                gamma_magnitude * angle.sin(),
            )
        })
        .collect()
}

/// Trace the impedance along a transmission line from load to generator.
///
/// # Arguments
/// * `load_point` - Smith chart point at the load
/// * `num_points` - Number of points to trace
/// * `total_electrical_length` - Total βl in radians
///
/// # Returns
/// Vector of SmithPoints from load to generator.
pub fn trace_toward_generator(
    load_point: &SmithPoint,
    num_points: usize,
    total_electrical_length: f64,
) -> Vec<SmithPoint> {
    (0..num_points)
        .map(|i| {
            let beta_l = total_electrical_length * i as f64 / (num_points - 1).max(1) as f64;
            load_point.move_toward_generator(beta_l)
        })
        .collect()
}

/// Q-circle: constant Q = |x|/r on the Smith chart.
///
/// For a given Q value, the circle passes through the origin and center of the chart
/// with center at (0, ±1/Q) and radius 1/Q (in normalized impedance plane).
/// In the Γ-plane, Q circles are more complex — we compute them by tracing.
pub fn q_circle_points(q: f64, num_points: usize) -> Vec<(f64, f64)> {
    // Trace points where |x|/r = Q on the upper half (positive x)
    // r ranges from 0 to ∞, x = Q·r
    let mut points = Vec::with_capacity(num_points * 2);

    for i in 0..num_points {
        let t = i as f64 / (num_points - 1).max(1) as f64;
        // Map t ∈ [0,1] to r ∈ [0, large] using tan mapping for better coverage
        let r = (t * PI / 2.0 * 0.99).tan(); // avoid infinity
        let x = q * r;
        let z = Complex64::new(r, x);
        let sp = SmithPoint::from_impedance(z);
        points.push((sp.gamma.re, sp.gamma.im));
    }
    // Mirror for negative x (lower half)
    for i in (0..num_points).rev() {
        let t = i as f64 / (num_points - 1).max(1) as f64;
        let r = (t * PI / 2.0 * 0.99).tan();
        let x = -q * r;
        let z = Complex64::new(r, x);
        let sp = SmithPoint::from_impedance(z);
        points.push((sp.gamma.re, sp.gamma.im));
    }
    points
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use std::f64::consts::PI;

    // ================================================================
    // SmithPoint creation and roundtrips
    // ================================================================

    #[test]
    fn smith_point_matched_load() {
        let sp = SmithPoint::from_impedance(Complex64::new(1.0, 0.0));
        assert_relative_eq!(sp.gamma.norm(), 0.0, epsilon = 1e-12);
        assert_relative_eq!(sp.vswr(), 1.0, epsilon = 1e-12);
    }

    #[test]
    fn smith_point_open_circuit() {
        let sp = SmithPoint::from_impedance(Complex64::new(1e15, 0.0));
        assert_relative_eq!(sp.gamma.re, 1.0, epsilon = 1e-6);
        assert_relative_eq!(sp.gamma.im, 0.0, epsilon = 1e-6);
    }

    #[test]
    fn smith_point_short_circuit() {
        let sp = SmithPoint::from_impedance(Complex64::new(0.0, 0.0));
        assert_relative_eq!(sp.gamma.re, -1.0, epsilon = 1e-12);
        assert_relative_eq!(sp.gamma.im, 0.0, epsilon = 1e-12);
    }

    #[test]
    fn smith_point_roundtrip_impedance_to_gamma() {
        let z = Complex64::new(0.5, 1.0);
        let sp = SmithPoint::from_impedance(z);
        let sp2 = SmithPoint::from_gamma(sp.gamma);
        assert_relative_eq!(sp2.z_normalized.re, z.re, epsilon = 1e-10);
        assert_relative_eq!(sp2.z_normalized.im, z.im, epsilon = 1e-10);
    }

    #[test]
    fn smith_point_from_actual_impedance() {
        let z = Complex64::new(100.0, 50.0);
        let z0 = 50.0;
        let sp = SmithPoint::from_impedance_and_z0(z, z0);
        assert_relative_eq!(sp.r(), 2.0, epsilon = 1e-10);
        assert_relative_eq!(sp.x(), 1.0, epsilon = 1e-10);
    }

    #[test]
    fn smith_point_admittance_is_reciprocal() {
        let z = Complex64::new(2.0, 1.0);
        let sp = SmithPoint::from_impedance(z);
        let y = sp.y_normalized;
        let product = z * y;
        assert_relative_eq!(product.re, 1.0, epsilon = 1e-10);
        assert_relative_eq!(product.im, 0.0, epsilon = 1e-10);
    }

    #[test]
    fn smith_point_impedance_recovery() {
        let z = Complex64::new(75.0, -25.0);
        let z0 = 50.0;
        let sp = SmithPoint::from_impedance_and_z0(z, z0);
        let recovered = sp.impedance(z0);
        assert_relative_eq!(recovered.re, z.re, epsilon = 1e-10);
        assert_relative_eq!(recovered.im, z.im, epsilon = 1e-10);
    }

    // ================================================================
    // VSWR and losses
    // ================================================================

    #[test]
    fn vswr_for_2_to_1_mismatch() {
        // Z = 2·Z0 → Γ = 1/3 → VSWR = 2
        let sp = SmithPoint::from_impedance(Complex64::new(2.0, 0.0));
        assert_relative_eq!(sp.vswr(), 2.0, epsilon = 1e-10);
    }

    #[test]
    fn return_loss_matched_is_infinite() {
        let sp = SmithPoint::from_impedance(Complex64::new(1.0, 0.0));
        assert!(sp.return_loss_db().is_infinite());
    }

    #[test]
    fn return_loss_3db_mismatch() {
        // |Γ| = 10^(-3/20) ≈ 0.708
        let gamma_mag = 10.0_f64.powf(-3.0 / 20.0);
        let gamma = Complex64::new(gamma_mag, 0.0);
        let sp = SmithPoint::from_gamma(gamma);
        assert_relative_eq!(sp.return_loss_db(), 3.0, max_relative = 0.01);
    }

    #[test]
    fn mismatch_loss_matched_is_zero() {
        let sp = SmithPoint::from_impedance(Complex64::new(1.0, 0.0));
        assert_relative_eq!(sp.mismatch_loss_db(), 0.0, epsilon = 1e-10);
    }

    // ================================================================
    // Moving along the line
    // ================================================================

    #[test]
    fn move_half_wavelength_returns_to_same_point() {
        let sp = SmithPoint::from_impedance(Complex64::new(0.5, 0.8));
        let moved = sp.move_toward_generator(PI); // βl = π → half wavelength
        assert_relative_eq!(moved.z_normalized.re, sp.z_normalized.re, epsilon = 1e-10);
        assert_relative_eq!(moved.z_normalized.im, sp.z_normalized.im, epsilon = 1e-10);
    }

    #[test]
    fn move_quarter_wave_from_short_gives_open() {
        let sp = SmithPoint::from_impedance(Complex64::new(0.0, 0.0)); // short
        let moved = sp.move_toward_generator(PI / 2.0); // quarter wave
        // Should be at open circuit: Γ ≈ +1
        assert_relative_eq!(moved.gamma.re, 1.0, epsilon = 1e-6);
        assert_relative_eq!(moved.gamma.im, 0.0, epsilon = 1e-6);
    }

    #[test]
    fn move_toward_load_is_inverse_of_generator() {
        let sp = SmithPoint::from_impedance(Complex64::new(1.5, 0.7));
        let beta_l = 0.3;
        let moved_gen = sp.move_toward_generator(beta_l);
        let back = moved_gen.move_toward_load(beta_l);
        assert_relative_eq!(back.z_normalized.re, sp.z_normalized.re, epsilon = 1e-10);
        assert_relative_eq!(back.z_normalized.im, sp.z_normalized.im, epsilon = 1e-10);
    }

    #[test]
    fn moving_preserves_gamma_magnitude() {
        let sp = SmithPoint::from_impedance(Complex64::new(0.3, 1.2));
        let moved = sp.move_toward_generator(1.234);
        assert_relative_eq!(
            moved.gamma_magnitude(),
            sp.gamma_magnitude(),
            epsilon = 1e-12
        );
    }

    // ================================================================
    // Constant-r circles
    // ================================================================

    #[test]
    fn constant_r_0_circle() {
        // r = 0: center at (0, 0), radius = 1 (unit circle)
        let c = constant_r_circle(0.0);
        assert_relative_eq!(c.center_x, 0.0, epsilon = 1e-12);
        assert_relative_eq!(c.radius, 1.0, epsilon = 1e-12);
    }

    #[test]
    fn constant_r_1_circle() {
        // r = 1: center at (0.5, 0), radius = 0.5
        let c = constant_r_circle(1.0);
        assert_relative_eq!(c.center_x, 0.5, epsilon = 1e-12);
        assert_relative_eq!(c.radius, 0.5, epsilon = 1e-12);
    }

    #[test]
    fn constant_r_infinity_degenerates_to_point() {
        let c = constant_r_circle(1e10);
        assert_relative_eq!(c.center_x, 1.0, epsilon = 1e-4);
        assert_relative_eq!(c.radius, 0.0, epsilon = 1e-4);
    }

    #[test]
    fn constant_r_circle_passes_through_correct_impedances() {
        // For r = 1, the circle should pass through Γ = 0 (matched, z = 1+j0)
        let c = constant_r_circle(1.0);
        // Point z = 1 + j0 → Γ = 0
        let dist_to_origin = (c.center_x * c.center_x + c.center_y * c.center_y).sqrt();
        assert_relative_eq!(dist_to_origin, c.radius, epsilon = 1e-12);
    }

    // ================================================================
    // Constant-x circles
    // ================================================================

    #[test]
    fn constant_x_1_circle() {
        // x = 1: center at (1, 1), radius = 1
        let c = constant_x_circle(1.0);
        assert_relative_eq!(c.center_x, 1.0, epsilon = 1e-12);
        assert_relative_eq!(c.center_y, 1.0, epsilon = 1e-12);
        assert_relative_eq!(c.radius, 1.0, epsilon = 1e-12);
    }

    #[test]
    fn constant_x_negative_circle() {
        // x = -1: center at (1, -1), radius = 1
        let c = constant_x_circle(-1.0);
        assert_relative_eq!(c.center_y, -1.0, epsilon = 1e-12);
        assert_relative_eq!(c.radius, 1.0, epsilon = 1e-12);
    }

    #[test]
    fn constant_x_0_is_real_axis() {
        let c = constant_x_circle(0.0);
        assert!(c.radius.is_infinite());
    }

    // ================================================================
    // SWR circles
    // ================================================================

    #[test]
    fn swr_circle_vswr_1_has_zero_radius() {
        let c = swr_circle(1.0);
        assert_relative_eq!(c.radius, 0.0, epsilon = 1e-12);
    }

    #[test]
    fn swr_circle_vswr_3_has_correct_gamma() {
        // VSWR = 3 → |Γ| = (3-1)/(3+1) = 0.5
        let c = swr_circle(3.0);
        assert_relative_eq!(c.gamma_magnitude, 0.5, epsilon = 1e-12);
        assert_relative_eq!(c.radius, 0.5, epsilon = 1e-12);
    }

    #[test]
    fn swr_circle_from_gamma_consistent() {
        let gamma = Complex64::new(0.3, 0.4); // |Γ| = 0.5
        let c = swr_circle_from_gamma(gamma);
        assert_relative_eq!(c.gamma_magnitude, 0.5, epsilon = 1e-12);
        assert_relative_eq!(c.vswr, 3.0, epsilon = 1e-12);
    }

    #[test]
    fn swr_circle_points_lie_on_circle() {
        let r = 0.6;
        let points = swr_circle_points(r, 100);
        for (x, y) in &points {
            let dist = (x * x + y * y).sqrt();
            assert_relative_eq!(dist, r, epsilon = 1e-10);
        }
    }

    // ================================================================
    // Trace along line
    // ================================================================

    #[test]
    fn trace_returns_correct_number_of_points() {
        let sp = SmithPoint::from_impedance(Complex64::new(0.5, 0.5));
        let trace = trace_toward_generator(&sp, 100, PI);
        assert_eq!(trace.len(), 100);
    }

    #[test]
    fn trace_first_point_is_load() {
        let sp = SmithPoint::from_impedance(Complex64::new(0.5, 0.5));
        let trace = trace_toward_generator(&sp, 50, PI);
        assert_relative_eq!(trace[0].z_normalized.re, sp.z_normalized.re, epsilon = 1e-12);
        assert_relative_eq!(trace[0].z_normalized.im, sp.z_normalized.im, epsilon = 1e-12);
    }

    #[test]
    fn trace_all_points_same_gamma_magnitude() {
        let sp = SmithPoint::from_impedance(Complex64::new(0.3, 0.8));
        let trace = trace_toward_generator(&sp, 100, 2.0 * PI);
        let expected_mag = sp.gamma_magnitude();
        for pt in &trace {
            assert_relative_eq!(pt.gamma_magnitude(), expected_mag, epsilon = 1e-12);
        }
    }
}

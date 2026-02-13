//! Magnetic field of circular current loops.
//!
//! Exact on-axis formula plus numerical Biot-Savart for off-axis points.

use em_core::constants::MU_0;
use em_core::coordinates::{Cartesian, Vector3};
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

use super::biot_savart::{CurrentSegment, b_field_total};

/// A circular current loop in the xy-plane centered at origin.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct CurrentLoop {
    /// Radius of the loop (m)
    pub radius: f64,
    /// Current in the loop (A) — positive = counterclockwise when viewed from +z
    pub current: f64,
    /// Center z-position (m)
    pub center_z: f64,
}

impl CurrentLoop {
    pub fn new(radius: f64, current: f64) -> Self {
        Self {
            radius,
            current,
            center_z: 0.0,
        }
    }

    pub fn at_z(radius: f64, current: f64, center_z: f64) -> Self {
        Self {
            radius,
            current,
            center_z,
        }
    }

    /// Exact on-axis magnetic field (along z-axis).
    ///
    /// B_z = μ₀ I a² / (2(a² + z²)^(3/2))
    ///
    /// where a = radius, z = distance from loop center along axis.
    pub fn b_on_axis(&self, z: f64) -> f64 {
        let z_rel = z - self.center_z;
        let a = self.radius;
        MU_0 * self.current * a * a / (2.0 * (a * a + z_rel * z_rel).powf(1.5))
    }

    /// Magnetic moment of the loop: m = I·A = I·π·a²
    pub fn magnetic_moment(&self) -> f64 {
        self.current * PI * self.radius * self.radius
    }

    /// Discretize the loop into segments for numerical Biot-Savart computation.
    pub fn discretize(&self, num_segments: usize) -> Vec<CurrentSegment> {
        assert!(num_segments >= 3);
        let a = self.radius;
        let dphi = 2.0 * PI / num_segments as f64;
        let mut segments = Vec::with_capacity(num_segments);

        for i in 0..num_segments {
            let phi0 = i as f64 * dphi;
            let phi1 = (i + 1) as f64 * dphi;
            segments.push(CurrentSegment::new(
                Cartesian::new(a * phi0.cos(), a * phi0.sin(), self.center_z),
                Cartesian::new(a * phi1.cos(), a * phi1.sin(), self.center_z),
                self.current,
            ));
        }

        segments
    }

    /// Compute B-field at any point using numerical Biot-Savart.
    pub fn b_field_at(&self, point: &Cartesian, num_segments: usize) -> Vector3 {
        let segments = self.discretize(num_segments);
        b_field_total(&segments, point)
    }
}

/// Helmholtz coil: two identical coaxial loops separated by their radius.
///
/// Creates a nearly uniform field in the region between the coils.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct HelmholtzCoil {
    /// Radius of each coil (m)
    pub radius: f64,
    /// Current in each coil (A)
    pub current: f64,
    /// Number of turns per coil
    pub turns: usize,
}

impl HelmholtzCoil {
    pub fn new(radius: f64, current: f64, turns: usize) -> Self {
        Self {
            radius,
            current,
            turns,
        }
    }

    /// Separation distance (= radius for Helmholtz condition).
    pub fn separation(&self) -> f64 {
        self.radius
    }

    /// B-field at the midpoint between the coils (on axis).
    ///
    /// B = μ₀ n I (4/5)^(3/2) / R (per coil pair)
    pub fn b_at_center(&self) -> f64 {
        let n = self.turns as f64;
        let effective_current = n * self.current;
        let loop1 = CurrentLoop::at_z(self.radius, effective_current, -self.radius / 2.0);
        let loop2 = CurrentLoop::at_z(self.radius, effective_current, self.radius / 2.0);
        loop1.b_on_axis(0.0) + loop2.b_on_axis(0.0)
    }

    /// B-field on axis at position z from the midpoint.
    pub fn b_on_axis(&self, z: f64) -> f64 {
        let n = self.turns as f64;
        let effective_current = n * self.current;
        let loop1 = CurrentLoop::at_z(self.radius, effective_current, -self.radius / 2.0);
        let loop2 = CurrentLoop::at_z(self.radius, effective_current, self.radius / 2.0);
        loop1.b_on_axis(z) + loop2.b_on_axis(z)
    }

    /// Sample on-axis B-field for uniformity visualization.
    pub fn sample_on_axis(&self, z_range: (f64, f64), num_points: usize) -> (Vec<f64>, Vec<f64>) {
        assert!(num_points >= 2);
        let dz = (z_range.1 - z_range.0) / (num_points - 1) as f64;
        let z_vals: Vec<f64> = (0..num_points).map(|i| z_range.0 + i as f64 * dz).collect();
        let b_vals: Vec<f64> = z_vals.iter().map(|&z| self.b_on_axis(z)).collect();
        (z_vals, b_vals)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn b_on_axis_at_center() {
        // B_z(0) = μ₀ I / (2a) for z=0
        let loop1 = CurrentLoop::new(0.1, 1.0);
        let b = loop1.b_on_axis(0.0);
        let expected = MU_0 * 1.0 / (2.0 * 0.1);
        assert_relative_eq!(b, expected, max_relative = 1e-10);
    }

    #[test]
    fn b_on_axis_decreases_with_distance() {
        let loop1 = CurrentLoop::new(0.1, 1.0);
        let b0 = loop1.b_on_axis(0.0);
        let b1 = loop1.b_on_axis(0.2);
        assert!(b0 > b1);
    }

    #[test]
    fn b_on_axis_symmetric() {
        let loop1 = CurrentLoop::new(0.1, 1.0);
        let b_pos = loop1.b_on_axis(0.05);
        let b_neg = loop1.b_on_axis(-0.05);
        assert_relative_eq!(b_pos, b_neg, max_relative = 1e-10);
    }

    #[test]
    fn b_on_axis_far_field_dipole() {
        // Far from loop, B ≈ μ₀ m / (2π z³) where m = I·π·a²
        let loop1 = CurrentLoop::new(0.01, 1.0);
        let z = 1.0; // far from loop
        let b = loop1.b_on_axis(z);
        let m = loop1.magnetic_moment();
        let b_dipole = MU_0 * m / (2.0 * PI * z.powi(3));
        assert_relative_eq!(b, b_dipole, max_relative = 0.001);
    }

    #[test]
    fn magnetic_moment_value() {
        let loop1 = CurrentLoop::new(0.1, 2.0);
        let expected = 2.0 * PI * 0.01;
        assert_relative_eq!(loop1.magnetic_moment(), expected, max_relative = 1e-10);
    }

    #[test]
    fn numerical_matches_on_axis_formula() {
        let loop1 = CurrentLoop::new(0.1, 1.0);
        let z = 0.05;
        let b_exact = loop1.b_on_axis(z);
        let b_num = loop1.b_field_at(&Cartesian::new(0.0, 0.0, z), 1000);
        assert_relative_eq!(b_num.z, b_exact, max_relative = 0.01);
        assert_relative_eq!(b_num.x, 0.0, epsilon = 1e-10);
    }

    #[test]
    fn discretize_loop_correct_segments() {
        let loop1 = CurrentLoop::new(0.1, 1.0);
        let segs = loop1.discretize(100);
        assert_eq!(segs.len(), 100);
        // First segment starts at (a, 0, 0)
        assert_relative_eq!(segs[0].start.x, 0.1, epsilon = 1e-12);
        assert_relative_eq!(segs[0].start.y, 0.0, epsilon = 1e-12);
    }

    #[test]
    fn discretize_loop_closes() {
        let loop1 = CurrentLoop::new(0.1, 1.0);
        let segs = loop1.discretize(100);
        let last = segs.last().unwrap();
        // Last segment end should be close to first segment start
        assert_relative_eq!(last.end.x, segs[0].start.x, epsilon = 1e-10);
        assert_relative_eq!(last.end.y, segs[0].start.y, epsilon = 1e-10);
    }

    // Helmholtz coil tests

    #[test]
    fn helmholtz_separation_equals_radius() {
        let hc = HelmholtzCoil::new(0.1, 1.0, 1);
        assert_relative_eq!(hc.separation(), 0.1, epsilon = 1e-12);
    }

    #[test]
    fn helmholtz_center_field_formula() {
        // B_center = μ₀ n I (4/5)^(3/2) / R
        let r = 0.1;
        let hc = HelmholtzCoil::new(r, 1.0, 1);
        let b = hc.b_at_center();
        let expected = MU_0 * 1.0 * (0.8_f64).powf(1.5) / r;
        assert_relative_eq!(b, expected, max_relative = 1e-10);
    }

    #[test]
    fn helmholtz_field_nearly_uniform_near_center() {
        let hc = HelmholtzCoil::new(0.1, 1.0, 10);
        let b_center = hc.b_on_axis(0.0);
        let b_slight_off = hc.b_on_axis(0.01); // 10% of radius
        // Should be within 1% for Helmholtz condition
        let variation = ((b_slight_off - b_center) / b_center).abs();
        assert!(variation < 0.01, "field variation {variation} should be < 1% near center");
    }

    #[test]
    fn helmholtz_symmetric_about_midpoint() {
        let hc = HelmholtzCoil::new(0.1, 1.0, 5);
        let b_pos = hc.b_on_axis(0.02);
        let b_neg = hc.b_on_axis(-0.02);
        assert_relative_eq!(b_pos, b_neg, max_relative = 1e-10);
    }

    #[test]
    fn helmholtz_sample_on_axis_dimensions() {
        let hc = HelmholtzCoil::new(0.1, 1.0, 1);
        let (zs, bs) = hc.sample_on_axis((-0.1, 0.1), 50);
        assert_eq!(zs.len(), 50);
        assert_eq!(bs.len(), 50);
    }
}

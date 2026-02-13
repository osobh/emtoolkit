//! Polarization of electromagnetic waves.
//!
//! Linear, circular, and elliptical polarization states.
//! Poincaré sphere representation, axial ratio, tilt angle.

use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

/// Polarization state defined by two orthogonal component amplitudes and phase difference.
///
/// E_x = a_x cos(ωt)
/// E_y = a_y cos(ωt + δ)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PolarizationState {
    /// Amplitude of x-component
    pub ax: f64,
    /// Amplitude of y-component
    pub ay: f64,
    /// Phase difference δ = φ_y - φ_x (radians)
    pub delta: f64,
}

/// Classification of polarization type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolarizationType {
    Linear,
    Circular,
    Elliptical,
}

/// Rotation sense (handedness).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RotationSense {
    LeftHand,
    RightHand,
    None, // linear polarization
}

impl PolarizationState {
    pub fn new(ax: f64, ay: f64, delta: f64) -> Self {
        Self { ax, ay, delta }
    }

    /// Linear polarization along x.
    pub fn linear_x(amplitude: f64) -> Self {
        Self::new(amplitude, 0.0, 0.0)
    }

    /// Linear polarization along y.
    pub fn linear_y(amplitude: f64) -> Self {
        Self::new(0.0, amplitude, 0.0)
    }

    /// Linear polarization at angle θ from x-axis.
    pub fn linear_at_angle(amplitude: f64, theta: f64) -> Self {
        Self::new(amplitude * theta.cos(), amplitude * theta.sin(), 0.0)
    }

    /// Right-hand circular polarization.
    pub fn rhcp(amplitude: f64) -> Self {
        Self::new(amplitude, amplitude, -PI / 2.0)
    }

    /// Left-hand circular polarization.
    pub fn lhcp(amplitude: f64) -> Self {
        Self::new(amplitude, amplitude, PI / 2.0)
    }

    /// Classify the polarization type.
    pub fn polarization_type(&self) -> PolarizationType {
        let eps = 1e-10;
        if self.ax.abs() < eps || self.ay.abs() < eps {
            return PolarizationType::Linear;
        }
        let delta_mod = normalize_angle(self.delta);
        if delta_mod.abs() < eps || (delta_mod - PI).abs() < eps || (delta_mod + PI).abs() < eps {
            return PolarizationType::Linear;
        }
        if (self.ax - self.ay).abs() < eps * self.ax
            && ((delta_mod - PI / 2.0).abs() < eps || (delta_mod + PI / 2.0).abs() < eps)
        {
            return PolarizationType::Circular;
        }
        PolarizationType::Elliptical
    }

    /// Rotation sense (handedness).
    pub fn rotation_sense(&self) -> RotationSense {
        if self.polarization_type() == PolarizationType::Linear {
            return RotationSense::None;
        }
        if self.delta.sin() > 0.0 {
            RotationSense::LeftHand
        } else {
            RotationSense::RightHand
        }
    }

    /// Axial ratio (AR ≥ 1). AR = 1 for circular, ∞ for linear.
    pub fn axial_ratio(&self) -> f64 {
        let (a, b) = self.semi_axes();
        if b.abs() < 1e-15 {
            f64::INFINITY
        } else {
            a / b
        }
    }

    /// Semi-major and semi-minor axes of the polarization ellipse.
    pub fn semi_axes(&self) -> (f64, f64) {
        let ax2 = self.ax * self.ax;
        let ay2 = self.ay * self.ay;
        let cos_d = self.delta.cos();

        let sum = ax2 + ay2;
        let discriminant = ((ax2 - ay2).powi(2) + 4.0 * ax2 * ay2 * cos_d * cos_d).sqrt();

        let a = ((sum + discriminant) / 2.0).sqrt();
        let b = ((sum - discriminant) / 2.0).sqrt();
        (a, b)
    }

    /// Tilt angle of the polarization ellipse (radians).
    pub fn tilt_angle(&self) -> f64 {
        let ax2 = self.ax * self.ax;
        let ay2 = self.ay * self.ay;
        if (ax2 - ay2).abs() < 1e-15 && self.delta.cos().abs() < 1e-15 {
            return 0.0; // circular, tilt undefined
        }
        0.5 * (2.0 * self.ax * self.ay * self.delta.cos() / (ax2 - ay2)).atan()
    }

    /// Trace the polarization ellipse for visualization.
    ///
    /// Returns (x_points, y_points) for one period.
    pub fn trace_ellipse(&self, num_points: usize) -> (Vec<f64>, Vec<f64>) {
        assert!(num_points >= 4);
        let dt = 2.0 * PI / num_points as f64;
        let x: Vec<f64> = (0..num_points).map(|i| {
            let t = i as f64 * dt;
            self.ax * t.cos()
        }).collect();
        let y: Vec<f64> = (0..num_points).map(|i| {
            let t = i as f64 * dt;
            self.ay * (t + self.delta).cos()
        }).collect();
        (x, y)
    }

    /// Poincaré sphere coordinates (S₁, S₂, S₃) normalized by S₀.
    ///
    /// S₀ = ax² + ay²
    /// S₁ = ax² - ay²
    /// S₂ = 2·ax·ay·cos(δ)
    /// S₃ = 2·ax·ay·sin(δ)
    pub fn stokes_parameters(&self) -> [f64; 4] {
        let s0 = self.ax * self.ax + self.ay * self.ay;
        let s1 = self.ax * self.ax - self.ay * self.ay;
        let s2 = 2.0 * self.ax * self.ay * self.delta.cos();
        let s3 = 2.0 * self.ax * self.ay * self.delta.sin();
        [s0, s1, s2, s3]
    }

    /// Normalized Poincaré sphere coordinates.
    pub fn poincare_point(&self) -> [f64; 3] {
        let [s0, s1, s2, s3] = self.stokes_parameters();
        if s0.abs() < 1e-15 {
            [0.0, 0.0, 0.0]
        } else {
            [s1 / s0, s2 / s0, s3 / s0]
        }
    }
}

fn normalize_angle(a: f64) -> f64 {
    let mut r = a % (2.0 * PI);
    if r > PI {
        r -= 2.0 * PI;
    }
    if r < -PI {
        r += 2.0 * PI;
    }
    r
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn linear_x_classification() {
        let p = PolarizationState::linear_x(1.0);
        assert_eq!(p.polarization_type(), PolarizationType::Linear);
        assert_eq!(p.rotation_sense(), RotationSense::None);
    }

    #[test]
    fn linear_at_45_degrees() {
        let p = PolarizationState::linear_at_angle(1.0, PI / 4.0);
        assert_eq!(p.polarization_type(), PolarizationType::Linear);
        assert_relative_eq!(p.ax, p.ay, max_relative = 1e-10);
    }

    #[test]
    fn rhcp_classification() {
        let p = PolarizationState::rhcp(1.0);
        assert_eq!(p.polarization_type(), PolarizationType::Circular);
        assert_eq!(p.rotation_sense(), RotationSense::RightHand);
    }

    #[test]
    fn lhcp_classification() {
        let p = PolarizationState::lhcp(1.0);
        assert_eq!(p.polarization_type(), PolarizationType::Circular);
        assert_eq!(p.rotation_sense(), RotationSense::LeftHand);
    }

    #[test]
    fn circular_axial_ratio_is_one() {
        let p = PolarizationState::rhcp(1.0);
        assert_relative_eq!(p.axial_ratio(), 1.0, max_relative = 1e-6);
    }

    #[test]
    fn linear_axial_ratio_infinite() {
        let p = PolarizationState::linear_x(1.0);
        assert!(p.axial_ratio().is_infinite());
    }

    #[test]
    fn elliptical_axial_ratio_between_1_and_inf() {
        let p = PolarizationState::new(2.0, 1.0, PI / 4.0);
        let ar = p.axial_ratio();
        assert!(ar > 1.0 && ar < f64::INFINITY);
    }

    #[test]
    fn stokes_fully_polarized() {
        // S₀² = S₁² + S₂² + S₃² for fully polarized
        let p = PolarizationState::new(2.0, 1.0, PI / 3.0);
        let [s0, s1, s2, s3] = p.stokes_parameters();
        let norm = (s1 * s1 + s2 * s2 + s3 * s3).sqrt();
        assert_relative_eq!(norm, s0, max_relative = 1e-10);
    }

    #[test]
    fn poincare_rhcp_at_north_pole() {
        let p = PolarizationState::rhcp(1.0);
        let [_s1, _s2, s3] = p.poincare_point();
        assert_relative_eq!(s3, -1.0, max_relative = 1e-10);
    }

    #[test]
    fn poincare_lhcp_at_south_pole() {
        let p = PolarizationState::lhcp(1.0);
        let [_s1, _s2, s3] = p.poincare_point();
        assert_relative_eq!(s3, 1.0, max_relative = 1e-10);
    }

    #[test]
    fn poincare_linear_on_equator() {
        let p = PolarizationState::linear_x(1.0);
        let [_s1, _s2, s3] = p.poincare_point();
        assert_relative_eq!(s3, 0.0, epsilon = 1e-12);
    }

    #[test]
    fn trace_ellipse_dimensions() {
        let p = PolarizationState::new(2.0, 1.0, PI / 4.0);
        let (x, y) = p.trace_ellipse(100);
        assert_eq!(x.len(), 100);
        assert_eq!(y.len(), 100);
    }

    #[test]
    fn trace_circle_uniform_radius() {
        let p = PolarizationState::rhcp(1.0);
        let (x, y) = p.trace_ellipse(100);
        for i in 0..100 {
            let r = (x[i] * x[i] + y[i] * y[i]).sqrt();
            assert_relative_eq!(r, 1.0, max_relative = 0.01);
        }
    }

    #[test]
    fn tilt_angle_linear_x_is_zero() {
        let p = PolarizationState::linear_x(1.0);
        // For linear along x (ay=0), tilt is 0
        assert_relative_eq!(p.tilt_angle(), 0.0, epsilon = 1e-10);
    }
}

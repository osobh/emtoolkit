//! Forces between current-carrying conductors.
//!
//! Module 5.3: Force between parallel wires and force on a wire in external B-field.

use em_core::constants::MU_0;
use em_core::coordinates::Vector3;
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

/// Force per unit length between two infinite parallel wires.
///
/// F/L = μ₀ I₁ I₂ / (2π d)
///
/// Positive = repulsive (opposite currents), Negative = attractive (same direction).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ParallelWireForce {
    /// Current in wire 1 (A)
    pub i1: f64,
    /// Current in wire 2 (A)
    pub i2: f64,
    /// Separation distance (m)
    pub separation: f64,
}

impl ParallelWireForce {
    pub fn new(i1: f64, i2: f64, separation: f64) -> Self {
        assert!(separation > 0.0, "separation must be positive");
        Self { i1, i2, separation }
    }

    /// Force per unit length (N/m).
    ///
    /// Positive = attractive (same direction currents), negative = repulsive.
    pub fn force_per_length(&self) -> f64 {
        MU_0 * self.i1 * self.i2 / (2.0 * PI * self.separation)
    }

    /// Whether the force is attractive.
    pub fn is_attractive(&self) -> bool {
        self.i1 * self.i2 > 0.0
    }

    /// Total force for a given wire length (N).
    pub fn total_force(&self, length: f64) -> f64 {
        self.force_per_length() * length
    }
}

/// Force on a straight current-carrying wire in a uniform external B-field.
///
/// F = I L × B
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct WireInField {
    /// Current (A)
    pub current: f64,
    /// Wire direction and length (m)
    pub wire_vector: Vector3,
    /// External uniform B-field (T)
    pub b_field: Vector3,
}

impl WireInField {
    pub fn new(current: f64, wire_vector: Vector3, b_field: Vector3) -> Self {
        Self {
            current,
            wire_vector,
            b_field,
        }
    }

    /// Force vector on the wire (N).
    ///
    /// F = I (L × B)
    pub fn force(&self) -> Vector3 {
        let cross = self.wire_vector.cross(&self.b_field);
        Vector3::new(
            self.current * cross.x,
            self.current * cross.y,
            self.current * cross.z,
        )
    }

    /// Magnitude of the force (N).
    pub fn force_magnitude(&self) -> f64 {
        self.force().magnitude()
    }

    /// Torque on a rectangular current loop in uniform B-field.
    ///
    /// τ = m × B where m = I·A·n̂
    pub fn torque_on_loop(current: f64, area: f64, normal: Vector3, b_field: Vector3) -> Vector3 {
        let m = normal.normalized() * (current * area);
        m.cross(&b_field)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    // ================================================================
    // Parallel wire forces
    // ================================================================

    #[test]
    fn same_direction_attractive() {
        let pw = ParallelWireForce::new(1.0, 1.0, 0.1);
        assert!(pw.is_attractive());
        assert!(pw.force_per_length() > 0.0);
    }

    #[test]
    fn opposite_direction_repulsive() {
        let pw = ParallelWireForce::new(1.0, -1.0, 0.1);
        assert!(!pw.is_attractive());
        assert!(pw.force_per_length() < 0.0);
    }

    #[test]
    fn force_per_length_value() {
        // Two wires 1m apart, 1A each
        let pw = ParallelWireForce::new(1.0, 1.0, 1.0);
        let expected = MU_0 / (2.0 * PI);
        assert_relative_eq!(pw.force_per_length(), expected, max_relative = 1e-10);
    }

    #[test]
    fn force_inverse_distance() {
        let pw1 = ParallelWireForce::new(1.0, 1.0, 0.1);
        let pw2 = ParallelWireForce::new(1.0, 1.0, 0.2);
        assert_relative_eq!(pw1.force_per_length() / pw2.force_per_length(), 2.0, max_relative = 1e-10);
    }

    #[test]
    fn force_proportional_to_currents() {
        let pw1 = ParallelWireForce::new(1.0, 1.0, 0.1);
        let pw2 = ParallelWireForce::new(3.0, 2.0, 0.1);
        assert_relative_eq!(pw2.force_per_length() / pw1.force_per_length(), 6.0, max_relative = 1e-10);
    }

    #[test]
    fn total_force_scales_with_length() {
        let pw = ParallelWireForce::new(1.0, 1.0, 0.1);
        let f2 = pw.total_force(2.0);
        let f1 = pw.total_force(1.0);
        assert_relative_eq!(f2 / f1, 2.0, max_relative = 1e-10);
    }

    // ================================================================
    // Wire in field
    // ================================================================

    #[test]
    fn force_on_wire_perpendicular_to_field() {
        // Wire along x, B along z → F along -y (or +y depending on sign)
        let w = WireInField::new(
            1.0,
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(0.0, 0.0, 1.0),
        );
        let f = w.force();
        // (1,0,0) × (0,0,1) = (0·1-0·0, 0·0-1·1, 1·0-0·0) = (0, -1, 0)
        assert_relative_eq!(f.y, -1.0, epsilon = 1e-12);
        assert_relative_eq!(f.x, 0.0, epsilon = 1e-12);
        assert_relative_eq!(f.z, 0.0, epsilon = 1e-12);
    }

    #[test]
    fn force_on_wire_parallel_to_field_is_zero() {
        let w = WireInField::new(
            5.0,
            Vector3::new(0.0, 0.0, 1.0),
            Vector3::new(0.0, 0.0, 2.0),
        );
        assert_relative_eq!(w.force_magnitude(), 0.0, epsilon = 1e-12);
    }

    #[test]
    fn force_proportional_to_current() {
        let w1 = WireInField::new(1.0, Vector3::new(1.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0));
        let w2 = WireInField::new(3.0, Vector3::new(1.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0));
        assert_relative_eq!(w2.force_magnitude() / w1.force_magnitude(), 3.0, max_relative = 1e-10);
    }

    // ================================================================
    // Torque on loop
    // ================================================================

    #[test]
    fn torque_loop_perpendicular_to_field() {
        // Loop normal along x, B along z → τ along -y
        let tau = WireInField::torque_on_loop(
            1.0,
            0.01, // 1 cm²
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(0.0, 0.0, 1.0),
        );
        // m = (0.01, 0, 0), B = (0, 0, 1)
        // m × B = (0·1-0·0, 0·0-0.01·1, 0.01·0-0·0) = (0, -0.01, 0)
        assert_relative_eq!(tau.y, -0.01, epsilon = 1e-12);
    }

    #[test]
    fn torque_loop_aligned_with_field_is_zero() {
        let tau = WireInField::torque_on_loop(
            1.0,
            0.01,
            Vector3::new(0.0, 0.0, 1.0),
            Vector3::new(0.0, 0.0, 1.0),
        );
        assert_relative_eq!(tau.magnitude(), 0.0, epsilon = 1e-12);
    }

    #[test]
    #[should_panic]
    fn parallel_wire_zero_separation_panics() {
        ParallelWireForce::new(1.0, 1.0, 0.0);
    }
}

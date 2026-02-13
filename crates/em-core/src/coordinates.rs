//! Coordinate system representations and transforms for EM computations.
//!
//! Supports Cartesian (x, y, z), cylindrical (ρ, φ, z), and spherical (r, θ, φ) systems.
//! All angles are in radians. Conversions follow the physics convention where spherical θ
//! is the polar angle from the z-axis and φ is the azimuthal angle from the x-axis.

use crate::error::{EmCoreError, EmCoreResult};
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

/// A point in 3D Cartesian coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Cartesian {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// A point in cylindrical coordinates (ρ, φ, z).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Cylindrical {
    /// Radial distance from z-axis (ρ ≥ 0)
    pub rho: f64,
    /// Azimuthal angle from x-axis (radians)
    pub phi: f64,
    /// Height along z-axis
    pub z: f64,
}

/// A point in spherical coordinates (r, θ, φ).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Spherical {
    /// Radial distance from origin (r ≥ 0)
    pub r: f64,
    /// Polar angle from z-axis (0 ≤ θ ≤ π)
    pub theta: f64,
    /// Azimuthal angle from x-axis (radians)
    pub phi: f64,
}

/// A 3D vector in Cartesian components, usable for E-fields, H-fields, etc.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    /// Magnitude of the vector.
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Return unit vector in the same direction. Returns zero vector if magnitude is zero.
    pub fn normalized(&self) -> Self {
        let mag = self.magnitude();
        if mag == 0.0 {
            Self::zero()
        } else {
            Self {
                x: self.x / mag,
                y: self.y / mag,
                z: self.z / mag,
            }
        }
    }

    /// Dot product with another vector.
    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Cross product with another vector.
    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    /// Scale the vector by a scalar.
    pub fn scale(&self, s: f64) -> Self {
        Self {
            x: self.x * s,
            y: self.y * s,
            z: self.z * s,
        }
    }

    /// Add another vector.
    pub fn add(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    /// Subtract another vector.
    pub fn sub(&self, other: &Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl std::ops::Add for Vector3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::Sub for Vector3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::ops::Mul<f64> for Vector3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl std::ops::Neg for Vector3 {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

// ============================================================================
// Coordinate conversions
// ============================================================================

impl Cartesian {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// Convert to cylindrical coordinates.
    pub fn to_cylindrical(&self) -> Cylindrical {
        Cylindrical {
            rho: (self.x * self.x + self.y * self.y).sqrt(),
            phi: self.y.atan2(self.x),
            z: self.z,
        }
    }

    /// Convert to spherical coordinates.
    pub fn to_spherical(&self) -> Spherical {
        let r = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        let theta = if r == 0.0 {
            0.0
        } else {
            (self.z / r).acos()
        };
        let phi = self.y.atan2(self.x);
        Spherical { r, theta, phi }
    }

    /// Distance to another Cartesian point.
    pub fn distance_to(&self, other: &Self) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    /// Convert to a Vector3.
    pub fn to_vector3(&self) -> Vector3 {
        Vector3::new(self.x, self.y, self.z)
    }
}

impl Cylindrical {
    pub fn new(rho: f64, phi: f64, z: f64) -> EmCoreResult<Self> {
        if rho < 0.0 {
            return Err(EmCoreError::OutOfRange {
                name: "rho".into(),
                value: rho,
                expected: "ρ ≥ 0".into(),
            });
        }
        Ok(Self { rho, phi, z })
    }

    /// Convert to Cartesian coordinates.
    pub fn to_cartesian(&self) -> Cartesian {
        Cartesian {
            x: self.rho * self.phi.cos(),
            y: self.rho * self.phi.sin(),
            z: self.z,
        }
    }

    /// Convert to spherical coordinates.
    pub fn to_spherical(&self) -> Spherical {
        self.to_cartesian().to_spherical()
    }
}

impl Spherical {
    pub fn new(r: f64, theta: f64, phi: f64) -> EmCoreResult<Self> {
        if r < 0.0 {
            return Err(EmCoreError::OutOfRange {
                name: "r".into(),
                value: r,
                expected: "r ≥ 0".into(),
            });
        }
        if !(0.0..=PI).contains(&theta) {
            return Err(EmCoreError::OutOfRange {
                name: "theta".into(),
                value: theta,
                expected: "0 ≤ θ ≤ π".into(),
            });
        }
        Ok(Self { r, theta, phi })
    }

    /// Convert to Cartesian coordinates.
    pub fn to_cartesian(&self) -> Cartesian {
        Cartesian {
            x: self.r * self.theta.sin() * self.phi.cos(),
            y: self.r * self.theta.sin() * self.phi.sin(),
            z: self.r * self.theta.cos(),
        }
    }

    /// Convert to cylindrical coordinates.
    pub fn to_cylindrical(&self) -> Cylindrical {
        Cylindrical {
            rho: self.r * self.theta.sin(),
            phi: self.phi,
            z: self.r * self.theta.cos(),
        }
    }
}

/// Transform a vector field from spherical (r̂, θ̂, φ̂) components at a given point
/// to Cartesian (x̂, ŷ, ẑ) components.
///
/// # Arguments
/// * `v_r`, `v_theta`, `v_phi` - Vector components in spherical basis
/// * `theta` - Polar angle of the evaluation point
/// * `phi` - Azimuthal angle of the evaluation point
pub fn spherical_to_cartesian_vector(
    v_r: f64,
    v_theta: f64,
    v_phi: f64,
    theta: f64,
    phi: f64,
) -> Vector3 {
    let st = theta.sin();
    let ct = theta.cos();
    let sp = phi.sin();
    let cp = phi.cos();

    Vector3 {
        x: v_r * st * cp + v_theta * ct * cp - v_phi * sp,
        y: v_r * st * sp + v_theta * ct * sp + v_phi * cp,
        z: v_r * ct - v_theta * st,
    }
}

/// Transform a vector field from cylindrical (ρ̂, φ̂, ẑ) components at a given point
/// to Cartesian (x̂, ŷ, ẑ) components.
///
/// # Arguments
/// * `v_rho`, `v_phi`, `v_z` - Vector components in cylindrical basis
/// * `phi` - Azimuthal angle of the evaluation point
pub fn cylindrical_to_cartesian_vector(v_rho: f64, v_phi: f64, v_z: f64, phi: f64) -> Vector3 {
    let cp = phi.cos();
    let sp = phi.sin();

    Vector3 {
        x: v_rho * cp - v_phi * sp,
        y: v_rho * sp + v_phi * cp,
        z: v_z,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use std::f64::consts::{FRAC_PI_2, FRAC_PI_4, PI};

    // ================================================================
    // Vector3 basic operations
    // ================================================================

    #[test]
    fn vector3_magnitude() {
        let v = Vector3::new(3.0, 4.0, 0.0);
        assert_relative_eq!(v.magnitude(), 5.0, epsilon = 1e-12);
    }

    #[test]
    fn vector3_magnitude_3d() {
        let v = Vector3::new(1.0, 2.0, 2.0);
        assert_relative_eq!(v.magnitude(), 3.0, epsilon = 1e-12);
    }

    #[test]
    fn vector3_zero_magnitude() {
        assert_relative_eq!(Vector3::zero().magnitude(), 0.0, epsilon = 1e-12);
    }

    #[test]
    fn vector3_normalized() {
        let v = Vector3::new(0.0, 0.0, 5.0);
        let n = v.normalized();
        assert_relative_eq!(n.z, 1.0, epsilon = 1e-12);
        assert_relative_eq!(n.magnitude(), 1.0, epsilon = 1e-12);
    }

    #[test]
    fn vector3_normalized_zero_returns_zero() {
        let n = Vector3::zero().normalized();
        assert_relative_eq!(n.magnitude(), 0.0, epsilon = 1e-12);
    }

    #[test]
    fn vector3_dot_product() {
        let a = Vector3::new(1.0, 0.0, 0.0);
        let b = Vector3::new(0.0, 1.0, 0.0);
        assert_relative_eq!(a.dot(&b), 0.0, epsilon = 1e-12);
    }

    #[test]
    fn vector3_dot_product_parallel() {
        let a = Vector3::new(2.0, 3.0, 4.0);
        let b = Vector3::new(2.0, 3.0, 4.0);
        assert_relative_eq!(a.dot(&b), 29.0, epsilon = 1e-12);
    }

    #[test]
    fn vector3_cross_product_orthogonal_basis() {
        let x = Vector3::new(1.0, 0.0, 0.0);
        let y = Vector3::new(0.0, 1.0, 0.0);
        let z = x.cross(&y);
        assert_relative_eq!(z.x, 0.0, epsilon = 1e-12);
        assert_relative_eq!(z.y, 0.0, epsilon = 1e-12);
        assert_relative_eq!(z.z, 1.0, epsilon = 1e-12);
    }

    #[test]
    fn vector3_cross_product_anticommutative() {
        let a = Vector3::new(1.0, 2.0, 3.0);
        let b = Vector3::new(4.0, 5.0, 6.0);
        let ab = a.cross(&b);
        let ba = b.cross(&a);
        assert_relative_eq!(ab.x, -ba.x, epsilon = 1e-12);
        assert_relative_eq!(ab.y, -ba.y, epsilon = 1e-12);
        assert_relative_eq!(ab.z, -ba.z, epsilon = 1e-12);
    }

    #[test]
    fn vector3_ops_add() {
        let a = Vector3::new(1.0, 2.0, 3.0);
        let b = Vector3::new(4.0, 5.0, 6.0);
        let c = a + b;
        assert_relative_eq!(c.x, 5.0, epsilon = 1e-12);
        assert_relative_eq!(c.y, 7.0, epsilon = 1e-12);
        assert_relative_eq!(c.z, 9.0, epsilon = 1e-12);
    }

    #[test]
    fn vector3_ops_sub() {
        let a = Vector3::new(4.0, 5.0, 6.0);
        let b = Vector3::new(1.0, 2.0, 3.0);
        let c = a - b;
        assert_relative_eq!(c.x, 3.0, epsilon = 1e-12);
        assert_relative_eq!(c.y, 3.0, epsilon = 1e-12);
        assert_relative_eq!(c.z, 3.0, epsilon = 1e-12);
    }

    #[test]
    fn vector3_ops_mul_scalar() {
        let v = Vector3::new(1.0, 2.0, 3.0) * 2.0;
        assert_relative_eq!(v.x, 2.0, epsilon = 1e-12);
        assert_relative_eq!(v.y, 4.0, epsilon = 1e-12);
        assert_relative_eq!(v.z, 6.0, epsilon = 1e-12);
    }

    #[test]
    fn vector3_ops_neg() {
        let v = -Vector3::new(1.0, -2.0, 3.0);
        assert_relative_eq!(v.x, -1.0, epsilon = 1e-12);
        assert_relative_eq!(v.y, 2.0, epsilon = 1e-12);
        assert_relative_eq!(v.z, -3.0, epsilon = 1e-12);
    }

    // ================================================================
    // Cartesian ↔ Cylindrical
    // ================================================================

    #[test]
    fn cartesian_to_cylindrical_on_x_axis() {
        let c = Cartesian::new(5.0, 0.0, 3.0);
        let cyl = c.to_cylindrical();
        assert_relative_eq!(cyl.rho, 5.0, epsilon = 1e-12);
        assert_relative_eq!(cyl.phi, 0.0, epsilon = 1e-12);
        assert_relative_eq!(cyl.z, 3.0, epsilon = 1e-12);
    }

    #[test]
    fn cartesian_to_cylindrical_on_y_axis() {
        let c = Cartesian::new(0.0, 4.0, 0.0);
        let cyl = c.to_cylindrical();
        assert_relative_eq!(cyl.rho, 4.0, epsilon = 1e-12);
        assert_relative_eq!(cyl.phi, FRAC_PI_2, epsilon = 1e-12);
    }

    #[test]
    fn cylindrical_to_cartesian_roundtrip() {
        let original = Cartesian::new(3.0, 4.0, 5.0);
        let cyl = original.to_cylindrical();
        let back = cyl.to_cartesian();
        assert_relative_eq!(back.x, original.x, epsilon = 1e-12);
        assert_relative_eq!(back.y, original.y, epsilon = 1e-12);
        assert_relative_eq!(back.z, original.z, epsilon = 1e-12);
    }

    #[test]
    fn cylindrical_negative_rho_rejected() {
        let result = Cylindrical::new(-1.0, 0.0, 0.0);
        assert!(result.is_err());
    }

    // ================================================================
    // Cartesian ↔ Spherical
    // ================================================================

    #[test]
    fn cartesian_to_spherical_on_z_axis() {
        let c = Cartesian::new(0.0, 0.0, 5.0);
        let s = c.to_spherical();
        assert_relative_eq!(s.r, 5.0, epsilon = 1e-12);
        assert_relative_eq!(s.theta, 0.0, epsilon = 1e-12);
    }

    #[test]
    fn cartesian_to_spherical_on_x_axis() {
        let c = Cartesian::new(3.0, 0.0, 0.0);
        let s = c.to_spherical();
        assert_relative_eq!(s.r, 3.0, epsilon = 1e-12);
        assert_relative_eq!(s.theta, FRAC_PI_2, epsilon = 1e-12);
        assert_relative_eq!(s.phi, 0.0, epsilon = 1e-12);
    }

    #[test]
    fn spherical_to_cartesian_roundtrip() {
        let original = Cartesian::new(1.0, 2.0, 3.0);
        let sph = original.to_spherical();
        let back = sph.to_cartesian();
        assert_relative_eq!(back.x, original.x, epsilon = 1e-10);
        assert_relative_eq!(back.y, original.y, epsilon = 1e-10);
        assert_relative_eq!(back.z, original.z, epsilon = 1e-10);
    }

    #[test]
    fn spherical_origin_has_zero_r() {
        let c = Cartesian::new(0.0, 0.0, 0.0);
        let s = c.to_spherical();
        assert_relative_eq!(s.r, 0.0, epsilon = 1e-12);
    }

    #[test]
    fn spherical_negative_r_rejected() {
        let result = Spherical::new(-1.0, 0.0, 0.0);
        assert!(result.is_err());
    }

    #[test]
    fn spherical_theta_out_of_range_rejected() {
        assert!(Spherical::new(1.0, -0.1, 0.0).is_err());
        assert!(Spherical::new(1.0, PI + 0.1, 0.0).is_err());
    }

    // ================================================================
    // Cylindrical ↔ Spherical
    // ================================================================

    #[test]
    fn cylindrical_to_spherical_roundtrip() {
        let original = Cartesian::new(2.0, 3.0, 4.0);
        let cyl = original.to_cylindrical();
        let sph = cyl.to_spherical();
        let back = sph.to_cartesian();
        assert_relative_eq!(back.x, original.x, epsilon = 1e-10);
        assert_relative_eq!(back.y, original.y, epsilon = 1e-10);
        assert_relative_eq!(back.z, original.z, epsilon = 1e-10);
    }

    #[test]
    fn spherical_to_cylindrical() {
        let sph = Spherical {
            r: 5.0,
            theta: FRAC_PI_4,
            phi: 0.0,
        };
        let cyl = sph.to_cylindrical();
        assert_relative_eq!(cyl.rho, 5.0 * FRAC_PI_4.sin(), epsilon = 1e-12);
        assert_relative_eq!(cyl.z, 5.0 * FRAC_PI_4.cos(), epsilon = 1e-12);
    }

    // ================================================================
    // Distance
    // ================================================================

    #[test]
    fn distance_between_cartesian_points() {
        let a = Cartesian::new(0.0, 0.0, 0.0);
        let b = Cartesian::new(3.0, 4.0, 0.0);
        assert_relative_eq!(a.distance_to(&b), 5.0, epsilon = 1e-12);
    }

    #[test]
    fn distance_to_self_is_zero() {
        let a = Cartesian::new(1.0, 2.0, 3.0);
        assert_relative_eq!(a.distance_to(&a), 0.0, epsilon = 1e-12);
    }

    // ================================================================
    // Vector field transforms
    // ================================================================

    #[test]
    fn spherical_r_hat_at_pole_is_z_hat() {
        // At θ=0 (north pole), r̂ = ẑ
        let v = spherical_to_cartesian_vector(1.0, 0.0, 0.0, 0.0, 0.0);
        assert_relative_eq!(v.x, 0.0, epsilon = 1e-12);
        assert_relative_eq!(v.y, 0.0, epsilon = 1e-12);
        assert_relative_eq!(v.z, 1.0, epsilon = 1e-12);
    }

    #[test]
    fn spherical_r_hat_at_equator_phi_0_is_x_hat() {
        // At θ=π/2, φ=0: r̂ = x̂
        let v = spherical_to_cartesian_vector(1.0, 0.0, 0.0, FRAC_PI_2, 0.0);
        assert_relative_eq!(v.x, 1.0, epsilon = 1e-12);
        assert_relative_eq!(v.y, 0.0, epsilon = 1e-12);
        assert_relative_eq!(v.z, 0.0, epsilon = 1e-12);
    }

    #[test]
    fn cylindrical_rho_hat_at_phi_0_is_x_hat() {
        let v = cylindrical_to_cartesian_vector(1.0, 0.0, 0.0, 0.0);
        assert_relative_eq!(v.x, 1.0, epsilon = 1e-12);
        assert_relative_eq!(v.y, 0.0, epsilon = 1e-12);
        assert_relative_eq!(v.z, 0.0, epsilon = 1e-12);
    }

    #[test]
    fn cylindrical_phi_hat_at_phi_0_is_y_hat() {
        let v = cylindrical_to_cartesian_vector(0.0, 1.0, 0.0, 0.0);
        assert_relative_eq!(v.x, 0.0, epsilon = 1e-12);
        assert_relative_eq!(v.y, 1.0, epsilon = 1e-12);
        assert_relative_eq!(v.z, 0.0, epsilon = 1e-12);
    }

    #[test]
    fn cylindrical_z_hat_unchanged() {
        let v = cylindrical_to_cartesian_vector(0.0, 0.0, 7.0, 1.234);
        assert_relative_eq!(v.x, 0.0, epsilon = 1e-12);
        assert_relative_eq!(v.y, 0.0, epsilon = 1e-12);
        assert_relative_eq!(v.z, 7.0, epsilon = 1e-12);
    }
}

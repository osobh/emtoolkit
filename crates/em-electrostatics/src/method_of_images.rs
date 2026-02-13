//! Method of images for electrostatic problems.
//!
//! Implements image charge configurations for:
//! - Point charge above an infinite conducting plane
//! - Point charge near a grounded conducting sphere

use super::point_charges::{PointCharge, electric_field, electric_potential};
use em_core::constants::EPSILON_0;
use em_core::coordinates::{Cartesian, Vector3};
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

/// Configuration for a charge above a conducting plane.
///
/// The conducting plane is at z = 0 (the xy-plane).
/// The real charge is at height h above the plane.
/// The image charge is at -h below the plane with opposite sign.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ChargeAbovePlane {
    /// Real charge value (C)
    pub charge: f64,
    /// Height of charge above conducting plane (m)
    pub height: f64,
    /// x position of charge
    pub x: f64,
    /// y position of charge
    pub y: f64,
}

impl ChargeAbovePlane {
    pub fn new(charge: f64, height: f64) -> Self {
        Self {
            charge,
            height,
            x: 0.0,
            y: 0.0,
        }
    }

    pub fn at_position(charge: f64, x: f64, y: f64, height: f64) -> Self {
        Self {
            charge,
            height,
            x,
            y,
        }
    }

    /// Get the real charge as a PointCharge.
    pub fn real_charge(&self) -> PointCharge {
        PointCharge::new(self.x, self.y, self.height, self.charge)
    }

    /// Get the image charge as a PointCharge.
    pub fn image_charge(&self) -> PointCharge {
        PointCharge::new(self.x, self.y, -self.height, -self.charge)
    }

    /// Get both real and image charges for field computation.
    pub fn charge_system(&self) -> [PointCharge; 2] {
        [self.real_charge(), self.image_charge()]
    }

    /// Compute the electric field at a point above the plane (z > 0).
    ///
    /// Uses superposition of real + image charges.
    pub fn field_at(&self, point: &Cartesian) -> Vector3 {
        let system = self.charge_system();
        electric_field(&system, point, EPSILON_0)
    }

    /// Compute the electric potential at a point above the plane (z > 0).
    pub fn potential_at(&self, point: &Cartesian) -> f64 {
        let system = self.charge_system();
        electric_potential(&system, point, EPSILON_0)
    }

    /// Compute the induced surface charge density σ at a point on the conducting plane.
    ///
    /// σ = -ε₀ · E_z(x, y, 0⁺) = -q·h / (2π·(ρ² + h²)^(3/2))
    /// where ρ² = (x-x₀)² + (y-y₀)²
    pub fn surface_charge_density(&self, x: f64, y: f64) -> f64 {
        let rho_sq = (x - self.x).powi(2) + (y - self.y).powi(2);
        let denom = (rho_sq + self.height * self.height).powf(1.5);
        -self.charge * self.height / (2.0 * PI * denom)
    }

    /// Total induced charge on the conducting plane (should equal -q).
    ///
    /// This is an analytical result, not numerical integration.
    pub fn total_induced_charge(&self) -> f64 {
        -self.charge
    }

    /// Force on the charge due to the conducting plane (attraction).
    ///
    /// F = -q² / (4πε₀ · (2h)²) in the -z direction
    pub fn force_on_charge(&self) -> Vector3 {
        let f_mag = self.charge * self.charge / (4.0 * PI * EPSILON_0 * (2.0 * self.height).powi(2));
        Vector3::new(0.0, 0.0, -f_mag) // attractive → toward plane
    }
}

/// Configuration for a charge near a grounded conducting sphere.
///
/// Sphere is centered at origin with radius a.
/// Real charge q is at distance d from center (d > a).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ChargeNearSphere {
    /// Real charge value (C)
    pub charge: f64,
    /// Sphere radius (m)
    pub sphere_radius: f64,
    /// Distance from sphere center to real charge (m)
    pub distance: f64,
}

impl ChargeNearSphere {
    pub fn new(charge: f64, sphere_radius: f64, distance: f64) -> Self {
        assert!(
            distance > sphere_radius,
            "charge must be outside sphere: d > a"
        );
        Self {
            charge,
            sphere_radius,
            distance,
        }
    }

    /// Image charge magnitude: q' = -q·a/d
    pub fn image_charge_value(&self) -> f64 {
        -self.charge * self.sphere_radius / self.distance
    }

    /// Image charge distance from center: d' = a²/d
    pub fn image_distance(&self) -> f64 {
        self.sphere_radius * self.sphere_radius / self.distance
    }

    /// Get the real charge (along +x axis at distance d).
    pub fn real_charge(&self) -> PointCharge {
        PointCharge::new(self.distance, 0.0, 0.0, self.charge)
    }

    /// Get the image charge (along +x axis at distance a²/d).
    pub fn image_charge(&self) -> PointCharge {
        PointCharge::new(self.image_distance(), 0.0, 0.0, self.image_charge_value())
    }

    /// Get both charges for field computation.
    pub fn charge_system(&self) -> [PointCharge; 2] {
        [self.real_charge(), self.image_charge()]
    }

    /// Compute the electric field at a point outside the sphere.
    pub fn field_at(&self, point: &Cartesian) -> Vector3 {
        let system = self.charge_system();
        electric_field(&system, point, EPSILON_0)
    }

    /// Compute the potential at a point outside the sphere.
    pub fn potential_at(&self, point: &Cartesian) -> f64 {
        let system = self.charge_system();
        electric_potential(&system, point, EPSILON_0)
    }

    /// Force on the charge due to the sphere (attraction).
    pub fn force_on_charge(&self) -> f64 {
        let q = self.charge;
        let d = self.distance;
        // F = q·q'/(4πε₀·(d-d')²) — attractive
        let q_prime = self.image_charge_value();
        let d_prime = self.image_distance();
        let separation = d - d_prime;
        q * q_prime / (4.0 * PI * EPSILON_0 * separation * separation)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    // ================================================================
    // Charge above conducting plane
    // ================================================================

    #[test]
    fn image_charge_opposite_sign() {
        let p = ChargeAbovePlane::new(1e-9, 0.1);
        assert_relative_eq!(p.image_charge().charge, -1e-9, epsilon = 1e-25);
    }

    #[test]
    fn image_charge_mirrored_position() {
        let p = ChargeAbovePlane::new(1e-9, 0.1);
        let real = p.real_charge();
        let image = p.image_charge();
        assert_relative_eq!(image.position.z, -real.position.z, epsilon = 1e-15);
        assert_relative_eq!(image.position.x, real.position.x, epsilon = 1e-15);
    }

    #[test]
    fn potential_on_plane_is_zero() {
        let p = ChargeAbovePlane::new(1e-9, 0.1);
        // V at any point on z=0 should be zero
        for x in [-0.5, 0.0, 0.3, 1.0] {
            let v = p.potential_at(&Cartesian::new(x, 0.0, 0.0));
            assert_relative_eq!(v, 0.0, epsilon = 1e-10);
        }
    }

    #[test]
    fn field_normal_to_plane_on_surface() {
        let p = ChargeAbovePlane::new(1e-9, 0.1);
        // On the conducting surface (z=0), E should be purely in z-direction
        // (tangential component is zero on conductor)
        let e = p.field_at(&Cartesian::new(0.5, 0.0, 1e-10));
        // Due to symmetry at x=0.5, y=0: Ex should be small compared to Ez
        // Actually no — at off-axis points, Ex is nonzero in the physical field
        // but the tangential component should vanish AT z=0
        // Let's test at the surface directly below the charge
        let e_below = p.field_at(&Cartesian::new(0.0, 0.0, 1e-10));
        assert_relative_eq!(e_below.x, 0.0, epsilon = 1e-5);
        assert_relative_eq!(e_below.y, 0.0, epsilon = 1e-5);
        assert!(e_below.z < 0.0, "field should point into conductor (downward)");
    }

    #[test]
    fn surface_charge_density_peaks_below_charge() {
        let p = ChargeAbovePlane::new(1e-9, 0.1);
        let sigma_0 = p.surface_charge_density(0.0, 0.0);
        let sigma_far = p.surface_charge_density(1.0, 0.0);
        assert!(sigma_0.abs() > sigma_far.abs(), "σ should peak below charge");
    }

    #[test]
    fn total_induced_charge_equals_minus_q() {
        let p = ChargeAbovePlane::new(1e-9, 0.1);
        assert_relative_eq!(p.total_induced_charge(), -1e-9, epsilon = 1e-25);
    }

    #[test]
    fn force_attractive_toward_plane() {
        let p = ChargeAbovePlane::new(1e-9, 0.1);
        let f = p.force_on_charge();
        assert!(f.z < 0.0, "force should be attractive (toward plane)");
    }

    #[test]
    fn force_increases_closer_to_plane() {
        let p1 = ChargeAbovePlane::new(1e-9, 0.1);
        let p2 = ChargeAbovePlane::new(1e-9, 0.05);
        let f1 = p1.force_on_charge().magnitude();
        let f2 = p2.force_on_charge().magnitude();
        assert!(f2 > f1, "force should increase closer to plane");
    }

    // ================================================================
    // Charge near conducting sphere
    // ================================================================

    #[test]
    fn sphere_image_charge_magnitude() {
        // q' = -q·a/d
        let s = ChargeNearSphere::new(1e-9, 0.1, 0.3);
        let expected = -1e-9 * 0.1 / 0.3;
        assert_relative_eq!(s.image_charge_value(), expected, max_relative = 1e-10);
    }

    #[test]
    fn sphere_image_distance() {
        // d' = a²/d
        let s = ChargeNearSphere::new(1e-9, 0.1, 0.3);
        let expected = 0.01 / 0.3;
        assert_relative_eq!(s.image_distance(), expected, max_relative = 1e-10);
    }

    #[test]
    fn sphere_image_inside_sphere() {
        let s = ChargeNearSphere::new(1e-9, 0.1, 0.3);
        assert!(
            s.image_distance() < s.sphere_radius,
            "image must be inside sphere"
        );
    }

    #[test]
    fn sphere_potential_on_surface_approximately_zero() {
        let s = ChargeNearSphere::new(1e-9, 0.1, 0.3);
        // Check potential at various points on sphere surface
        let a = s.sphere_radius;
        for angle in [0.0_f64, 0.5, 1.0, 2.0, 3.0] {
            let pt = Cartesian::new(a * angle.cos(), a * angle.sin(), 0.0);
            let v = s.potential_at(&pt);
            assert_relative_eq!(v, 0.0, epsilon = 0.5);
        }
    }

    #[test]
    fn sphere_force_attractive() {
        let s = ChargeNearSphere::new(1e-9, 0.1, 0.3);
        let f = s.force_on_charge();
        // Positive charge near grounded sphere → attractive (negative force)
        assert!(f < 0.0, "force should be attractive");
    }

    #[test]
    #[should_panic]
    fn sphere_charge_inside_panics() {
        ChargeNearSphere::new(1e-9, 0.1, 0.05); // d < a → should panic
    }
}

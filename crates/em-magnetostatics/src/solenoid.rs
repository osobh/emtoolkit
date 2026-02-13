//! Solenoid and toroid magnetic field computation.
//!
//! Module 5.4: Ampère's law applications — solenoids, toroids, coaxial cables.

use em_core::constants::MU_0;
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

/// An ideal solenoid (long, tightly wound).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Solenoid {
    /// Number of turns
    pub turns: usize,
    /// Length of solenoid (m)
    pub length: f64,
    /// Current (A)
    pub current: f64,
    /// Radius (m) — used for inductance calculation
    pub radius: f64,
    /// Relative permeability of core material
    pub mu_r: f64,
}

impl Solenoid {
    pub fn new(turns: usize, length: f64, current: f64, radius: f64) -> Self {
        Self {
            turns,
            length,
            current,
            radius,
            mu_r: 1.0,
        }
    }

    pub fn with_core(mut self, mu_r: f64) -> Self {
        self.mu_r = mu_r;
        self
    }

    /// Turns per unit length: n = N/L
    pub fn turns_per_length(&self) -> f64 {
        self.turns as f64 / self.length
    }

    /// Interior B-field magnitude (ideal, uniform inside).
    ///
    /// B = μ₀ μᵣ n I
    pub fn b_interior(&self) -> f64 {
        MU_0 * self.mu_r * self.turns_per_length() * self.current
    }

    /// Self-inductance.
    ///
    /// L = μ₀ μᵣ N² A / l
    pub fn inductance(&self) -> f64 {
        let a = PI * self.radius * self.radius;
        MU_0 * self.mu_r * (self.turns as f64).powi(2) * a / self.length
    }

    /// Energy stored in the magnetic field.
    ///
    /// W = ½ L I²
    pub fn stored_energy(&self) -> f64 {
        0.5 * self.inductance() * self.current * self.current
    }

    /// Magnetic field energy density inside the solenoid.
    ///
    /// u = B²/(2μ₀μᵣ)
    pub fn energy_density(&self) -> f64 {
        let b = self.b_interior();
        b * b / (2.0 * MU_0 * self.mu_r)
    }

    /// On-axis B-field at position z from center using the finite solenoid formula.
    ///
    /// B(z) = (μ₀ μᵣ n I / 2) [cos(θ₁) - cos(θ₂)]
    /// where θ₁, θ₂ are angles from the axis to the ends.
    pub fn b_on_axis(&self, z: f64) -> f64 {
        let n = self.turns_per_length();
        let half_l = self.length / 2.0;
        let z1 = z + half_l; // distance to left end
        let z2 = z - half_l; // distance to right end (negative if inside)

        let cos1 = z1 / (z1 * z1 + self.radius * self.radius).sqrt();
        let cos2 = z2 / (z2 * z2 + self.radius * self.radius).sqrt();

        MU_0 * self.mu_r * n * self.current * (cos1 - cos2) / 2.0
    }
}

/// An ideal toroid (torus-shaped solenoid).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Toroid {
    /// Number of turns
    pub turns: usize,
    /// Inner radius (m)
    pub inner_radius: f64,
    /// Outer radius (m)
    pub outer_radius: f64,
    /// Current (A)
    pub current: f64,
    /// Relative permeability of core
    pub mu_r: f64,
}

impl Toroid {
    pub fn new(turns: usize, inner_radius: f64, outer_radius: f64, current: f64) -> Self {
        assert!(inner_radius < outer_radius, "inner < outer required");
        Self {
            turns,
            inner_radius,
            outer_radius,
            current,
            mu_r: 1.0,
        }
    }

    pub fn with_core(mut self, mu_r: f64) -> Self {
        self.mu_r = mu_r;
        self
    }

    /// Mean radius: (a + b) / 2
    pub fn mean_radius(&self) -> f64 {
        (self.inner_radius + self.outer_radius) / 2.0
    }

    /// B-field inside the toroid at radius r from the center.
    ///
    /// B = μ₀ μᵣ N I / (2π r) for inner_radius < r < outer_radius
    /// B = 0 outside
    pub fn b_at_radius(&self, r: f64) -> f64 {
        if r < self.inner_radius || r > self.outer_radius {
            0.0
        } else {
            MU_0 * self.mu_r * self.turns as f64 * self.current / (2.0 * PI * r)
        }
    }

    /// B-field at the mean radius.
    pub fn b_mean(&self) -> f64 {
        self.b_at_radius(self.mean_radius())
    }

    /// Cross-sectional area (rectangular approximation).
    ///
    /// A = h × (b - a) where h is the height. For circular cross-section: A = π((b-a)/2)²
    /// Using rectangular with h = b - a for simplicity.
    pub fn cross_section_area(&self) -> f64 {
        let width = self.outer_radius - self.inner_radius;
        width * width // square cross-section approximation
    }

    /// Self-inductance of the toroid.
    ///
    /// L = μ₀ μᵣ N² h ln(b/a) / (2π) (for rectangular cross-section with h = b-a)
    pub fn inductance(&self) -> f64 {
        let h = self.outer_radius - self.inner_radius;
        MU_0 * self.mu_r * (self.turns as f64).powi(2) * h
            * (self.outer_radius / self.inner_radius).ln()
            / (2.0 * PI)
    }
}

/// Magnetic field inside a coaxial cable.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct CoaxialCable {
    /// Inner conductor radius (m)
    pub inner_radius: f64,
    /// Outer conductor inner radius (m)
    pub outer_inner_radius: f64,
    /// Outer conductor outer radius (m)
    pub outer_outer_radius: f64,
    /// Current in inner conductor (A), return current in outer conductor
    pub current: f64,
}

impl CoaxialCable {
    pub fn new(inner_radius: f64, outer_inner: f64, outer_outer: f64, current: f64) -> Self {
        assert!(inner_radius < outer_inner && outer_inner < outer_outer);
        Self {
            inner_radius,
            outer_inner_radius: outer_inner,
            outer_outer_radius: outer_outer,
            current,
        }
    }

    /// B-field magnitude at radius r from the center axis.
    pub fn b_at_radius(&self, r: f64) -> f64 {
        let a = self.inner_radius;
        let b = self.outer_inner_radius;
        let c = self.outer_outer_radius;
        let i = self.current;

        if r < 0.0 {
            0.0
        } else if r <= a {
            // Inside inner conductor: B = μ₀ I r / (2π a²)
            MU_0 * i * r / (2.0 * PI * a * a)
        } else if r <= b {
            // Between conductors: B = μ₀ I / (2π r)
            MU_0 * i / (2.0 * PI * r)
        } else if r <= c {
            // Inside outer conductor: current decreases
            let fraction = (r * r - b * b) / (c * c - b * b);
            let enclosed = i * (1.0 - fraction);
            MU_0 * enclosed / (2.0 * PI * r)
        } else {
            // Outside: B = 0 (equal and opposite currents)
            0.0
        }
    }

    /// Inductance per unit length.
    ///
    /// L/l = μ₀ ln(b/a) / (2π)
    pub fn inductance_per_length(&self) -> f64 {
        MU_0 * (self.outer_inner_radius / self.inner_radius).ln() / (2.0 * PI)
    }

    /// Sample B-field vs radius for visualization.
    pub fn sample_b_vs_r(&self, r_max: f64, num_points: usize) -> (Vec<f64>, Vec<f64>) {
        assert!(num_points >= 2);
        let dr = r_max / (num_points - 1) as f64;
        let r_vals: Vec<f64> = (0..num_points).map(|i| i as f64 * dr).collect();
        let b_vals: Vec<f64> = r_vals.iter().map(|&r| self.b_at_radius(r)).collect();
        (r_vals, b_vals)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    // ================================================================
    // Solenoid tests
    // ================================================================

    #[test]
    fn solenoid_b_interior() {
        let s = Solenoid::new(1000, 0.5, 2.0, 0.02);
        let n = 2000.0; // N/L
        let expected = MU_0 * n * 2.0;
        assert_relative_eq!(s.b_interior(), expected, max_relative = 1e-10);
    }

    #[test]
    fn solenoid_b_with_core() {
        let s = Solenoid::new(1000, 0.5, 1.0, 0.02).with_core(200.0);
        let s_air = Solenoid::new(1000, 0.5, 1.0, 0.02);
        assert_relative_eq!(s.b_interior() / s_air.b_interior(), 200.0, max_relative = 1e-10);
    }

    #[test]
    fn solenoid_on_axis_center_matches_interior() {
        let s = Solenoid::new(10000, 10.0, 1.0, 0.01); // very long
        let b_center = s.b_on_axis(0.0);
        let b_ideal = s.b_interior();
        assert_relative_eq!(b_center, b_ideal, max_relative = 0.01);
    }

    #[test]
    fn solenoid_on_axis_falls_off_at_ends() {
        let s = Solenoid::new(1000, 1.0, 1.0, 0.02);
        let b_center = s.b_on_axis(0.0);
        let b_end = s.b_on_axis(0.5); // at the end
        // B at end ≈ B_center/2 for ideal solenoid
        assert_relative_eq!(b_end / b_center, 0.5, max_relative = 0.05);
    }

    #[test]
    fn solenoid_inductance() {
        // L = μ₀ N² A / l
        let s = Solenoid::new(100, 0.1, 1.0, 0.01);
        let a = PI * 0.01 * 0.01; // πr²
        let expected = MU_0 * 10000.0 * a / 0.1;
        assert_relative_eq!(s.inductance(), expected, max_relative = 1e-10);
    }

    #[test]
    fn solenoid_stored_energy() {
        let s = Solenoid::new(100, 0.1, 2.0, 0.01);
        let expected = 0.5 * s.inductance() * 4.0;
        assert_relative_eq!(s.stored_energy(), expected, max_relative = 1e-10);
    }

    #[test]
    fn solenoid_energy_density_consistent() {
        let s = Solenoid::new(100, 0.1, 1.0, 0.01);
        let volume = PI * s.radius * s.radius * s.length;
        let energy_from_density = s.energy_density() * volume;
        assert_relative_eq!(energy_from_density, s.stored_energy(), max_relative = 0.01);
    }

    // ================================================================
    // Toroid tests
    // ================================================================

    #[test]
    fn toroid_b_zero_outside() {
        let t = Toroid::new(500, 0.08, 0.12, 1.0);
        assert_relative_eq!(t.b_at_radius(0.05), 0.0, epsilon = 1e-15);
        assert_relative_eq!(t.b_at_radius(0.15), 0.0, epsilon = 1e-15);
    }

    #[test]
    fn toroid_b_inside() {
        let t = Toroid::new(500, 0.08, 0.12, 1.0);
        let r = 0.1;
        let expected = MU_0 * 500.0 * 1.0 / (2.0 * PI * r);
        assert_relative_eq!(t.b_at_radius(r), expected, max_relative = 1e-10);
    }

    #[test]
    fn toroid_b_decreases_outward() {
        let t = Toroid::new(500, 0.08, 0.12, 1.0);
        assert!(t.b_at_radius(0.09) > t.b_at_radius(0.11));
    }

    #[test]
    fn toroid_with_core() {
        let t = Toroid::new(500, 0.08, 0.12, 1.0).with_core(1000.0);
        let t_air = Toroid::new(500, 0.08, 0.12, 1.0);
        assert_relative_eq!(t.b_at_radius(0.1) / t_air.b_at_radius(0.1), 1000.0, max_relative = 1e-10);
    }

    #[test]
    #[should_panic]
    fn toroid_inner_greater_than_outer_panics() {
        Toroid::new(100, 0.12, 0.08, 1.0);
    }

    // ================================================================
    // Coaxial cable tests
    // ================================================================

    #[test]
    fn coax_b_at_center_is_zero() {
        let c = CoaxialCable::new(0.001, 0.005, 0.007, 1.0);
        assert_relative_eq!(c.b_at_radius(0.0), 0.0, epsilon = 1e-15);
    }

    #[test]
    fn coax_b_between_conductors() {
        let c = CoaxialCable::new(0.001, 0.005, 0.007, 1.0);
        let r = 0.003;
        let expected = MU_0 * 1.0 / (2.0 * PI * r);
        assert_relative_eq!(c.b_at_radius(r), expected, max_relative = 1e-10);
    }

    #[test]
    fn coax_b_outside_is_zero() {
        let c = CoaxialCable::new(0.001, 0.005, 0.007, 1.0);
        assert_relative_eq!(c.b_at_radius(0.01), 0.0, epsilon = 1e-15);
    }

    #[test]
    fn coax_b_inside_inner_linear() {
        let c = CoaxialCable::new(0.001, 0.005, 0.007, 1.0);
        let b1 = c.b_at_radius(0.0005);
        let b2 = c.b_at_radius(0.001); // at surface
        // Inside: B ∝ r, so B(a) / B(a/2) = 2
        assert_relative_eq!(b2 / b1, 2.0, max_relative = 1e-10);
    }

    #[test]
    fn coax_b_continuous_at_inner_surface() {
        let c = CoaxialCable::new(0.001, 0.005, 0.007, 1.0);
        let r = 0.001;
        let b_in = MU_0 * 1.0 * r / (2.0 * PI * r * r);
        let b_out = MU_0 * 1.0 / (2.0 * PI * r);
        assert_relative_eq!(b_in, b_out, max_relative = 1e-10);
    }

    #[test]
    fn coax_inductance_per_length() {
        let c = CoaxialCable::new(0.001, 0.005, 0.007, 1.0);
        let expected = MU_0 * (5.0_f64).ln() / (2.0 * PI);
        assert_relative_eq!(c.inductance_per_length(), expected, max_relative = 1e-10);
    }

    #[test]
    fn coax_sample_b_dimensions() {
        let c = CoaxialCable::new(0.001, 0.005, 0.007, 1.0);
        let (rs, bs) = c.sample_b_vs_r(0.01, 100);
        assert_eq!(rs.len(), 100);
        assert_eq!(bs.len(), 100);
    }

    #[test]
    #[should_panic]
    fn coax_invalid_radii_panics() {
        CoaxialCable::new(0.005, 0.001, 0.007, 1.0);
    }
}

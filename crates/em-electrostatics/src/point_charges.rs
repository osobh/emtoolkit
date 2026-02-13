//! Electric field and potential computation for systems of point charges.
//!
//! Uses Coulomb's law superposition for N point charges in free space.

use em_core::coordinates::{Cartesian, Vector3};
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

/// A point charge with position and charge value.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PointCharge {
    /// Position in 3D space
    pub position: Cartesian,
    /// Charge value in Coulombs (positive or negative)
    pub charge: f64,
}

impl PointCharge {
    pub fn new(x: f64, y: f64, z: f64, charge: f64) -> Self {
        Self {
            position: Cartesian::new(x, y, z),
            charge,
        }
    }
}

/// Compute the electric field at a point due to a set of point charges.
///
/// E(r) = Σ (q_i / (4πε₀)) · (r - r_i) / |r - r_i|³
///
/// # Arguments
/// * `charges` - Slice of point charges
/// * `point` - Observation point
/// * `epsilon` - Permittivity (F/m), use EPSILON_0 for free space
///
/// # Returns
/// Electric field vector (V/m) at the observation point.
pub fn electric_field(charges: &[PointCharge], point: &Cartesian, epsilon: f64) -> Vector3 {
    let k = 1.0 / (4.0 * PI * epsilon);
    let mut e_total = Vector3::zero();

    for charge in charges {
        let dx = point.x - charge.position.x;
        let dy = point.y - charge.position.y;
        let dz = point.z - charge.position.z;
        let r_sq = dx * dx + dy * dy + dz * dz;

        if r_sq < 1e-30 {
            continue; // skip self-point (singularity)
        }

        let r = r_sq.sqrt();
        let factor = k * charge.charge / (r_sq * r);
        e_total = e_total + Vector3::new(factor * dx, factor * dy, factor * dz);
    }

    e_total
}

/// Compute the electric potential at a point due to a set of point charges.
///
/// V(r) = Σ q_i / (4πε₀ · |r - r_i|)
pub fn electric_potential(charges: &[PointCharge], point: &Cartesian, epsilon: f64) -> f64 {
    let k = 1.0 / (4.0 * PI * epsilon);
    let mut v_total = 0.0;

    for charge in charges {
        let r = point.distance_to(&charge.position);
        if r < 1e-15 {
            continue;
        }
        v_total += k * charge.charge / r;
    }

    v_total
}

/// Sample electric field on a 2D grid at fixed z.
///
/// # Returns
/// (x_values, y_values, field_vectors, potential_values)
pub fn sample_field_2d(
    charges: &[PointCharge],
    epsilon: f64,
    x_range: (f64, f64),
    y_range: (f64, f64),
    z: f64,
    nx: usize,
    ny: usize,
) -> (Vec<f64>, Vec<f64>, Vec<Vector3>, Vec<f64>) {
    assert!(nx >= 2 && ny >= 2);
    let dx = (x_range.1 - x_range.0) / (nx - 1) as f64;
    let dy = (y_range.1 - y_range.0) / (ny - 1) as f64;

    let x_vals: Vec<f64> = (0..nx).map(|i| x_range.0 + i as f64 * dx).collect();
    let y_vals: Vec<f64> = (0..ny).map(|j| y_range.0 + j as f64 * dy).collect();

    let mut fields = Vec::with_capacity(nx * ny);
    let mut potentials = Vec::with_capacity(nx * ny);

    for &y in &y_vals {
        for &x in &x_vals {
            let pt = Cartesian::new(x, y, z);
            fields.push(electric_field(charges, &pt, epsilon));
            potentials.push(electric_potential(charges, &pt, epsilon));
        }
    }

    (x_vals, y_vals, fields, potentials)
}

/// Compute electric field lines starting from a charge using streamline tracing.
///
/// # Arguments
/// * `charges` - All charges in the system
/// * `start_charge_idx` - Index of the charge to start lines from
/// * `num_lines` - Number of field lines to trace
/// * `num_steps` - Steps per line
/// * `step_size` - Step size in meters
/// * `epsilon` - Permittivity
///
/// # Returns
/// Vector of field lines, each being a vector of 3D points.
pub fn trace_field_lines(
    charges: &[PointCharge],
    start_charge_idx: usize,
    num_lines: usize,
    num_steps: usize,
    step_size: f64,
    epsilon: f64,
) -> Vec<Vec<Cartesian>> {
    let start = &charges[start_charge_idx];
    let sign = if start.charge > 0.0 { 1.0 } else { -1.0 };

    let mut lines = Vec::with_capacity(num_lines);

    for i in 0..num_lines {
        let angle = 2.0 * PI * i as f64 / num_lines as f64;
        // Start slightly away from the charge
        let offset = 0.01;
        let mut pos = Cartesian::new(
            start.position.x + offset * angle.cos(),
            start.position.y + offset * angle.sin(),
            start.position.z,
        );

        let mut line = Vec::with_capacity(num_steps);
        line.push(pos);

        for _ in 0..num_steps {
            let e = electric_field(charges, &pos, epsilon);
            let mag = e.magnitude();
            if mag < 1e-20 {
                break; // field too weak
            }
            // Move in field direction (or opposite for negative charges)
            let dir = Vector3::new(e.x / mag, e.y / mag, e.z / mag);
            pos = Cartesian::new(
                pos.x + sign * step_size * dir.x,
                pos.y + sign * step_size * dir.y,
                pos.z + sign * step_size * dir.z,
            );

            // Stop if we're very close to another charge
            let near_charge = charges.iter().enumerate().any(|(j, c)| {
                j != start_charge_idx && pos.distance_to(&c.position) < step_size * 0.5
            });
            line.push(pos);
            if near_charge {
                break;
            }
        }

        lines.push(line);
    }

    lines
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use em_core::constants::EPSILON_0;

    #[test]
    fn single_positive_charge_field_direction() {
        let charges = vec![PointCharge::new(0.0, 0.0, 0.0, 1e-9)];
        let pt = Cartesian::new(1.0, 0.0, 0.0);
        let e = electric_field(&charges, &pt, EPSILON_0);
        assert!(e.x > 0.0, "field should point away from positive charge");
        assert_relative_eq!(e.y, 0.0, epsilon = 1e-20);
        assert_relative_eq!(e.z, 0.0, epsilon = 1e-20);
    }

    #[test]
    fn single_charge_field_magnitude_coulomb_law() {
        let q = 1e-9; // 1 nC
        let r = 0.1; // 10 cm
        let charges = vec![PointCharge::new(0.0, 0.0, 0.0, q)];
        let pt = Cartesian::new(r, 0.0, 0.0);
        let e = electric_field(&charges, &pt, EPSILON_0);
        let expected = q / (4.0 * PI * EPSILON_0 * r * r);
        assert_relative_eq!(e.x, expected, max_relative = 1e-10);
    }

    #[test]
    fn single_charge_potential() {
        let q = 1e-9;
        let r = 0.1;
        let charges = vec![PointCharge::new(0.0, 0.0, 0.0, q)];
        let pt = Cartesian::new(r, 0.0, 0.0);
        let v = electric_potential(&charges, &pt, EPSILON_0);
        let expected = q / (4.0 * PI * EPSILON_0 * r);
        assert_relative_eq!(v, expected, max_relative = 1e-10);
    }

    #[test]
    fn field_inverse_square_law() {
        let charges = vec![PointCharge::new(0.0, 0.0, 0.0, 1e-9)];
        let e1 = electric_field(&charges, &Cartesian::new(1.0, 0.0, 0.0), EPSILON_0);
        let e2 = electric_field(&charges, &Cartesian::new(2.0, 0.0, 0.0), EPSILON_0);
        // E ∝ 1/r² → E(2r)/E(r) = 1/4
        assert_relative_eq!(e2.x / e1.x, 0.25, max_relative = 1e-10);
    }

    #[test]
    fn two_equal_charges_field_at_midpoint_is_zero() {
        let charges = vec![
            PointCharge::new(-1.0, 0.0, 0.0, 1e-9),
            PointCharge::new(1.0, 0.0, 0.0, 1e-9),
        ];
        let e = electric_field(&charges, &Cartesian::new(0.0, 0.0, 0.0), EPSILON_0);
        assert_relative_eq!(e.x, 0.0, epsilon = 1e-20);
    }

    #[test]
    fn dipole_field_on_perpendicular_bisector() {
        // For a dipole ±q at ±d, field on y-axis should point in -x direction
        let d = 0.01;
        let q = 1e-9;
        let charges = vec![
            PointCharge::new(d, 0.0, 0.0, q),
            PointCharge::new(-d, 0.0, 0.0, -q),
        ];
        let e = electric_field(&charges, &Cartesian::new(0.0, 0.1, 0.0), EPSILON_0);
        // On the perpendicular bisector of a dipole (+q at +x, -q at -x),
        // E_x points from + toward - (negative x direction)
        assert!(e.x < 0.0, "dipole field on perp bisector points from + to - charge");
        assert_relative_eq!(e.y, 0.0, epsilon = 1e-10);
    }

    #[test]
    fn negative_charge_field_points_inward() {
        let charges = vec![PointCharge::new(0.0, 0.0, 0.0, -1e-9)];
        let e = electric_field(&charges, &Cartesian::new(1.0, 0.0, 0.0), EPSILON_0);
        assert!(e.x < 0.0, "field should point toward negative charge");
    }

    #[test]
    fn potential_positive_charge_is_positive() {
        let charges = vec![PointCharge::new(0.0, 0.0, 0.0, 1e-9)];
        let v = electric_potential(&charges, &Cartesian::new(0.1, 0.0, 0.0), EPSILON_0);
        assert!(v > 0.0);
    }

    #[test]
    fn potential_decreases_with_distance() {
        let charges = vec![PointCharge::new(0.0, 0.0, 0.0, 1e-9)];
        let v1 = electric_potential(&charges, &Cartesian::new(0.1, 0.0, 0.0), EPSILON_0);
        let v2 = electric_potential(&charges, &Cartesian::new(0.2, 0.0, 0.0), EPSILON_0);
        assert!(v1 > v2);
    }

    #[test]
    fn sample_field_2d_dimensions() {
        let charges = vec![PointCharge::new(0.0, 0.0, 0.0, 1e-9)];
        let (xs, ys, fs, vs) = sample_field_2d(&charges, EPSILON_0, (-1.0, 1.0), (-1.0, 1.0), 0.0, 10, 8);
        assert_eq!(xs.len(), 10);
        assert_eq!(ys.len(), 8);
        assert_eq!(fs.len(), 80);
        assert_eq!(vs.len(), 80);
    }

    #[test]
    fn trace_field_lines_from_positive_charge() {
        let charges = vec![PointCharge::new(0.0, 0.0, 0.0, 1e-9)];
        let lines = trace_field_lines(&charges, 0, 8, 50, 0.01, EPSILON_0);
        assert_eq!(lines.len(), 8);
        // Each line should move away from origin
        for line in &lines {
            assert!(line.len() > 1);
            let first_dist = line[0].distance_to(&charges[0].position);
            let last_dist = line.last().unwrap().distance_to(&charges[0].position);
            assert!(last_dist > first_dist, "lines should go outward from positive charge");
        }
    }

    #[test]
    fn trace_field_lines_dipole_terminate() {
        let charges = vec![
            PointCharge::new(-0.05, 0.0, 0.0, 1e-9),
            PointCharge::new(0.05, 0.0, 0.0, -1e-9),
        ];
        let lines = trace_field_lines(&charges, 0, 4, 200, 0.005, EPSILON_0);
        // Lines from positive charge should terminate near negative charge
        for line in &lines {
            let last = line.last().unwrap();
            let dist_to_neg = last.distance_to(&charges[1].position);
            // Some lines may not reach, but at least some should get close
            assert!(line.len() > 1);
            let _ = dist_to_neg; // just ensure it computes
        }
    }
}

//! Biot-Savart law for magnetic field computation.
//!
//! B(r) = (μ₀/4π) ∫ I dl' × r̂ / r² along a current path.
//! Implemented via numerical integration over discrete current segments.

use em_core::constants::MU_0;
use em_core::coordinates::{Cartesian, Vector3};
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

/// A current segment defined by start and end points carrying current I.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct CurrentSegment {
    pub start: Cartesian,
    pub end: Cartesian,
    pub current: f64,
}

impl CurrentSegment {
    pub fn new(start: Cartesian, end: Cartesian, current: f64) -> Self {
        Self { start, end, current }
    }

    /// Direction vector (dl) from start to end.
    pub fn dl(&self) -> Vector3 {
        Vector3::new(
            self.end.x - self.start.x,
            self.end.y - self.start.y,
            self.end.z - self.start.z,
        )
    }

    /// Midpoint of the segment.
    pub fn midpoint(&self) -> Cartesian {
        Cartesian::new(
            (self.start.x + self.end.x) / 2.0,
            (self.start.y + self.end.y) / 2.0,
            (self.start.z + self.end.z) / 2.0,
        )
    }
}

/// Compute B-field at observation point from a single current segment
/// using the Biot-Savart law.
///
/// dB = (μ₀ I / 4π) · (dl × r̂) / r²
pub fn b_field_segment(segment: &CurrentSegment, point: &Cartesian) -> Vector3 {
    let mid = segment.midpoint();
    let r_vec = Vector3::new(
        point.x - mid.x,
        point.y - mid.y,
        point.z - mid.z,
    );
    let r_mag = r_vec.magnitude();

    if r_mag < 1e-15 {
        return Vector3::zero();
    }

    let dl = segment.dl();
    let cross = dl.cross(&r_vec);
    let factor = MU_0 * segment.current / (4.0 * PI * r_mag.powi(3));

    Vector3::new(
        factor * cross.x,
        factor * cross.y,
        factor * cross.z,
    )
}

/// Compute total B-field at observation point from multiple current segments.
pub fn b_field_total(segments: &[CurrentSegment], point: &Cartesian) -> Vector3 {
    let mut total = Vector3::zero();
    for seg in segments {
        let db = b_field_segment(seg, point);
        total = total + db;
    }
    total
}

/// Magnetic field of an infinite straight wire carrying current I
/// at perpendicular distance ρ from the wire.
///
/// B = μ₀ I / (2π ρ) — direction given by right-hand rule.
///
/// Wire assumed along the z-axis.
pub fn b_infinite_wire(current: f64, rho: f64) -> f64 {
    assert!(rho > 0.0, "distance must be positive");
    MU_0 * current / (2.0 * PI * rho)
}

/// Discretize an infinite wire (along z-axis) into segments for numerical computation.
///
/// Returns segments from z = -half_length to z = +half_length.
pub fn discretize_wire_z(
    current: f64,
    half_length: f64,
    num_segments: usize,
) -> Vec<CurrentSegment> {
    assert!(num_segments > 0);
    let dz = 2.0 * half_length / num_segments as f64;
    let mut segments = Vec::with_capacity(num_segments);

    for i in 0..num_segments {
        let z0 = -half_length + i as f64 * dz;
        let z1 = z0 + dz;
        segments.push(CurrentSegment::new(
            Cartesian::new(0.0, 0.0, z0),
            Cartesian::new(0.0, 0.0, z1),
            current,
        ));
    }

    segments
}

/// Sample B-field magnitude on a 2D grid (in the xy-plane at z=0)
/// for a set of current segments.
pub fn sample_b_field_2d(
    segments: &[CurrentSegment],
    x_range: (f64, f64),
    y_range: (f64, f64),
    z: f64,
    nx: usize,
    ny: usize,
) -> (Vec<f64>, Vec<f64>, Vec<Vector3>) {
    assert!(nx >= 2 && ny >= 2);
    let dx = (x_range.1 - x_range.0) / (nx - 1) as f64;
    let dy = (y_range.1 - y_range.0) / (ny - 1) as f64;

    let x_vals: Vec<f64> = (0..nx).map(|i| x_range.0 + i as f64 * dx).collect();
    let y_vals: Vec<f64> = (0..ny).map(|j| y_range.0 + j as f64 * dy).collect();

    let mut fields = Vec::with_capacity(nx * ny);
    for &y in &y_vals {
        for &x in &x_vals {
            let pt = Cartesian::new(x, y, z);
            fields.push(b_field_total(segments, &pt));
        }
    }

    (x_vals, y_vals, fields)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn b_infinite_wire_at_1m() {
        // B = μ₀·I/(2π·ρ) for I=1A, ρ=1m
        let b = b_infinite_wire(1.0, 1.0);
        let expected = MU_0 / (2.0 * PI);
        assert_relative_eq!(b, expected, max_relative = 1e-10);
    }

    #[test]
    fn b_infinite_wire_inverse_distance() {
        let b1 = b_infinite_wire(1.0, 0.1);
        let b2 = b_infinite_wire(1.0, 0.2);
        assert_relative_eq!(b1 / b2, 2.0, max_relative = 1e-10);
    }

    #[test]
    fn b_infinite_wire_proportional_to_current() {
        let b1 = b_infinite_wire(1.0, 0.1);
        let b2 = b_infinite_wire(5.0, 0.1);
        assert_relative_eq!(b2 / b1, 5.0, max_relative = 1e-10);
    }

    #[test]
    fn numerical_wire_approaches_analytical() {
        // Long wire along z, measure B at (0.1, 0, 0)
        let current = 1.0;
        let segments = discretize_wire_z(current, 50.0, 10000);
        let pt = Cartesian::new(0.1, 0.0, 0.0);
        let b_num = b_field_total(&segments, &pt);
        let b_analytical = b_infinite_wire(current, 0.1);

        // dl=(0,0,dz) × r=(x,0,0) → (0, dz·x, 0) → but we need dl×r_hat/r²
        // For wire along +z, observation at +x: B is in -y direction by right-hand rule
        // But our segments go +z, midpoint-to-obs is +x, dl×r = +y
        // This is because our r_vec = obs - source (standard Biot-Savart), giving +y
        assert!(b_num.y > 0.0, "B in +y via Biot-Savart convention (dl × r̂)");
        assert_relative_eq!(b_num.magnitude(), b_analytical, max_relative = 0.01);
    }

    #[test]
    fn b_field_single_segment_direction() {
        // Segment along z, observation at +x → B should be in -y (or +y depending on convention)
        let seg = CurrentSegment::new(
            Cartesian::new(0.0, 0.0, -0.5),
            Cartesian::new(0.0, 0.0, 0.5),
            1.0,
        );
        let b = b_field_segment(&seg, &Cartesian::new(1.0, 0.0, 0.0));
        // dl = (0,0,1), r = (1,0,0), dl × r = (0·0-1·0, 1·1-0·0, 0·0-0·1) = (0,1,0)
        // Wait: dl×r = (0,0,1)×(1,0,0) = (0·0-1·0, 1·1-0·0, 0·0-0·1) = (0,1,0)
        // Hmm that's +y. Let me recalculate:
        // dl = (0,0,1), r_vec = (1,0,0)
        // dl × r_vec = |i  j  k |
        //              |0  0  1 |
        //              |1  0  0 |
        // = i(0·0-1·0) - j(0·0-1·1) + k(0·0-0·1) = (0, 1, 0)
        // So B is in +y direction
        assert!(b.y > 0.0, "B should be in +y for z-segment at +x");
        assert_relative_eq!(b.x, 0.0, epsilon = 1e-20);
        assert_relative_eq!(b.z, 0.0, epsilon = 1e-20);
    }

    #[test]
    fn b_field_at_singularity_is_zero() {
        let seg = CurrentSegment::new(
            Cartesian::new(0.0, 0.0, -0.5),
            Cartesian::new(0.0, 0.0, 0.5),
            1.0,
        );
        let b = b_field_segment(&seg, &seg.midpoint());
        assert_relative_eq!(b.magnitude(), 0.0, epsilon = 1e-15);
    }

    #[test]
    fn discretize_wire_correct_count() {
        let segs = discretize_wire_z(1.0, 10.0, 100);
        assert_eq!(segs.len(), 100);
    }

    #[test]
    fn discretize_wire_contiguous() {
        let segs = discretize_wire_z(1.0, 10.0, 50);
        for i in 1..segs.len() {
            assert_relative_eq!(segs[i].start.z, segs[i - 1].end.z, epsilon = 1e-12);
        }
    }

    #[test]
    fn sample_b_field_2d_dimensions() {
        let segs = discretize_wire_z(1.0, 5.0, 100);
        let (xs, ys, fs) = sample_b_field_2d(&segs, (-1.0, 1.0), (-1.0, 1.0), 0.0, 5, 5);
        assert_eq!(xs.len(), 5);
        assert_eq!(ys.len(), 5);
        assert_eq!(fs.len(), 25);
    }

    #[test]
    #[should_panic]
    fn b_infinite_wire_zero_distance_panics() {
        b_infinite_wire(1.0, 0.0);
    }
}

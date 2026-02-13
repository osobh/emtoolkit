//! Module 3.1: Vector Addition/Subtraction
//!
//! Provides 3D vector operations with visualization data for the
//! parallelogram rule and vector decomposition.

use em_core::coordinates::Vector3;
use serde::{Deserialize, Serialize};

/// Result of a vector addition with visualization data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VectorAddResult {
    /// Input vector A
    pub a: Vector3,
    /// Input vector B
    pub b: Vector3,
    /// Result vector C = A + B
    pub result: Vector3,
    /// Magnitude of result
    pub result_magnitude: f64,
    /// Angle between A and B (radians)
    pub angle_between: f64,
    /// Parallelogram vertices for visualization: [origin, A, A+B, B]
    pub parallelogram: [Vector3; 4],
}

/// Add two vectors with full visualization data.
pub fn vector_add(a: Vector3, b: Vector3) -> VectorAddResult {
    let result = a + b;
    let mag_a = a.magnitude();
    let mag_b = b.magnitude();
    let angle = if mag_a > 0.0 && mag_b > 0.0 {
        (a.dot(&b) / (mag_a * mag_b)).clamp(-1.0, 1.0).acos()
    } else {
        0.0
    };

    VectorAddResult {
        a,
        b,
        result,
        result_magnitude: result.magnitude(),
        angle_between: angle,
        parallelogram: [Vector3::zero(), a, result, b],
    }
}

/// Result of a vector subtraction.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VectorSubResult {
    pub a: Vector3,
    pub b: Vector3,
    /// Result D = A - B
    pub result: Vector3,
    pub result_magnitude: f64,
}

/// Subtract two vectors.
pub fn vector_sub(a: Vector3, b: Vector3) -> VectorSubResult {
    let result = a - b;
    VectorSubResult {
        a,
        b,
        result,
        result_magnitude: result.magnitude(),
    }
}

/// Decompose a vector into components along and perpendicular to a reference direction.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VectorProjection {
    /// Component along the reference direction
    pub parallel: Vector3,
    /// Component perpendicular to the reference direction
    pub perpendicular: Vector3,
    /// Scalar projection (signed magnitude along reference)
    pub scalar_projection: f64,
}

/// Project vector `v` onto direction `reference`.
pub fn project(v: Vector3, reference: Vector3) -> VectorProjection {
    let ref_mag = reference.magnitude();
    if ref_mag == 0.0 {
        return VectorProjection {
            parallel: Vector3::zero(),
            perpendicular: v,
            scalar_projection: 0.0,
        };
    }
    let ref_unit = reference.normalized();
    let scalar = v.dot(&ref_unit);
    let parallel = ref_unit * scalar;
    let perpendicular = v - parallel;

    VectorProjection {
        parallel,
        perpendicular,
        scalar_projection: scalar,
    }
}

/// Compute the cross product with magnitude and direction info.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CrossProductResult {
    pub a: Vector3,
    pub b: Vector3,
    pub result: Vector3,
    pub magnitude: f64,
    /// Area of the parallelogram formed by A and B
    pub parallelogram_area: f64,
}

/// Cross product with visualization data.
pub fn cross_product(a: Vector3, b: Vector3) -> CrossProductResult {
    let result = a.cross(&b);
    let magnitude = result.magnitude();
    CrossProductResult {
        a,
        b,
        result,
        magnitude,
        parallelogram_area: magnitude,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use std::f64::consts::{FRAC_PI_2, PI};

    #[test]
    fn add_parallel_vectors() {
        let r = vector_add(
            Vector3::new(3.0, 0.0, 0.0),
            Vector3::new(4.0, 0.0, 0.0),
        );
        assert_relative_eq!(r.result.x, 7.0, epsilon = 1e-12);
        assert_relative_eq!(r.result_magnitude, 7.0, epsilon = 1e-12);
        assert_relative_eq!(r.angle_between, 0.0, epsilon = 1e-12);
    }

    #[test]
    fn add_antiparallel_vectors() {
        let r = vector_add(
            Vector3::new(5.0, 0.0, 0.0),
            Vector3::new(-3.0, 0.0, 0.0),
        );
        assert_relative_eq!(r.result.x, 2.0, epsilon = 1e-12);
        assert_relative_eq!(r.angle_between, PI, epsilon = 1e-12);
    }

    #[test]
    fn add_orthogonal_vectors() {
        let r = vector_add(
            Vector3::new(3.0, 0.0, 0.0),
            Vector3::new(0.0, 4.0, 0.0),
        );
        assert_relative_eq!(r.result_magnitude, 5.0, epsilon = 1e-12);
        assert_relative_eq!(r.angle_between, FRAC_PI_2, epsilon = 1e-12);
    }

    #[test]
    fn add_parallelogram_vertices_correct() {
        let a = Vector3::new(1.0, 0.0, 0.0);
        let b = Vector3::new(0.0, 1.0, 0.0);
        let r = vector_add(a, b);
        assert_eq!(r.parallelogram[0], Vector3::zero());
        assert_eq!(r.parallelogram[1], a);
        assert_eq!(r.parallelogram[2], r.result);
        assert_eq!(r.parallelogram[3], b);
    }

    #[test]
    fn sub_vectors() {
        let r = vector_sub(
            Vector3::new(5.0, 3.0, 1.0),
            Vector3::new(2.0, 1.0, 0.0),
        );
        assert_relative_eq!(r.result.x, 3.0, epsilon = 1e-12);
        assert_relative_eq!(r.result.y, 2.0, epsilon = 1e-12);
        assert_relative_eq!(r.result.z, 1.0, epsilon = 1e-12);
    }

    #[test]
    fn project_parallel() {
        let p = project(
            Vector3::new(6.0, 0.0, 0.0),
            Vector3::new(1.0, 0.0, 0.0),
        );
        assert_relative_eq!(p.scalar_projection, 6.0, epsilon = 1e-12);
        assert_relative_eq!(p.parallel.x, 6.0, epsilon = 1e-12);
        assert_relative_eq!(p.perpendicular.magnitude(), 0.0, epsilon = 1e-12);
    }

    #[test]
    fn project_orthogonal() {
        let p = project(
            Vector3::new(0.0, 5.0, 0.0),
            Vector3::new(1.0, 0.0, 0.0),
        );
        assert_relative_eq!(p.scalar_projection, 0.0, epsilon = 1e-12);
        assert_relative_eq!(p.parallel.magnitude(), 0.0, epsilon = 1e-12);
        assert_relative_eq!(p.perpendicular.y, 5.0, epsilon = 1e-12);
    }

    #[test]
    fn project_at_45_degrees() {
        let p = project(
            Vector3::new(1.0, 1.0, 0.0),
            Vector3::new(1.0, 0.0, 0.0),
        );
        assert_relative_eq!(p.scalar_projection, 1.0, epsilon = 1e-12);
        assert_relative_eq!(p.parallel.x, 1.0, epsilon = 1e-12);
        assert_relative_eq!(p.perpendicular.y, 1.0, epsilon = 1e-12);
    }

    #[test]
    fn project_onto_zero_vector() {
        let p = project(Vector3::new(1.0, 2.0, 3.0), Vector3::zero());
        assert_relative_eq!(p.scalar_projection, 0.0, epsilon = 1e-12);
        assert_relative_eq!(p.perpendicular.x, 1.0, epsilon = 1e-12);
    }

    #[test]
    fn cross_product_orthogonal() {
        let r = cross_product(
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
        );
        assert_relative_eq!(r.result.z, 1.0, epsilon = 1e-12);
        assert_relative_eq!(r.magnitude, 1.0, epsilon = 1e-12);
        assert_relative_eq!(r.parallelogram_area, 1.0, epsilon = 1e-12);
    }

    #[test]
    fn cross_product_parallel_is_zero() {
        let r = cross_product(
            Vector3::new(3.0, 0.0, 0.0),
            Vector3::new(6.0, 0.0, 0.0),
        );
        assert_relative_eq!(r.magnitude, 0.0, epsilon = 1e-12);
    }
}

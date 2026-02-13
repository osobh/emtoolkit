//! Vector field evaluation and sampling for divergence/curl visualization.

use em_core::coordinates::Vector3;
use serde::{Deserialize, Serialize};

/// Preset vector fields for demonstration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VectorFieldPreset {
    /// F = (x, y, z) — radial outward (source)
    RadialOutward,
    /// F = (-y, x, 0) — 2D rotation (vortex)
    Rotation2D,
    /// F = (y, -x, 0) — clockwise rotation
    RotationCW,
    /// F = (x², xy, 0) — non-uniform
    NonUniform,
    /// F = (1, 0, 0) — uniform in x
    UniformX,
    /// F = (-x, -y, -z) — radial inward (sink)
    RadialInward,
}

impl VectorFieldPreset {
    /// Evaluate the vector field at (x, y, z).
    pub fn evaluate(&self, x: f64, y: f64, z: f64) -> Vector3 {
        match self {
            Self::RadialOutward => Vector3::new(x, y, z),
            Self::Rotation2D => Vector3::new(-y, x, 0.0),
            Self::RotationCW => Vector3::new(y, -x, 0.0),
            Self::NonUniform => Vector3::new(x * x, x * y, 0.0),
            Self::UniformX => Vector3::new(1.0, 0.0, 0.0),
            Self::RadialInward => Vector3::new(-x, -y, -z),
        }
    }

    /// Analytical divergence (exact) for validation.
    /// div(F) = ∂Fx/∂x + ∂Fy/∂y + ∂Fz/∂z
    pub fn divergence_exact(&self, x: f64, _y: f64, _z: f64) -> f64 {
        match self {
            Self::RadialOutward => 3.0,           // 1 + 1 + 1
            Self::Rotation2D => 0.0,              // 0 + 0 + 0
            Self::RotationCW => 0.0,              // 0 + 0 + 0
            Self::NonUniform => 3.0 * x,            // ∂(x²)/∂x + ∂(xy)/∂y + 0 = 2x + x = 3x
            Self::UniformX => 0.0,                // 0 + 0 + 0
            Self::RadialInward => -3.0,           // -1 + -1 + -1
        }
    }

    /// Analytical curl (exact) for validation.
    /// curl(F) = (∂Fz/∂y - ∂Fy/∂z, ∂Fx/∂z - ∂Fz/∂x, ∂Fy/∂x - ∂Fx/∂y)
    pub fn curl_exact(&self, _x: f64, _y: f64, _z: f64) -> Vector3 {
        match self {
            Self::RadialOutward => Vector3::zero(), // irrotational
            Self::Rotation2D => Vector3::new(0.0, 0.0, 2.0), // constant curl in z
            Self::RotationCW => Vector3::new(0.0, 0.0, -2.0),
            Self::NonUniform => Vector3::new(0.0, 0.0, _y), // ∂(xy)/∂x - ∂(x²)/∂y = y - 0 = y
            Self::UniformX => Vector3::zero(),
            Self::RadialInward => Vector3::zero(),
        }
    }
}

/// A 2D grid of vector field samples for arrow/streamline visualization.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VectorFieldGrid2D {
    /// x coordinates
    pub x_values: Vec<f64>,
    /// y coordinates
    pub y_values: Vec<f64>,
    /// Vector values indexed as [iy * nx + ix]
    pub vectors: Vec<Vector3>,
    /// Scalar divergence at each point
    pub divergence: Vec<f64>,
    /// z-component of curl at each point (for 2D visualization)
    pub curl_z: Vec<f64>,
    pub nx: usize,
    pub ny: usize,
}

/// Sample a vector field on a 2D grid at fixed z.
pub fn sample_2d(
    field: VectorFieldPreset,
    x_range: (f64, f64),
    y_range: (f64, f64),
    z: f64,
    nx: usize,
    ny: usize,
) -> VectorFieldGrid2D {
    assert!(nx >= 2 && ny >= 2);
    let dx = (x_range.1 - x_range.0) / (nx - 1) as f64;
    let dy = (y_range.1 - y_range.0) / (ny - 1) as f64;

    let x_values: Vec<f64> = (0..nx).map(|i| x_range.0 + i as f64 * dx).collect();
    let y_values: Vec<f64> = (0..ny).map(|j| y_range.0 + j as f64 * dy).collect();

    let mut vectors = Vec::with_capacity(nx * ny);
    let mut divergence = Vec::with_capacity(nx * ny);
    let mut curl_z = Vec::with_capacity(nx * ny);

    for &y in &y_values {
        for &x in &x_values {
            vectors.push(field.evaluate(x, y, z));
            divergence.push(field.divergence_exact(x, y, z));
            let curl = field.curl_exact(x, y, z);
            curl_z.push(curl.z);
        }
    }

    VectorFieldGrid2D {
        x_values,
        y_values,
        vectors,
        divergence,
        curl_z,
        nx,
        ny,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn radial_outward_at_origin_is_zero() {
        let v = VectorFieldPreset::RadialOutward.evaluate(0.0, 0.0, 0.0);
        assert_relative_eq!(v.magnitude(), 0.0, epsilon = 1e-12);
    }

    #[test]
    fn radial_outward_points_away() {
        let v = VectorFieldPreset::RadialOutward.evaluate(3.0, 4.0, 0.0);
        assert_relative_eq!(v.x, 3.0, epsilon = 1e-12);
        assert_relative_eq!(v.y, 4.0, epsilon = 1e-12);
    }

    #[test]
    fn rotation_2d_perpendicular_to_position() {
        let pos = Vector3::new(1.0, 0.0, 0.0);
        let v = VectorFieldPreset::Rotation2D.evaluate(1.0, 0.0, 0.0);
        assert_relative_eq!(pos.dot(&v), 0.0, epsilon = 1e-12);
    }

    #[test]
    fn div_radial_outward_is_3() {
        assert_relative_eq!(
            VectorFieldPreset::RadialOutward.divergence_exact(5.0, 3.0, 1.0),
            3.0,
            epsilon = 1e-12
        );
    }

    #[test]
    fn div_rotation_is_zero() {
        assert_relative_eq!(
            VectorFieldPreset::Rotation2D.divergence_exact(5.0, 3.0, 0.0),
            0.0,
            epsilon = 1e-12
        );
    }

    #[test]
    fn div_radial_inward_is_minus_3() {
        assert_relative_eq!(
            VectorFieldPreset::RadialInward.divergence_exact(1.0, 1.0, 1.0),
            -3.0,
            epsilon = 1e-12
        );
    }

    #[test]
    fn curl_rotation_2d_is_2z() {
        let c = VectorFieldPreset::Rotation2D.curl_exact(0.0, 0.0, 0.0);
        assert_relative_eq!(c.z, 2.0, epsilon = 1e-12);
    }

    #[test]
    fn curl_radial_outward_is_zero() {
        let c = VectorFieldPreset::RadialOutward.curl_exact(1.0, 2.0, 3.0);
        assert_relative_eq!(c.magnitude(), 0.0, epsilon = 1e-12);
    }

    #[test]
    fn curl_uniform_is_zero() {
        let c = VectorFieldPreset::UniformX.curl_exact(1.0, 2.0, 3.0);
        assert_relative_eq!(c.magnitude(), 0.0, epsilon = 1e-12);
    }

    #[test]
    fn sample_2d_grid_dimensions() {
        let grid = sample_2d(VectorFieldPreset::RadialOutward, (-1.0, 1.0), (-1.0, 1.0), 0.0, 10, 8);
        assert_eq!(grid.vectors.len(), 80);
        assert_eq!(grid.divergence.len(), 80);
        assert_eq!(grid.curl_z.len(), 80);
    }

    #[test]
    fn sample_2d_values_match_field() {
        let grid = sample_2d(VectorFieldPreset::UniformX, (-1.0, 1.0), (-1.0, 1.0), 0.0, 5, 5);
        for v in &grid.vectors {
            assert_relative_eq!(v.x, 1.0, epsilon = 1e-12);
            assert_relative_eq!(v.y, 0.0, epsilon = 1e-12);
        }
    }
}

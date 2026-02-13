//! Scalar field evaluation and sampling for gradient visualization.
//!
//! Provides preset scalar fields and evaluation on 2D/3D grids.

use em_core::coordinates::Vector3;
use serde::{Deserialize, Serialize};

/// A scalar field f(x, y, z) → f64.
pub type ScalarFieldFn = fn(f64, f64, f64) -> f64;

/// Preset scalar fields for demonstration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScalarFieldPreset {
    /// f = x² + y² + z² (paraboloid)
    Paraboloid,
    /// f = x² - y² (saddle)
    Saddle,
    /// f = sin(x)·cos(y)
    SinCos,
    /// f = 1/√(x² + y² + z²) (point source potential)
    InverseR,
    /// f = x·y·z
    Product,
    /// f = e^(-(x²+y²+z²)) (Gaussian)
    Gaussian,
}

impl ScalarFieldPreset {
    /// Evaluate the scalar field at (x, y, z).
    pub fn evaluate(&self, x: f64, y: f64, z: f64) -> f64 {
        match self {
            Self::Paraboloid => x * x + y * y + z * z,
            Self::Saddle => x * x - y * y,
            Self::SinCos => x.sin() * y.cos(),
            Self::InverseR => {
                let r = (x * x + y * y + z * z).sqrt();
                if r < 1e-10 { 1e10 } else { 1.0 / r }
            }
            Self::Product => x * y * z,
            Self::Gaussian => (-(x * x + y * y + z * z)).exp(),
        }
    }

    /// Analytical gradient (exact) for validation.
    pub fn gradient_exact(&self, x: f64, y: f64, z: f64) -> Vector3 {
        match self {
            Self::Paraboloid => Vector3::new(2.0 * x, 2.0 * y, 2.0 * z),
            Self::Saddle => Vector3::new(2.0 * x, -2.0 * y, 0.0),
            Self::SinCos => Vector3::new(x.cos() * y.cos(), -x.sin() * y.sin(), 0.0),
            Self::InverseR => {
                let r2 = x * x + y * y + z * z;
                let r = r2.sqrt();
                if r < 1e-10 {
                    Vector3::zero()
                } else {
                    let factor = -1.0 / (r2 * r);
                    Vector3::new(factor * x, factor * y, factor * z)
                }
            }
            Self::Product => Vector3::new(y * z, x * z, x * y),
            Self::Gaussian => {
                let g = (-(x * x + y * y + z * z)).exp();
                Vector3::new(-2.0 * x * g, -2.0 * y * g, -2.0 * z * g)
            }
        }
    }
}

/// A 2D grid of scalar field values for contour/surface plotting.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScalarFieldGrid2D {
    /// x coordinates
    pub x_values: Vec<f64>,
    /// y coordinates
    pub y_values: Vec<f64>,
    /// Field values indexed as [iy * nx + ix]
    pub values: Vec<f64>,
    /// Number of x points
    pub nx: usize,
    /// Number of y points
    pub ny: usize,
}

/// Sample a scalar field on a 2D grid at fixed z.
pub fn sample_2d(
    field: ScalarFieldPreset,
    x_range: (f64, f64),
    y_range: (f64, f64),
    z: f64,
    nx: usize,
    ny: usize,
) -> ScalarFieldGrid2D {
    assert!(nx >= 2 && ny >= 2);
    let dx = (x_range.1 - x_range.0) / (nx - 1) as f64;
    let dy = (y_range.1 - y_range.0) / (ny - 1) as f64;

    let x_values: Vec<f64> = (0..nx).map(|i| x_range.0 + i as f64 * dx).collect();
    let y_values: Vec<f64> = (0..ny).map(|j| y_range.0 + j as f64 * dy).collect();

    let mut values = Vec::with_capacity(nx * ny);
    for &y in &y_values {
        for &x in &x_values {
            values.push(field.evaluate(x, y, z));
        }
    }

    ScalarFieldGrid2D {
        x_values,
        y_values,
        values,
        nx,
        ny,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn paraboloid_at_origin_is_zero() {
        assert_relative_eq!(ScalarFieldPreset::Paraboloid.evaluate(0.0, 0.0, 0.0), 0.0, epsilon = 1e-12);
    }

    #[test]
    fn paraboloid_symmetric() {
        let f = ScalarFieldPreset::Paraboloid;
        assert_relative_eq!(f.evaluate(1.0, 0.0, 0.0), f.evaluate(0.0, 1.0, 0.0), epsilon = 1e-12);
    }

    #[test]
    fn saddle_zero_on_diagonals() {
        let f = ScalarFieldPreset::Saddle;
        assert_relative_eq!(f.evaluate(1.0, 1.0, 0.0), 0.0, epsilon = 1e-12);
        assert_relative_eq!(f.evaluate(-1.0, 1.0, 0.0), 0.0, epsilon = 1e-12);
    }

    #[test]
    fn gaussian_peak_at_origin() {
        let f = ScalarFieldPreset::Gaussian;
        assert_relative_eq!(f.evaluate(0.0, 0.0, 0.0), 1.0, epsilon = 1e-12);
        assert!(f.evaluate(1.0, 0.0, 0.0) < 1.0);
    }

    #[test]
    fn inverse_r_decreases_with_distance() {
        let f = ScalarFieldPreset::InverseR;
        assert!(f.evaluate(1.0, 0.0, 0.0) > f.evaluate(2.0, 0.0, 0.0));
    }

    #[test]
    fn gradient_paraboloid_at_1_2_3() {
        let g = ScalarFieldPreset::Paraboloid.gradient_exact(1.0, 2.0, 3.0);
        assert_relative_eq!(g.x, 2.0, epsilon = 1e-12);
        assert_relative_eq!(g.y, 4.0, epsilon = 1e-12);
        assert_relative_eq!(g.z, 6.0, epsilon = 1e-12);
    }

    #[test]
    fn gradient_saddle_at_origin_is_zero() {
        let g = ScalarFieldPreset::Saddle.gradient_exact(0.0, 0.0, 0.0);
        assert_relative_eq!(g.magnitude(), 0.0, epsilon = 1e-12);
    }

    #[test]
    fn gradient_gaussian_points_inward() {
        let g = ScalarFieldPreset::Gaussian.gradient_exact(1.0, 0.0, 0.0);
        assert!(g.x < 0.0, "gradient should point toward origin (decreasing r)");
    }

    #[test]
    fn sample_2d_dimensions() {
        let grid = sample_2d(ScalarFieldPreset::Paraboloid, (-1.0, 1.0), (-1.0, 1.0), 0.0, 20, 15);
        assert_eq!(grid.nx, 20);
        assert_eq!(grid.ny, 15);
        assert_eq!(grid.values.len(), 300);
        assert_eq!(grid.x_values.len(), 20);
        assert_eq!(grid.y_values.len(), 15);
    }

    #[test]
    fn sample_2d_values_match_field() {
        let grid = sample_2d(ScalarFieldPreset::Saddle, (-2.0, 2.0), (-2.0, 2.0), 0.0, 5, 5);
        // Check corner value: f(-2, -2, 0) = 4 - 4 = 0
        assert_relative_eq!(grid.values[0], 0.0, epsilon = 1e-12);
    }
}

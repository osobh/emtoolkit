//! Numerical gradient, divergence, and curl using finite differences.
//!
//! These provide the actual computation for arbitrary fields when analytical
//! expressions aren't available. Uses central differences for O(h²) accuracy.

use em_core::coordinates::Vector3;

/// Compute the numerical gradient of a scalar field at a point using central differences.
///
/// ∇f ≈ ((f(x+h)-f(x-h))/(2h), (f(y+h)-f(y-h))/(2h), (f(z+h)-f(z-h))/(2h))
///
/// # Arguments
/// * `f` - Scalar field function
/// * `x`, `y`, `z` - Evaluation point
/// * `h` - Step size for finite differences
pub fn gradient<F: Fn(f64, f64, f64) -> f64>(f: &F, x: f64, y: f64, z: f64, h: f64) -> Vector3 {
    let dfdx = (f(x + h, y, z) - f(x - h, y, z)) / (2.0 * h);
    let dfdy = (f(x, y + h, z) - f(x, y - h, z)) / (2.0 * h);
    let dfdz = (f(x, y, z + h) - f(x, y, z - h)) / (2.0 * h);
    Vector3::new(dfdx, dfdy, dfdz)
}

/// Compute the numerical divergence of a vector field at a point.
///
/// div(F) ≈ (Fx(x+h)-Fx(x-h))/(2h) + (Fy(y+h)-Fy(y-h))/(2h) + (Fz(z+h)-Fz(z-h))/(2h)
pub fn divergence<F: Fn(f64, f64, f64) -> Vector3>(
    f: &F,
    x: f64,
    y: f64,
    z: f64,
    h: f64,
) -> f64 {
    let dfx_dx = (f(x + h, y, z).x - f(x - h, y, z).x) / (2.0 * h);
    let dfy_dy = (f(x, y + h, z).y - f(x, y - h, z).y) / (2.0 * h);
    let dfz_dz = (f(x, y, z + h).z - f(x, y, z - h).z) / (2.0 * h);
    dfx_dx + dfy_dy + dfz_dz
}

/// Compute the numerical curl of a vector field at a point.
///
/// curl(F) = (∂Fz/∂y - ∂Fy/∂z, ∂Fx/∂z - ∂Fz/∂x, ∂Fy/∂x - ∂Fx/∂y)
pub fn curl<F: Fn(f64, f64, f64) -> Vector3>(f: &F, x: f64, y: f64, z: f64, h: f64) -> Vector3 {
    let dfz_dy = (f(x, y + h, z).z - f(x, y - h, z).z) / (2.0 * h);
    let dfy_dz = (f(x, y, z + h).y - f(x, y, z - h).y) / (2.0 * h);
    let dfx_dz = (f(x, y, z + h).x - f(x, y, z - h).x) / (2.0 * h);
    let dfz_dx = (f(x + h, y, z).z - f(x - h, y, z).z) / (2.0 * h);
    let dfy_dx = (f(x + h, y, z).y - f(x - h, y, z).y) / (2.0 * h);
    let dfx_dy = (f(x, y + h, z).x - f(x, y - h, z).x) / (2.0 * h);

    Vector3::new(dfz_dy - dfy_dz, dfx_dz - dfz_dx, dfy_dx - dfx_dy)
}

/// Compute the Laplacian of a scalar field: ∇²f = div(grad(f)).
pub fn laplacian<F: Fn(f64, f64, f64) -> f64>(f: &F, x: f64, y: f64, z: f64, h: f64) -> f64 {
    let d2f_dx2 = (f(x + h, y, z) - 2.0 * f(x, y, z) + f(x - h, y, z)) / (h * h);
    let d2f_dy2 = (f(x, y + h, z) - 2.0 * f(x, y, z) + f(x, y - h, z)) / (h * h);
    let d2f_dz2 = (f(x, y, z + h) - 2.0 * f(x, y, z) + f(x, y, z - h)) / (h * h);
    d2f_dx2 + d2f_dy2 + d2f_dz2
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scalar_field::ScalarFieldPreset;
    use crate::vector_field::VectorFieldPreset;
    use approx::assert_relative_eq;

    const H: f64 = 1e-5;

    // ================================================================
    // Gradient tests — compare numerical to analytical
    // ================================================================

    #[test]
    fn gradient_paraboloid() {
        let f = |x, y, z| ScalarFieldPreset::Paraboloid.evaluate(x, y, z);
        let g = gradient(&f, 1.0, 2.0, 3.0, H);
        let exact = ScalarFieldPreset::Paraboloid.gradient_exact(1.0, 2.0, 3.0);
        assert_relative_eq!(g.x, exact.x, max_relative = 1e-6);
        assert_relative_eq!(g.y, exact.y, max_relative = 1e-6);
        assert_relative_eq!(g.z, exact.z, max_relative = 1e-6);
    }

    #[test]
    fn gradient_saddle() {
        let f = |x, y, z| ScalarFieldPreset::Saddle.evaluate(x, y, z);
        let g = gradient(&f, 3.0, -2.0, 0.0, H);
        let exact = ScalarFieldPreset::Saddle.gradient_exact(3.0, -2.0, 0.0);
        assert_relative_eq!(g.x, exact.x, max_relative = 1e-6);
        assert_relative_eq!(g.y, exact.y, max_relative = 1e-6);
    }

    #[test]
    fn gradient_sincos() {
        let f = |x, y, z| ScalarFieldPreset::SinCos.evaluate(x, y, z);
        let g = gradient(&f, 0.5, 0.7, 0.0, H);
        let exact = ScalarFieldPreset::SinCos.gradient_exact(0.5, 0.7, 0.0);
        assert_relative_eq!(g.x, exact.x, max_relative = 1e-5);
        assert_relative_eq!(g.y, exact.y, max_relative = 1e-5);
    }

    #[test]
    fn gradient_gaussian() {
        let f = |x, y, z| ScalarFieldPreset::Gaussian.evaluate(x, y, z);
        let g = gradient(&f, 0.5, 0.3, 0.1, H);
        let exact = ScalarFieldPreset::Gaussian.gradient_exact(0.5, 0.3, 0.1);
        assert_relative_eq!(g.x, exact.x, max_relative = 1e-5);
        assert_relative_eq!(g.y, exact.y, max_relative = 1e-5);
        assert_relative_eq!(g.z, exact.z, max_relative = 1e-5);
    }

    // ================================================================
    // Divergence tests
    // ================================================================

    #[test]
    fn divergence_radial_outward() {
        let f = |x, y, z| VectorFieldPreset::RadialOutward.evaluate(x, y, z);
        let div = divergence(&f, 1.0, 2.0, 3.0, H);
        assert_relative_eq!(div, 3.0, max_relative = 1e-6);
    }

    #[test]
    fn divergence_rotation_is_zero() {
        let f = |x, y, z| VectorFieldPreset::Rotation2D.evaluate(x, y, z);
        let div = divergence(&f, 5.0, 3.0, 0.0, H);
        assert_relative_eq!(div, 0.0, epsilon = 1e-8);
    }

    #[test]
    fn divergence_uniform_is_zero() {
        let f = |x, y, z| VectorFieldPreset::UniformX.evaluate(x, y, z);
        let div = divergence(&f, 1.0, 1.0, 1.0, H);
        assert_relative_eq!(div, 0.0, epsilon = 1e-8);
    }

    #[test]
    fn divergence_radial_inward_is_minus_3() {
        let f = |x, y, z| VectorFieldPreset::RadialInward.evaluate(x, y, z);
        let div = divergence(&f, 1.0, 2.0, 3.0, H);
        assert_relative_eq!(div, -3.0, max_relative = 1e-6);
    }

    // ================================================================
    // Curl tests
    // ================================================================

    #[test]
    fn curl_rotation_2d() {
        let f = |x, y, z| VectorFieldPreset::Rotation2D.evaluate(x, y, z);
        let c = curl(&f, 1.0, 2.0, 0.0, H);
        assert_relative_eq!(c.z, 2.0, max_relative = 1e-6);
        assert_relative_eq!(c.x, 0.0, epsilon = 1e-8);
        assert_relative_eq!(c.y, 0.0, epsilon = 1e-8);
    }

    #[test]
    fn curl_radial_outward_is_zero() {
        let f = |x, y, z| VectorFieldPreset::RadialOutward.evaluate(x, y, z);
        let c = curl(&f, 1.0, 2.0, 3.0, H);
        assert_relative_eq!(c.magnitude(), 0.0, epsilon = 1e-8);
    }

    #[test]
    fn curl_uniform_is_zero() {
        let f = |x, y, z| VectorFieldPreset::UniformX.evaluate(x, y, z);
        let c = curl(&f, 5.0, 3.0, 1.0, H);
        assert_relative_eq!(c.magnitude(), 0.0, epsilon = 1e-8);
    }

    // ================================================================
    // Laplacian tests
    // ================================================================

    #[test]
    fn laplacian_paraboloid_is_constant() {
        // ∇²(x²+y²+z²) = 2+2+2 = 6
        let f = |x, y, z| ScalarFieldPreset::Paraboloid.evaluate(x, y, z);
        let lap = laplacian(&f, 1.0, 2.0, 3.0, H);
        assert_relative_eq!(lap, 6.0, max_relative = 1e-4);
    }

    #[test]
    fn laplacian_saddle() {
        // ∇²(x²-y²) = 2 - 2 + 0 = 0
        let f = |x, y, z| ScalarFieldPreset::Saddle.evaluate(x, y, z);
        let lap = laplacian(&f, 3.0, -2.0, 0.0, H);
        assert_relative_eq!(lap, 0.0, epsilon = 1e-4);
    }

    // ================================================================
    // Vector identity: div(curl(F)) = 0
    // ================================================================

    #[test]
    fn div_curl_is_zero() {
        let f = |x, y, z| VectorFieldPreset::NonUniform.evaluate(x, y, z);
        let curl_f = |x: f64, y: f64, z: f64| curl(&f, x, y, z, H);
        let div_curl = divergence(&curl_f, 1.0, 2.0, 0.0, H * 10.0); // wider h for nested finite diff
        assert_relative_eq!(div_curl, 0.0, epsilon = 0.1);
    }

    // ================================================================
    // Vector identity: curl(grad(f)) = 0
    // ================================================================

    #[test]
    fn curl_grad_is_zero() {
        let f = |x, y, z| ScalarFieldPreset::Gaussian.evaluate(x, y, z);
        let grad_f = |x: f64, y: f64, z: f64| gradient(&f, x, y, z, H);
        let c = curl(&grad_f, 0.5, 0.3, 0.1, H * 10.0);
        assert_relative_eq!(c.magnitude(), 0.0, epsilon = 0.1);
    }
}

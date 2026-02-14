//! Gauss's Law applications — E-field from symmetric charge distributions.

use std::f64::consts::PI;

const EPSILON_0: f64 = 8.854187817e-12;

/// E-field from infinite line charge.
/// E = ρ_L / (2π ε₀ ρ) ρ̂
pub fn e_line_charge(rho_l: f64, rho: f64, epsilon_r: f64) -> f64 {
    rho_l / (2.0 * PI * EPSILON_0 * epsilon_r * rho)
}

/// E-field from infinite surface charge.
/// E = ρ_s / (2 ε₀) (each side)
pub fn e_surface_charge(rho_s: f64, epsilon_r: f64) -> f64 {
    rho_s / (2.0 * EPSILON_0 * epsilon_r)
}

/// E-field from uniformly charged sphere (radius R, total charge Q) at distance r.
/// r < R: E = Qr / (4π ε₀ R³)
/// r ≥ R: E = Q / (4π ε₀ r²)
pub fn e_charged_sphere(total_charge: f64, radius: f64, r: f64, epsilon_r: f64) -> f64 {
    let eps = EPSILON_0 * epsilon_r;
    if r < radius {
        total_charge * r / (4.0 * PI * eps * radius.powi(3))
    } else {
        total_charge / (4.0 * PI * eps * r * r)
    }
}

/// E-field from coaxial cylinders at radius r.
/// a < r < b: E = ρ_L / (2π ε₀ r)
/// Elsewhere: E = 0 (assuming equal and opposite charge)
pub fn e_coaxial(rho_l: f64, inner_r: f64, outer_r: f64, r: f64, epsilon_r: f64) -> f64 {
    if r < inner_r || r > outer_r {
        0.0
    } else {
        rho_l / (2.0 * PI * EPSILON_0 * epsilon_r * r)
    }
}

/// E-field profile for charged sphere: E(r) from 0 to r_max.
pub fn sphere_e_profile(total_charge: f64, radius: f64, epsilon_r: f64, r_max: f64, n: usize) -> (Vec<f64>, Vec<f64>) {
    let mut rs = Vec::with_capacity(n);
    let mut es = Vec::with_capacity(n);
    for i in 0..n {
        let r = r_max * (i as f64 + 0.001) / n as f64;
        rs.push(r);
        es.push(e_charged_sphere(total_charge, radius, r, epsilon_r));
    }
    (rs, es)
}

/// Potential from uniformly charged sphere.
/// r < R: V = Q(3R² - r²) / (8π ε₀ R³)
/// r ≥ R: V = Q / (4π ε₀ r)
pub fn v_charged_sphere(total_charge: f64, radius: f64, r: f64, epsilon_r: f64) -> f64 {
    let eps = EPSILON_0 * epsilon_r;
    if r < radius {
        total_charge * (3.0 * radius * radius - r * r) / (8.0 * PI * eps * radius.powi(3))
    } else {
        total_charge / (4.0 * PI * eps * r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_charge() {
        let e = e_line_charge(1e-9, 0.1, 1.0);
        let expected = 1e-9 / (2.0 * PI * EPSILON_0 * 0.1);
        assert!((e - expected).abs() / expected < 1e-10);
    }

    #[test]
    fn test_surface_charge() {
        let e = e_surface_charge(1e-6, 1.0);
        let expected = 1e-6 / (2.0 * EPSILON_0);
        assert!((e - expected).abs() / expected < 1e-10);
    }

    #[test]
    fn test_sphere_interior_exterior() {
        let q = 1e-6;
        let r_sphere = 0.1;
        // At surface
        let e_surface = e_charged_sphere(q, r_sphere, r_sphere, 1.0);
        let expected = q / (4.0 * PI * EPSILON_0 * r_sphere * r_sphere);
        assert!((e_surface - expected).abs() / expected < 1e-10);
        // Interior should be less
        let e_half = e_charged_sphere(q, r_sphere, 0.05, 1.0);
        assert!(e_half < e_surface);
        // At center should be zero
        let e_center = e_charged_sphere(q, r_sphere, 1e-15, 1.0);
        assert!(e_center < 1e-3);
    }

    #[test]
    fn test_sphere_profile() {
        let (rs, es) = sphere_e_profile(1e-6, 0.1, 1.0, 0.3, 100);
        assert_eq!(rs.len(), 100);
        assert_eq!(es.len(), 100);
        // E should increase inside, then decrease as 1/r² outside
        let max_idx = es.iter().enumerate().max_by(|a, b| a.1.partial_cmp(b.1).unwrap()).unwrap().0;
        let r_max_e = rs[max_idx];
        assert!((r_max_e - 0.1).abs() < 0.01); // max near surface
    }

    #[test]
    fn test_coaxial_field() {
        let e_inside = e_coaxial(1e-9, 0.01, 0.05, 0.03, 1.0);
        assert!(e_inside > 0.0);
        let e_outside = e_coaxial(1e-9, 0.01, 0.05, 0.06, 1.0);
        assert_eq!(e_outside, 0.0);
    }

    #[test]
    fn test_potential_continuity() {
        let q = 1e-6;
        let r = 0.1;
        let v_in = v_charged_sphere(q, r, r - 1e-10, 1.0);
        let v_out = v_charged_sphere(q, r, r + 1e-10, 1.0);
        assert!((v_in - v_out).abs() / v_in < 1e-6);
    }
}

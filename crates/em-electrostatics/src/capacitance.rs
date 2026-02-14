//! Capacitance calculations for standard geometries.

use std::f64::consts::PI;

/// Parallel plate capacitor.
/// C = ε₀ εᵣ A / d
pub fn parallel_plate(area: f64, separation: f64, epsilon_r: f64) -> f64 {
    let eps0 = 8.854187817e-12;
    eps0 * epsilon_r * area / separation
}

/// Coaxial capacitor (per unit length).
/// C/L = 2π ε₀ εᵣ / ln(b/a)
pub fn coaxial_per_length(inner_r: f64, outer_r: f64, epsilon_r: f64) -> f64 {
    let eps0 = 8.854187817e-12;
    2.0 * PI * eps0 * epsilon_r / (outer_r / inner_r).ln()
}

/// Coaxial capacitor of given length.
pub fn coaxial(inner_r: f64, outer_r: f64, epsilon_r: f64, length: f64) -> f64 {
    coaxial_per_length(inner_r, outer_r, epsilon_r) * length
}

/// Spherical capacitor.
/// C = 4π ε₀ εᵣ a b / (b - a)
pub fn spherical(inner_r: f64, outer_r: f64, epsilon_r: f64) -> f64 {
    let eps0 = 8.854187817e-12;
    4.0 * PI * eps0 * epsilon_r * inner_r * outer_r / (outer_r - inner_r)
}

/// Isolated sphere.
/// C = 4π ε₀ a
pub fn isolated_sphere(radius: f64) -> f64 {
    let eps0 = 8.854187817e-12;
    4.0 * PI * eps0 * radius
}

/// Stored energy in capacitor.
/// W = ½ C V²
pub fn energy(capacitance: f64, voltage: f64) -> f64 {
    0.5 * capacitance * voltage * voltage
}

/// Charge on capacitor.
/// Q = C V
pub fn charge(capacitance: f64, voltage: f64) -> f64 {
    capacitance * voltage
}

/// Series combination of N capacitors.
pub fn series(caps: &[f64]) -> f64 {
    let inv_sum: f64 = caps.iter().map(|c| 1.0 / c).sum();
    1.0 / inv_sum
}

/// Parallel combination of N capacitors.
pub fn parallel(caps: &[f64]) -> f64 {
    caps.iter().sum()
}

/// Electric field in parallel plate capacitor.
/// E = V / d
pub fn field_parallel_plate(voltage: f64, separation: f64) -> f64 {
    voltage / separation
}

/// Energy density in parallel plate capacitor.
/// u = ½ ε₀ εᵣ E²
pub fn energy_density(e_field: f64, epsilon_r: f64) -> f64 {
    let eps0 = 8.854187817e-12;
    0.5 * eps0 * epsilon_r * e_field * e_field
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parallel_plate() {
        let c = parallel_plate(0.01, 0.001, 1.0);
        let expected = 8.854187817e-12 * 0.01 / 0.001;
        assert!((c - expected).abs() < 1e-20);
    }

    #[test]
    fn test_parallel_plate_dielectric() {
        let c1 = parallel_plate(0.01, 0.001, 1.0);
        let c4 = parallel_plate(0.01, 0.001, 4.0);
        assert!((c4 / c1 - 4.0).abs() < 1e-10);
    }

    #[test]
    fn test_coaxial() {
        let cpl = coaxial_per_length(0.001, 0.004, 2.25);
        assert!(cpl > 0.0);
        let c = coaxial(0.001, 0.004, 2.25, 1.0);
        assert!((c - cpl).abs() < 1e-20);
    }

    #[test]
    fn test_spherical() {
        let c = spherical(0.05, 0.1, 1.0);
        let expected = 4.0 * PI * 8.854187817e-12 * 0.05 * 0.1 / 0.05;
        assert!((c - expected).abs() < 1e-20);
    }

    #[test]
    fn test_isolated_sphere() {
        let c = isolated_sphere(1.0);
        let expected = 4.0 * PI * 8.854187817e-12;
        assert!((c - expected).abs() < 1e-20);
    }

    #[test]
    fn test_energy_and_charge() {
        let cap = 1e-6; // 1 μF
        let v = 100.0;
        let w = energy(cap, v);
        assert!((w - 0.005).abs() < 1e-10);
        let q = charge(cap, v);
        assert!((q - 1e-4).abs() < 1e-12);
    }

    #[test]
    fn test_series_parallel() {
        let caps = vec![1e-6, 1e-6];
        let s = series(&caps);
        assert!((s - 0.5e-6).abs() < 1e-15);
        let p = parallel(&caps);
        assert!((p - 2e-6).abs() < 1e-15);
    }

    #[test]
    fn test_field_and_energy_density() {
        let e = field_parallel_plate(100.0, 0.001);
        assert!((e - 1e5).abs() < 1e-5);
        let u = energy_density(e, 1.0);
        assert!(u > 0.0);
    }
}

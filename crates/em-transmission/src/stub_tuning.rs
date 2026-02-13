//! Single-stub and double-stub impedance matching.
//!
//! Computes stub placement and length for matching a complex load
//! to a transmission line using open or short-circuited stubs.

use em_core::complex::reflection_coefficient;
use num_complex::Complex64;
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

/// Stub termination type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StubType {
    Open,
    Short,
}

/// Result of a single-stub matching design.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SingleStubResult {
    /// Distance from load to stub attachment point (m)
    pub stub_distance: f64,
    /// Stub length (m)
    pub stub_length: f64,
    /// Stub distance in wavelengths
    pub stub_distance_wavelengths: f64,
    /// Stub length in wavelengths
    pub stub_length_wavelengths: f64,
    /// Stub termination type
    pub stub_type: StubType,
}

/// Design a single-stub matching network.
///
/// Finds the stub position d and length l that match Z_L to Z₀ on a lossless line.
///
/// # Arguments
/// * `z0` - Characteristic impedance (Ω)
/// * `z_load` - Complex load impedance (Ω)
/// * `frequency` - Operating frequency (Hz)
/// * `phase_velocity` - Phase velocity on the line (m/s)
/// * `stub_type` - Open or short circuit stub
///
/// # Returns
/// Two solutions (there are always exactly two stub positions per half-wavelength).
pub fn single_stub(
    z0: f64,
    z_load: Complex64,
    frequency: f64,
    phase_velocity: f64,
    stub_type: StubType,
) -> [SingleStubResult; 2] {
    let wavelength = phase_velocity / frequency;
    let beta = 2.0 * PI / wavelength;

    // Normalized load
    let z_l_norm = z_load / z0;
    let _r = z_l_norm.re;
    let _x = z_l_norm.im;

    // Distance d: we need the admittance at distance d to have real part = 1/Z₀
    // (i.e., normalized conductance g = 1)
    //
    // Using the formula: the two distances where g(d) = 1 are found from:
    // tan(βd) = [x ± √(r·((1-r)² + x²) / r)] / (r² + x² - r) ... simplified approach

    // Direct approach: sweep isn't TDD-friendly. Use analytical solution.
    // Γ_L = (z_l_norm - 1)/(z_l_norm + 1)
    let gamma_l = (z_l_norm - Complex64::new(1.0, 0.0)) / (z_l_norm + Complex64::new(1.0, 0.0));
    let gamma_mag = gamma_l.norm();
    let theta_r = gamma_l.arg();

    // Positions where the normalized conductance = 1:
    // These occur where |Γ|·cos(θ_r - 2βd) = 0... more precisely:
    // At distance d from load, the normalized input admittance is:
    // y(d) = (1 - Γ_L·e^{-j2βd}) / (1 + Γ_L·e^{-j2βd})
    // We need Re(y(d)) = 1.
    //
    // Let φ = θ_r - 2βd. Then:
    // g = (1 - |Γ|²) / (1 + |Γ|² + 2|Γ|cos(φ))
    // Setting g = 1: cos(φ) = -|Γ|/2 (when |Γ| ≤ 2, which it always is since |Γ| ≤ 1)
    // ... actually g = 1 when: 1 - |Γ|² = 1 + |Γ|² + 2|Γ|cos(φ)
    // → -2|Γ|² = 2|Γ|cos(φ) → cos(φ) = -|Γ|

    // Two solutions for φ:
    let phi_1 = (-gamma_mag).acos(); // φ₁ ∈ [0, π]
    let phi_2 = -phi_1; // φ₂ ∈ [-π, 0]

    let d_from_phi = |phi: f64| -> f64 {
        let mut d = (theta_r - phi) / (2.0 * beta);
        // Normalize to [0, λ/2)
        let half_wave = wavelength / 2.0;
        d %= half_wave;
        if d < 0.0 {
            d += half_wave;
        }
        d
    };

    let d1 = d_from_phi(phi_1);
    let d2 = d_from_phi(phi_2);

    // For each d, compute the susceptance b that needs to be cancelled by the stub
    let susceptance_at = |d: f64| -> f64 {
        let phase = Complex64::from_polar(1.0, -2.0 * beta * d);
        let gamma_d = gamma_l * phase;
        let one = Complex64::new(1.0, 0.0);
        let y = (one - gamma_d) / (one + gamma_d);
        y.im // normalized susceptance to cancel
    };

    let b1 = susceptance_at(d1);
    let b2 = susceptance_at(d2);

    // Stub length to produce susceptance -b:
    let stub_length_for = |b: f64, stype: StubType| -> f64 {
        let target_b = -b; // stub must cancel line susceptance
        let l = match stype {
            StubType::Short => {
                // Short stub: B_stub = -1/tan(βl) (normalized)
                // -1/tan(βl) = target_b → tan(βl) = -1/target_b
                (-1.0 / target_b).atan() / beta
            }
            StubType::Open => {
                // Open stub: B_stub = tan(βl) (normalized)
                // tan(βl) = target_b
                target_b.atan() / beta
            }
        };
        // Normalize to positive length
        let mut length = l % (wavelength / 2.0);
        if length < 0.0 {
            length += wavelength / 2.0;
        }
        length
    };

    let l1 = stub_length_for(b1, stub_type);
    let l2 = stub_length_for(b2, stub_type);

    [
        SingleStubResult {
            stub_distance: d1,
            stub_length: l1,
            stub_distance_wavelengths: d1 / wavelength,
            stub_length_wavelengths: l1 / wavelength,
            stub_type,
        },
        SingleStubResult {
            stub_distance: d2,
            stub_length: l2,
            stub_distance_wavelengths: d2 / wavelength,
            stub_length_wavelengths: l2 / wavelength,
            stub_type,
        },
    ]
}

/// Verify a single-stub solution by computing the reflection coefficient at the input.
pub fn verify_single_stub(
    z0: f64,
    z_load: Complex64,
    result: &SingleStubResult,
    frequency: f64,
    phase_velocity: f64,
) -> f64 {
    let beta = 2.0 * PI * frequency / phase_velocity;
    let z0c = Complex64::new(z0, 0.0);

    // Input impedance of line section from load to stub
    let z_at_stub = em_core::complex::input_impedance_lossless(z0, z_load, beta * result.stub_distance);

    // Stub input impedance
    let z_stub = match result.stub_type {
        StubType::Short => {
            // Short-circuited stub: Z = jZ₀·tan(βl)
            let j = Complex64::new(0.0, 1.0);
            j * z0 * (beta * result.stub_length).tan()
        }
        StubType::Open => {
            // Open-circuited stub: Z = -jZ₀/tan(βl) = -jZ₀·cot(βl)
            let j = Complex64::new(0.0, 1.0);
            -j * z0 / (beta * result.stub_length).tan()
        }
    };

    // Parallel combination at stub point (use admittances)
    let y_line = Complex64::new(1.0, 0.0) / z_at_stub;
    let y_stub = Complex64::new(1.0, 0.0) / z_stub;
    let y_total = y_line + y_stub;
    let z_total = Complex64::new(1.0, 0.0) / y_total;

    // Reflection coefficient looking into the matched section
    reflection_coefficient(z_total, z0c).norm()
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    fn test_params() -> (f64, Complex64, f64, f64) {
        (50.0, Complex64::new(25.0, 50.0), 1e9, em_core::constants::C_0)
    }

    #[test]
    fn single_stub_returns_two_solutions() {
        let (z0, zl, f, vp) = test_params();
        let results = single_stub(z0, zl, f, vp, StubType::Short);
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn single_stub_distances_are_positive() {
        let (z0, zl, f, vp) = test_params();
        for stype in [StubType::Short, StubType::Open] {
            let results = single_stub(z0, zl, f, vp, stype);
            for r in &results {
                assert!(r.stub_distance >= 0.0, "distance must be non-negative");
                assert!(r.stub_length >= 0.0, "length must be non-negative");
            }
        }
    }

    #[test]
    fn single_stub_distances_within_half_wavelength() {
        let (z0, zl, f, vp) = test_params();
        let wavelength = vp / f;
        for stype in [StubType::Short, StubType::Open] {
            let results = single_stub(z0, zl, f, vp, stype);
            for r in &results {
                assert!(
                    r.stub_distance < wavelength / 2.0 + 1e-10,
                    "distance should be < λ/2"
                );
            }
        }
    }

    #[test]
    fn single_stub_short_achieves_match() {
        let (z0, zl, f, vp) = test_params();
        let results = single_stub(z0, zl, f, vp, StubType::Short);
        // At least one solution should give good match
        let best = results
            .iter()
            .map(|r| verify_single_stub(z0, zl, r, f, vp))
            .fold(f64::INFINITY, f64::min);
        assert!(best < 0.05, "best |Γ| should be < 0.05, got {best}");
    }

    #[test]
    fn single_stub_open_achieves_match() {
        let (z0, zl, f, vp) = test_params();
        let results = single_stub(z0, zl, f, vp, StubType::Open);
        let best = results
            .iter()
            .map(|r| verify_single_stub(z0, zl, r, f, vp))
            .fold(f64::INFINITY, f64::min);
        assert!(best < 0.05, "best |Γ| should be < 0.05, got {best}");
    }

    #[test]
    fn single_stub_purely_resistive_load() {
        let z0 = 50.0;
        let zl = Complex64::new(100.0, 0.0);
        let f = 1e9;
        let vp = em_core::constants::C_0;
        let results = single_stub(z0, zl, f, vp, StubType::Short);
        let best = results
            .iter()
            .map(|r| verify_single_stub(z0, zl, r, f, vp))
            .fold(f64::INFINITY, f64::min);
        assert!(best < 0.05, "should match resistive load, got |Γ| = {best}");
    }

    #[test]
    fn single_stub_wavelengths_consistent() {
        let (z0, zl, f, vp) = test_params();
        let wavelength = vp / f;
        let results = single_stub(z0, zl, f, vp, StubType::Short);
        for r in &results {
            assert_relative_eq!(
                r.stub_distance_wavelengths,
                r.stub_distance / wavelength,
                epsilon = 1e-12
            );
            assert_relative_eq!(
                r.stub_length_wavelengths,
                r.stub_length / wavelength,
                epsilon = 1e-12
            );
        }
    }
}

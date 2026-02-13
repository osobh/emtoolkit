//! Extended complex number operations for electromagnetic computations.
//!
//! Wraps `num_complex::Complex64` with EM-specific operations:
//! - Polar ↔ rectangular conversion
//! - Reflection coefficient ↔ impedance mapping
//! - Phasor arithmetic
//! - Complex propagation constant decomposition (α + jβ)

use num_complex::Complex64;
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

/// A phasor representation: magnitude and phase angle.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Phasor {
    /// Magnitude (always non-negative)
    pub magnitude: f64,
    /// Phase angle in radians
    pub phase_rad: f64,
}

impl Phasor {
    /// Create a new phasor from magnitude and phase in radians.
    pub fn new(magnitude: f64, phase_rad: f64) -> Self {
        Self {
            magnitude: magnitude.abs(),
            phase_rad: if magnitude < 0.0 {
                normalize_angle(phase_rad + PI)
            } else {
                normalize_angle(phase_rad)
            },
        }
    }

    /// Create a phasor from magnitude and phase in degrees.
    pub fn from_degrees(magnitude: f64, phase_deg: f64) -> Self {
        Self::new(magnitude, phase_deg.to_radians())
    }

    /// Convert to rectangular Complex64.
    pub fn to_complex(self) -> Complex64 {
        Complex64::from_polar(self.magnitude, self.phase_rad)
    }

    /// Create a phasor from a Complex64.
    pub fn from_complex(z: Complex64) -> Self {
        Self {
            magnitude: z.norm(),
            phase_rad: z.arg(),
        }
    }

    /// Phase angle in degrees.
    pub fn phase_deg(&self) -> f64 {
        self.phase_rad.to_degrees()
    }
}

/// Normalize an angle to the range (-π, π].
pub fn normalize_angle(angle: f64) -> f64 {
    let mut a = angle % (2.0 * PI);
    if a > PI {
        a -= 2.0 * PI;
    } else if a <= -PI {
        a += 2.0 * PI;
    }
    a
}

/// Compute the voltage reflection coefficient Γ = (Z_L - Z_0) / (Z_L + Z_0).
///
/// # Arguments
/// * `z_load` - Complex load impedance (Ω)
/// * `z_0` - Characteristic impedance (Ω), typically real
///
/// # Returns
/// Complex reflection coefficient. |Γ| ≤ 1 for passive loads.
pub fn reflection_coefficient(z_load: Complex64, z_0: Complex64) -> Complex64 {
    (z_load - z_0) / (z_load + z_0)
}

/// Compute the load impedance from reflection coefficient: Z_L = Z_0 · (1 + Γ) / (1 - Γ).
///
/// # Arguments
/// * `gamma` - Complex reflection coefficient
/// * `z_0` - Characteristic impedance (Ω)
///
/// # Returns
/// Complex load impedance.
pub fn impedance_from_gamma(gamma: Complex64, z_0: Complex64) -> Complex64 {
    z_0 * (Complex64::new(1.0, 0.0) + gamma) / (Complex64::new(1.0, 0.0) - gamma)
}

/// Compute the Voltage Standing Wave Ratio from reflection coefficient magnitude.
///
/// VSWR = (1 + |Γ|) / (1 - |Γ|)
///
/// # Returns
/// VSWR ≥ 1. Returns `f64::INFINITY` if |Γ| = 1.
pub fn vswr(gamma: Complex64) -> f64 {
    let mag = gamma.norm();
    (1.0 + mag) / (1.0 - mag)
}

/// Decompose a complex propagation constant γ into attenuation and phase constants.
///
/// γ = α + jβ where α is the attenuation constant (Np/m) and β is the phase constant (rad/m).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PropagationConstant {
    /// Attenuation constant α (Np/m)
    pub alpha: f64,
    /// Phase constant β (rad/m)
    pub beta: f64,
}

impl PropagationConstant {
    /// Create from a complex propagation constant γ = α + jβ.
    pub fn from_complex(gamma: Complex64) -> Self {
        Self {
            alpha: gamma.re,
            beta: gamma.im,
        }
    }

    /// Convert back to complex form.
    pub fn to_complex(self) -> Complex64 {
        Complex64::new(self.alpha, self.beta)
    }

    /// Compute the complex propagation constant for a lossy medium.
    ///
    /// γ = jω√(μ(ε - jσ/ω)) = jω√(με)·√(1 - jσ/(ωε))
    ///
    /// # Arguments
    /// * `omega` - Angular frequency (rad/s)
    /// * `mu` - Permeability (H/m)
    /// * `epsilon` - Permittivity (F/m)
    /// * `sigma` - Conductivity (S/m)
    pub fn for_lossy_medium(omega: f64, mu: f64, epsilon: f64, sigma: f64) -> Self {
        let j = Complex64::new(0.0, 1.0);
        let complex_eps = Complex64::new(epsilon, -sigma / omega);
        let gamma = j * omega * (mu * complex_eps).sqrt();
        // Ensure α ≥ 0 (wave decays in propagation direction)
        let result = if gamma.re < 0.0 { -gamma } else { gamma };
        Self::from_complex(result)
    }

    /// Wavelength in the medium: λ = 2π/β
    pub fn wavelength(&self) -> f64 {
        2.0 * PI / self.beta
    }

    /// Phase velocity: v_p = ω/β
    pub fn phase_velocity(&self, omega: f64) -> f64 {
        omega / self.beta
    }
}

/// Compute the complex impedance of a transmission line section.
///
/// Z_in = Z_0 · (Z_L + Z_0·tanh(γl)) / (Z_0 + Z_L·tanh(γl))
///
/// # Arguments
/// * `z_0` - Characteristic impedance (complex for lossy lines)
/// * `z_load` - Load impedance
/// * `gamma` - Complex propagation constant
/// * `length` - Line length (m)
pub fn input_impedance_lossy(
    z_0: Complex64,
    z_load: Complex64,
    gamma: Complex64,
    length: f64,
) -> Complex64 {
    let gl = gamma * length;
    let tanh_gl = gl.tanh();
    z_0 * (z_load + z_0 * tanh_gl) / (z_0 + z_load * tanh_gl)
}

/// Compute the input impedance of a lossless transmission line section.
///
/// Z_in = Z_0 · (Z_L + jZ_0·tan(βl)) / (Z_0 + jZ_L·tan(βl))
///
/// # Arguments
/// * `z_0` - Real characteristic impedance (Ω)
/// * `z_load` - Complex load impedance (Ω)
/// * `beta_l` - Electrical length β·l (radians)
pub fn input_impedance_lossless(z_0: f64, z_load: Complex64, beta_l: f64) -> Complex64 {
    let j = Complex64::new(0.0, 1.0);
    let z0c = Complex64::new(z_0, 0.0);
    let tan_bl = Complex64::new(beta_l.tan(), 0.0);
    z0c * (z_load + j * z0c * tan_bl) / (z0c + j * z_load * tan_bl)
}

/// Add two phasors (complex addition in polar form, returning polar).
pub fn phasor_add(a: Phasor, b: Phasor) -> Phasor {
    Phasor::from_complex(a.to_complex() + b.to_complex())
}

/// Multiply two phasors: magnitudes multiply, phases add.
pub fn phasor_multiply(a: Phasor, b: Phasor) -> Phasor {
    Phasor::new(a.magnitude * b.magnitude, a.phase_rad + b.phase_rad)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use std::f64::consts::PI;

    // ================================================================
    // Phasor tests
    // ================================================================

    #[test]
    fn phasor_roundtrip_to_complex_and_back() {
        let p = Phasor::new(5.0, PI / 4.0);
        let z = p.to_complex();
        let p2 = Phasor::from_complex(z);
        assert_relative_eq!(p.magnitude, p2.magnitude, epsilon = 1e-12);
        assert_relative_eq!(p.phase_rad, p2.phase_rad, epsilon = 1e-12);
    }

    #[test]
    fn phasor_from_degrees() {
        let p = Phasor::from_degrees(10.0, 90.0);
        assert_relative_eq!(p.magnitude, 10.0, epsilon = 1e-12);
        assert_relative_eq!(p.phase_rad, PI / 2.0, epsilon = 1e-12);
    }

    #[test]
    fn phasor_negative_magnitude_flips_phase() {
        let p = Phasor::new(-3.0, 0.0);
        assert_relative_eq!(p.magnitude, 3.0, epsilon = 1e-12);
        assert_relative_eq!(p.phase_rad, PI, epsilon = 1e-12);
    }

    #[test]
    fn phasor_phase_deg_conversion() {
        let p = Phasor::new(1.0, PI / 3.0);
        assert_relative_eq!(p.phase_deg(), 60.0, epsilon = 1e-10);
    }

    // ================================================================
    // Normalize angle tests
    // ================================================================

    #[test]
    fn normalize_angle_within_range_unchanged() {
        assert_relative_eq!(normalize_angle(1.0), 1.0, epsilon = 1e-12);
        assert_relative_eq!(normalize_angle(-1.0), -1.0, epsilon = 1e-12);
    }

    #[test]
    fn normalize_angle_wraps_positive() {
        assert_relative_eq!(normalize_angle(3.0 * PI), PI, epsilon = 1e-12);
    }

    #[test]
    fn normalize_angle_wraps_negative() {
        assert_relative_eq!(normalize_angle(-3.0 * PI), -PI + 2.0 * PI, epsilon = 1e-10);
    }

    // ================================================================
    // Reflection coefficient tests
    // ================================================================

    #[test]
    fn gamma_matched_load_is_zero() {
        let z0 = Complex64::new(50.0, 0.0);
        let gamma = reflection_coefficient(z0, z0);
        assert_relative_eq!(gamma.norm(), 0.0, epsilon = 1e-12);
    }

    #[test]
    fn gamma_open_circuit_is_plus_one() {
        let z0 = Complex64::new(50.0, 0.0);
        let z_open = Complex64::new(1e15, 0.0); // approximate open
        let gamma = reflection_coefficient(z_open, z0);
        assert_relative_eq!(gamma.re, 1.0, epsilon = 1e-6);
        assert_relative_eq!(gamma.im, 0.0, epsilon = 1e-6);
    }

    #[test]
    fn gamma_short_circuit_is_minus_one() {
        let z0 = Complex64::new(50.0, 0.0);
        let z_short = Complex64::new(0.0, 0.0);
        let gamma = reflection_coefficient(z_short, z0);
        assert_relative_eq!(gamma.re, -1.0, epsilon = 1e-12);
        assert_relative_eq!(gamma.im, 0.0, epsilon = 1e-12);
    }

    #[test]
    fn gamma_complex_load_25_plus_j50() {
        // ZL = 25 + j50 Ω, Z0 = 50 Ω
        // Γ = (-25 + j50)/(75 + j50) = 0.6202∠82.87°
        let z0 = Complex64::new(50.0, 0.0);
        let zl = Complex64::new(25.0, 50.0);
        let gamma = reflection_coefficient(zl, z0);
        assert_relative_eq!(gamma.norm(), 0.6202, max_relative = 0.01);
        assert_relative_eq!(gamma.arg().to_degrees(), 82.87, max_relative = 0.01);
    }

    // ================================================================
    // Impedance from Gamma roundtrip
    // ================================================================

    #[test]
    fn impedance_from_gamma_roundtrip() {
        let z0 = Complex64::new(50.0, 0.0);
        let zl = Complex64::new(25.0, 50.0);
        let gamma = reflection_coefficient(zl, z0);
        let zl_recovered = impedance_from_gamma(gamma, z0);
        assert_relative_eq!(zl_recovered.re, zl.re, epsilon = 1e-10);
        assert_relative_eq!(zl_recovered.im, zl.im, epsilon = 1e-10);
    }

    // ================================================================
    // VSWR tests
    // ================================================================

    #[test]
    fn vswr_matched_load_is_1() {
        let gamma = Complex64::new(0.0, 0.0);
        assert_relative_eq!(vswr(gamma), 1.0, epsilon = 1e-12);
    }

    #[test]
    fn vswr_total_reflection_is_infinity() {
        let gamma = Complex64::new(-1.0, 0.0);
        assert!(vswr(gamma).is_infinite());
    }

    #[test]
    fn vswr_half_reflection() {
        // |Γ| = 0.5 → VSWR = 1.5/0.5 = 3.0
        let gamma = Complex64::new(0.5, 0.0);
        assert_relative_eq!(vswr(gamma), 3.0, epsilon = 1e-12);
    }

    // ================================================================
    // Propagation constant tests
    // ================================================================

    #[test]
    fn propagation_constant_lossless_medium() {
        // Lossless medium: α = 0, β = ω√(με)
        let omega = 2.0 * PI * 1.0e9;
        let mu = crate::constants::MU_0;
        let epsilon = crate::constants::EPSILON_0;
        let pc = PropagationConstant::for_lossy_medium(omega, mu, epsilon, 0.0);
        assert_relative_eq!(pc.alpha, 0.0, epsilon = 1e-6);
        let expected_beta = omega * (mu * epsilon).sqrt();
        assert_relative_eq!(pc.beta, expected_beta, max_relative = 1e-6);
    }

    #[test]
    fn propagation_constant_lossy_has_positive_alpha() {
        let omega = 2.0 * PI * 1.0e9;
        let mu = crate::constants::MU_0;
        let epsilon = crate::constants::EPSILON_0;
        let sigma = 0.01; // slightly lossy
        let pc = PropagationConstant::for_lossy_medium(omega, mu, epsilon, sigma);
        assert!(pc.alpha > 0.0, "attenuation must be positive for lossy medium");
        assert!(pc.beta > 0.0, "phase constant must be positive");
    }

    #[test]
    fn propagation_constant_wavelength() {
        let pc = PropagationConstant {
            alpha: 0.0,
            beta: 2.0 * PI,
        };
        assert_relative_eq!(pc.wavelength(), 1.0, epsilon = 1e-12);
    }

    #[test]
    fn propagation_constant_phase_velocity() {
        let omega = 2.0 * PI * 1.0e9;
        let beta = omega / crate::constants::C_0;
        let pc = PropagationConstant { alpha: 0.0, beta };
        assert_relative_eq!(
            pc.phase_velocity(omega),
            crate::constants::C_0,
            max_relative = 1e-6
        );
    }

    // ================================================================
    // Input impedance tests
    // ================================================================

    #[test]
    fn input_impedance_half_wave_line_equals_load() {
        // βl = π (half wavelength) → Zin = ZL
        let z0 = 50.0;
        let zl = Complex64::new(75.0, 25.0);
        let zin = input_impedance_lossless(z0, zl, PI);
        assert_relative_eq!(zin.re, zl.re, epsilon = 1e-8);
        assert_relative_eq!(zin.im, zl.im, epsilon = 1e-8);
    }

    #[test]
    fn input_impedance_quarter_wave_transformer() {
        // βl = π/2 (quarter wave) → Zin = Z0²/ZL
        let z0 = 50.0;
        let zl = Complex64::new(100.0, 0.0);
        let zin = input_impedance_lossless(z0, zl, PI / 2.0);
        let expected = z0 * z0 / 100.0; // 25 Ω
        assert_relative_eq!(zin.re, expected, epsilon = 1e-8);
        assert_relative_eq!(zin.im, 0.0, epsilon = 1e-8);
    }

    #[test]
    fn input_impedance_short_circuit_quarter_wave_is_open() {
        // Shorted line, λ/4 → Zin → ∞ (very large)
        let z0 = 50.0;
        let zl = Complex64::new(0.0, 0.0);
        let zin = input_impedance_lossless(z0, zl, PI / 2.0);
        // tan(π/2) is very large, so Zin ≈ -jZ0/tan(ε) which is huge
        assert!(zin.norm() > 1e10, "shorted quarter-wave should look like open circuit");
    }

    #[test]
    fn input_impedance_lossy_reduces_to_lossless() {
        // With α = 0, lossy formula should match lossless
        let z0 = Complex64::new(50.0, 0.0);
        let zl = Complex64::new(75.0, -25.0);
        let beta = 2.0 * PI; // β = 2π → βl = 2π·l
        let length = 0.3;
        let gamma = Complex64::new(0.0, beta); // lossless: α=0

        let zin_lossy = input_impedance_lossy(z0, zl, gamma, length);
        let zin_lossless = input_impedance_lossless(50.0, zl, beta * length);

        assert_relative_eq!(zin_lossy.re, zin_lossless.re, max_relative = 1e-10);
        assert_relative_eq!(zin_lossy.im, zin_lossless.im, max_relative = 1e-10);
    }

    // ================================================================
    // Phasor arithmetic tests
    // ================================================================

    #[test]
    fn phasor_add_same_phase() {
        let a = Phasor::new(3.0, 0.0);
        let b = Phasor::new(4.0, 0.0);
        let sum = phasor_add(a, b);
        assert_relative_eq!(sum.magnitude, 7.0, epsilon = 1e-12);
        assert_relative_eq!(sum.phase_rad, 0.0, epsilon = 1e-12);
    }

    #[test]
    fn phasor_add_opposite_phase_cancels() {
        let a = Phasor::new(5.0, 0.0);
        let b = Phasor::new(5.0, PI);
        let sum = phasor_add(a, b);
        assert_relative_eq!(sum.magnitude, 0.0, epsilon = 1e-10);
    }

    #[test]
    fn phasor_add_orthogonal() {
        let a = Phasor::new(3.0, 0.0);
        let b = Phasor::new(4.0, PI / 2.0);
        let sum = phasor_add(a, b);
        assert_relative_eq!(sum.magnitude, 5.0, epsilon = 1e-10);
    }

    #[test]
    fn phasor_multiply_magnitudes_and_phases() {
        let a = Phasor::new(3.0, PI / 6.0);
        let b = Phasor::new(2.0, PI / 3.0);
        let product = phasor_multiply(a, b);
        assert_relative_eq!(product.magnitude, 6.0, epsilon = 1e-12);
        assert_relative_eq!(product.phase_rad, PI / 2.0, epsilon = 1e-12);
    }
}

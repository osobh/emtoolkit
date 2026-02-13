//! Transmission line parameter computation for common line geometries.
//!
//! Computes characteristic impedance Z₀, propagation constant γ, and
//! per-unit-length parameters (R, L, G, C) for:
//! - Two-wire transmission line
//! - Coaxial cable
//! - Microstrip line

use em_core::constants::{self, EPSILON_0, MU_0};
use num_complex::Complex64;
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

/// Per-unit-length parameters of a transmission line.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct LineParameters {
    /// Resistance per unit length (Ω/m)
    pub r_per_m: f64,
    /// Inductance per unit length (H/m)
    pub l_per_m: f64,
    /// Conductance per unit length (S/m)
    pub g_per_m: f64,
    /// Capacitance per unit length (F/m)
    pub c_per_m: f64,
}

impl LineParameters {
    /// Compute complex characteristic impedance Z₀ = √((R+jωL)/(G+jωC)).
    pub fn characteristic_impedance(&self, frequency: f64) -> Complex64 {
        let omega = 2.0 * PI * frequency;
        let z_series = Complex64::new(self.r_per_m, omega * self.l_per_m);
        let y_shunt = Complex64::new(self.g_per_m, omega * self.c_per_m);
        (z_series / y_shunt).sqrt()
    }

    /// Compute complex propagation constant γ = √((R+jωL)(G+jωC)).
    pub fn propagation_constant(&self, frequency: f64) -> Complex64 {
        let omega = 2.0 * PI * frequency;
        let z_series = Complex64::new(self.r_per_m, omega * self.l_per_m);
        let y_shunt = Complex64::new(self.g_per_m, omega * self.c_per_m);
        let gamma = (z_series * y_shunt).sqrt();
        // Ensure α ≥ 0
        if gamma.re < 0.0 { -gamma } else { gamma }
    }

    /// Lossless characteristic impedance Z₀ = √(L/C).
    pub fn z0_lossless(&self) -> f64 {
        (self.l_per_m / self.c_per_m).sqrt()
    }

    /// Lossless phase velocity v_p = 1/√(LC).
    pub fn phase_velocity_lossless(&self) -> f64 {
        1.0 / (self.l_per_m * self.c_per_m).sqrt()
    }
}

/// Two-wire transmission line geometry and parameters.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct TwoWireLine {
    /// Wire radius (m)
    pub wire_radius: f64,
    /// Center-to-center separation (m)
    pub separation: f64,
    /// Relative permittivity of surrounding medium
    pub epsilon_r: f64,
    /// Relative permeability of surrounding medium
    pub mu_r: f64,
    /// Conductivity of wire material (S/m)
    pub sigma_conductor: f64,
    /// Conductivity of dielectric (S/m)
    pub sigma_dielectric: f64,
}

impl TwoWireLine {
    /// Create a lossless two-wire line in a dielectric medium.
    pub fn lossless(wire_radius: f64, separation: f64, epsilon_r: f64) -> Self {
        Self {
            wire_radius,
            separation,
            epsilon_r,
            mu_r: 1.0,
            sigma_conductor: 0.0,
            sigma_dielectric: 0.0,
        }
    }

    /// Compute per-unit-length parameters.
    ///
    /// For d >> a (wire separation >> wire radius):
    /// - L = (μ/π) · acosh(d/(2a)) ≈ (μ/π) · ln(d/a) for d >> a
    /// - C = πε / acosh(d/(2a))
    pub fn parameters(&self, frequency: f64) -> LineParameters {
        let mu = self.mu_r * MU_0;
        let epsilon = self.epsilon_r * EPSILON_0;
        let ratio = self.separation / (2.0 * self.wire_radius);
        let acosh_val = (ratio).acosh();

        let l_per_m = mu * acosh_val / PI;
        let c_per_m = PI * epsilon / acosh_val;

        // AC resistance due to skin effect: R = 1/(πaδσ) per wire, ×2 for both wires
        let r_per_m = if self.sigma_conductor > 0.0 && frequency > 0.0 {
            let delta = constants::skin_depth(frequency, mu, self.sigma_conductor);
            2.0 / (PI * self.wire_radius * delta * self.sigma_conductor)
        } else {
            0.0
        };

        let g_per_m = if self.sigma_dielectric > 0.0 {
            PI * self.sigma_dielectric / acosh_val
        } else {
            0.0
        };

        LineParameters {
            r_per_m,
            l_per_m,
            g_per_m,
            c_per_m,
        }
    }
}

/// Coaxial cable geometry and parameters.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct CoaxialLine {
    /// Inner conductor radius (m)
    pub inner_radius: f64,
    /// Outer conductor inner radius (m)
    pub outer_radius: f64,
    /// Relative permittivity of dielectric fill
    pub epsilon_r: f64,
    /// Relative permeability of dielectric fill
    pub mu_r: f64,
    /// Conductivity of conductor material (S/m)
    pub sigma_conductor: f64,
    /// Conductivity of dielectric (S/m)
    pub sigma_dielectric: f64,
}

impl CoaxialLine {
    /// Create a lossless coaxial line.
    pub fn lossless(inner_radius: f64, outer_radius: f64, epsilon_r: f64) -> Self {
        Self {
            inner_radius,
            outer_radius,
            epsilon_r,
            mu_r: 1.0,
            sigma_conductor: 0.0,
            sigma_dielectric: 0.0,
        }
    }

    /// Compute per-unit-length parameters.
    ///
    /// - L = (μ/(2π)) · ln(b/a)
    /// - C = 2πε / ln(b/a)
    pub fn parameters(&self, frequency: f64) -> LineParameters {
        let mu = self.mu_r * MU_0;
        let epsilon = self.epsilon_r * EPSILON_0;
        let ln_ratio = (self.outer_radius / self.inner_radius).ln();

        let l_per_m = mu * ln_ratio / (2.0 * PI);
        let c_per_m = 2.0 * PI * epsilon / ln_ratio;

        let r_per_m = if self.sigma_conductor > 0.0 && frequency > 0.0 {
            let delta = constants::skin_depth(frequency, mu, self.sigma_conductor);
            // R for inner + outer conductor
            let r_inner = 1.0 / (2.0 * PI * self.inner_radius * delta * self.sigma_conductor);
            let r_outer = 1.0 / (2.0 * PI * self.outer_radius * delta * self.sigma_conductor);
            r_inner + r_outer
        } else {
            0.0
        };

        let g_per_m = if self.sigma_dielectric > 0.0 {
            2.0 * PI * self.sigma_dielectric / ln_ratio
        } else {
            0.0
        };

        LineParameters {
            r_per_m,
            l_per_m,
            g_per_m,
            c_per_m,
        }
    }
}

/// Microstrip line geometry and parameters.
///
/// Uses the Hammerstad-Jensen model for effective permittivity and impedance.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct MicrostripLine {
    /// Strip width (m)
    pub width: f64,
    /// Substrate height (m)
    pub height: f64,
    /// Substrate relative permittivity
    pub epsilon_r: f64,
    /// Strip thickness (m), 0 for infinitely thin
    pub thickness: f64,
}

impl MicrostripLine {
    /// Create a microstrip line with zero-thickness strip.
    pub fn new(width: f64, height: f64, epsilon_r: f64) -> Self {
        Self {
            width,
            height,
            epsilon_r,
            thickness: 0.0,
        }
    }

    /// Effective relative permittivity using Hammerstad-Jensen model.
    ///
    /// ε_eff = (ε_r + 1)/2 + (ε_r - 1)/2 · F(w/h)
    pub fn effective_epsilon_r(&self) -> f64 {
        let u = self.width / self.height;
        let f = if u <= 1.0 {
            (1.0 + 12.0 / u).powf(-0.5) + 0.04 * (1.0 - u).powi(2)
        } else {
            (1.0 + 12.0 / u).powf(-0.5)
        };
        (self.epsilon_r + 1.0) / 2.0 + (self.epsilon_r - 1.0) / 2.0 * f
    }

    /// Characteristic impedance using Hammerstad-Jensen model (Ω).
    pub fn characteristic_impedance(&self) -> f64 {
        let u = self.width / self.height;
        let eps_eff = self.effective_epsilon_r();

        if u <= 1.0 {
            // Narrow strip
            (60.0 / eps_eff.sqrt()) * ((8.0 / u + u / 4.0).ln())
        } else {
            // Wide strip
            (120.0 * PI) / (eps_eff.sqrt() * (u + 1.393 + 0.667 * (u + 1.444).ln()))
        }
    }

    /// Phase velocity in the microstrip (m/s).
    pub fn phase_velocity(&self) -> f64 {
        constants::C_0 / self.effective_epsilon_r().sqrt()
    }

    /// Compute approximate per-unit-length parameters (lossless).
    pub fn parameters(&self) -> LineParameters {
        let z0 = self.characteristic_impedance();
        let v_p = self.phase_velocity();
        let l_per_m = z0 / v_p;
        let c_per_m = 1.0 / (z0 * v_p);

        LineParameters {
            r_per_m: 0.0,
            l_per_m,
            g_per_m: 0.0,
            c_per_m,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    // ================================================================
    // LineParameters tests
    // ================================================================

    #[test]
    fn lossless_z0_equals_sqrt_l_over_c() {
        let p = LineParameters {
            r_per_m: 0.0,
            l_per_m: 250e-9,
            g_per_m: 0.0,
            c_per_m: 100e-12,
        };
        assert_relative_eq!(p.z0_lossless(), 50.0, epsilon = 1e-10);
    }

    #[test]
    fn lossless_z0_matches_complex_z0_at_high_freq() {
        let p = LineParameters {
            r_per_m: 0.0,
            l_per_m: 250e-9,
            g_per_m: 0.0,
            c_per_m: 100e-12,
        };
        let z0_complex = p.characteristic_impedance(1e9);
        assert_relative_eq!(z0_complex.re, 50.0, epsilon = 1e-6);
        assert_relative_eq!(z0_complex.im, 0.0, epsilon = 1e-6);
    }

    #[test]
    fn lossy_z0_has_negative_imaginary_part() {
        // A lossy line typically has Z0 with small negative imaginary part
        let p = LineParameters {
            r_per_m: 0.5,
            l_per_m: 250e-9,
            g_per_m: 0.0,
            c_per_m: 100e-12,
        };
        let z0 = p.characteristic_impedance(1e9);
        assert!(z0.re > 0.0, "real part should be positive");
        assert!(z0.im < 0.0, "imaginary part should be negative for series loss");
    }

    #[test]
    fn propagation_constant_lossless_has_zero_alpha() {
        let p = LineParameters {
            r_per_m: 0.0,
            l_per_m: 250e-9,
            g_per_m: 0.0,
            c_per_m: 100e-12,
        };
        let gamma = p.propagation_constant(1e9);
        assert_relative_eq!(gamma.re, 0.0, epsilon = 1e-10);
        assert!(gamma.im > 0.0, "β should be positive");
    }

    #[test]
    fn propagation_constant_lossy_has_positive_alpha() {
        let p = LineParameters {
            r_per_m: 0.5,
            l_per_m: 250e-9,
            g_per_m: 1e-5,
            c_per_m: 100e-12,
        };
        let gamma = p.propagation_constant(1e9);
        assert!(gamma.re > 0.0, "α should be positive for lossy line");
        assert!(gamma.im > 0.0, "β should be positive");
    }

    #[test]
    fn phase_velocity_lossless_free_space() {
        let p = LineParameters {
            r_per_m: 0.0,
            l_per_m: MU_0,
            g_per_m: 0.0,
            c_per_m: EPSILON_0,
        };
        assert_relative_eq!(
            p.phase_velocity_lossless(),
            constants::C_0,
            max_relative = 1e-6
        );
    }

    // ================================================================
    // Two-wire line tests
    // ================================================================

    #[test]
    fn two_wire_lossless_z0_in_air() {
        // Standard formula: Z0 = (120/√εr) · acosh(d/2a)
        // For d = 10mm, a = 1mm, εr = 1:
        // acosh(5) = ln(5 + √24) ≈ 2.292
        // Z0 = 120 · 2.292 ≈ 275 Ω
        let line = TwoWireLine::lossless(1e-3, 10e-3, 1.0);
        let params = line.parameters(0.0);
        let z0 = params.z0_lossless();
        assert_relative_eq!(z0, 275.0, max_relative = 0.02);
    }

    #[test]
    fn two_wire_dielectric_reduces_z0() {
        let air = TwoWireLine::lossless(1e-3, 10e-3, 1.0);
        let dielectric = TwoWireLine::lossless(1e-3, 10e-3, 4.0);
        let z0_air = air.parameters(0.0).z0_lossless();
        let z0_diel = dielectric.parameters(0.0).z0_lossless();
        // Z0 ∝ 1/√εr, so ratio should be 2
        assert_relative_eq!(z0_air / z0_diel, 2.0, max_relative = 1e-6);
    }

    #[test]
    fn two_wire_phase_velocity_in_air_is_c() {
        let line = TwoWireLine::lossless(1e-3, 10e-3, 1.0);
        let params = line.parameters(0.0);
        assert_relative_eq!(
            params.phase_velocity_lossless(),
            constants::C_0,
            max_relative = 1e-3
        );
    }

    // ================================================================
    // Coaxial line tests
    // ================================================================

    #[test]
    fn coax_50_ohm_standard() {
        // 50Ω coax: Z0 = (60/√εr) · ln(b/a)
        // For air-filled (εr=1): Z0 = 60·ln(b/a) = 50 → b/a = e^(50/60) ≈ 2.302
        let ratio = (50.0 / 60.0_f64).exp();
        let line = CoaxialLine::lossless(1e-3, ratio * 1e-3, 1.0);
        let z0 = line.parameters(0.0).z0_lossless();
        assert_relative_eq!(z0, 50.0, max_relative = 0.01);
    }

    #[test]
    fn coax_75_ohm_with_pe_dielectric() {
        // Standard 75Ω coax with PE (εr ≈ 2.25)
        // Z0 = (60/√2.25) · ln(b/a) = 40·ln(b/a) = 75 → b/a = e^(75/40) ≈ 6.52
        let ratio = (75.0 / 40.0_f64).exp();
        let line = CoaxialLine::lossless(0.5e-3, ratio * 0.5e-3, 2.25);
        let z0 = line.parameters(0.0).z0_lossless();
        assert_relative_eq!(z0, 75.0, max_relative = 0.01);
    }

    #[test]
    fn coax_phase_velocity_in_dielectric() {
        let epsilon_r = 2.25;
        let line = CoaxialLine::lossless(1e-3, 3e-3, epsilon_r);
        let vp = line.parameters(0.0).phase_velocity_lossless();
        assert_relative_eq!(
            vp,
            constants::C_0 / epsilon_r.sqrt(),
            max_relative = 1e-3
        );
    }

    #[test]
    fn coax_lossless_has_zero_r_and_g() {
        let line = CoaxialLine::lossless(1e-3, 3e-3, 2.25);
        let p = line.parameters(1e9);
        assert_eq!(p.r_per_m, 0.0);
        assert_eq!(p.g_per_m, 0.0);
    }

    // ================================================================
    // Microstrip tests
    // ================================================================

    #[test]
    fn microstrip_effective_epsilon_between_1_and_er() {
        let ms = MicrostripLine::new(1e-3, 0.5e-3, 4.4); // FR4-like
        let eps_eff = ms.effective_epsilon_r();
        assert!(eps_eff > 1.0, "ε_eff must be > 1");
        assert!(eps_eff < 4.4, "ε_eff must be < ε_r");
    }

    #[test]
    fn microstrip_air_substrate_eps_eff_is_1() {
        let ms = MicrostripLine::new(1e-3, 1e-3, 1.0);
        let eps_eff = ms.effective_epsilon_r();
        assert_relative_eq!(eps_eff, 1.0, epsilon = 1e-10);
    }

    #[test]
    fn microstrip_50_ohm_on_fr4() {
        // Rough rule: for FR4 (εr≈4.4, h=1.6mm), w ≈ 3mm gives ~50Ω
        let ms = MicrostripLine::new(3.0e-3, 1.6e-3, 4.4);
        let z0 = ms.characteristic_impedance();
        assert_relative_eq!(z0, 50.0, max_relative = 0.15); // ~15% tolerance for simplified model
    }

    #[test]
    fn microstrip_wider_strip_lower_impedance() {
        let narrow = MicrostripLine::new(0.5e-3, 1e-3, 4.4);
        let wide = MicrostripLine::new(5e-3, 1e-3, 4.4);
        assert!(
            wide.characteristic_impedance() < narrow.characteristic_impedance(),
            "wider strip → lower Z0"
        );
    }

    #[test]
    fn microstrip_phase_velocity_slower_than_c() {
        let ms = MicrostripLine::new(1e-3, 1e-3, 4.4);
        assert!(ms.phase_velocity() < constants::C_0);
    }

    #[test]
    fn microstrip_parameters_consistent_with_z0_and_vp() {
        let ms = MicrostripLine::new(2e-3, 1e-3, 4.4);
        let p = ms.parameters();
        let z0_from_params = p.z0_lossless();
        let z0_direct = ms.characteristic_impedance();
        assert_relative_eq!(z0_from_params, z0_direct, max_relative = 1e-6);
    }
}

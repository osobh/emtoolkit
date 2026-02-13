//! Physical constants for electromagnetic computations.
//!
//! All values use SI units and are sourced from CODATA 2018 recommended values.
//! Constants are provided as `f64` for maximum precision in WASM environments.

use std::f64::consts::PI;

// ============================================================================
// Fundamental electromagnetic constants
// ============================================================================

/// Speed of light in vacuum (m/s)
pub const C_0: f64 = 299_792_458.0;

/// Permeability of free space (H/m)
pub const MU_0: f64 = 4.0e-7 * PI;

/// Permittivity of free space (F/m)
/// ε₀ = 1/(μ₀·c₀²)
pub const EPSILON_0: f64 = 8.854_187_812_8e-12;

/// Intrinsic impedance of free space (Ω)
/// η₀ = √(μ₀/ε₀) ≈ 376.73 Ω
pub const ETA_0: f64 = 376.730_313_668;

/// Elementary charge (C)
pub const ELEMENTARY_CHARGE: f64 = 1.602_176_634e-19;

/// Electron mass (kg)
pub const ELECTRON_MASS: f64 = 9.109_383_701_5e-31;

/// Boltzmann constant (J/K)
pub const BOLTZMANN: f64 = 1.380_649e-23;

/// Planck constant (J·s)
pub const PLANCK: f64 = 6.626_070_15e-34;

// ============================================================================
// Derived constants and convenience functions
// ============================================================================

/// Compute the wavelength in free space for a given frequency (Hz).
///
/// λ = c₀ / f
///
/// # Panics
/// Does not panic. Returns `f64::INFINITY` for zero frequency.
#[inline]
pub fn wavelength(frequency_hz: f64) -> f64 {
    C_0 / frequency_hz
}

/// Compute the angular frequency ω = 2πf for a given frequency (Hz).
#[inline]
pub fn angular_frequency(frequency_hz: f64) -> f64 {
    2.0 * PI * frequency_hz
}

/// Compute the free-space wavenumber k₀ = ω/c₀ = 2π/λ for a given frequency (Hz).
#[inline]
pub fn wavenumber(frequency_hz: f64) -> f64 {
    2.0 * PI * frequency_hz / C_0
}

/// Compute the skin depth δ = √(2/(ωμσ)) for a conductor.
///
/// # Arguments
/// * `frequency_hz` - Frequency in Hz
/// * `mu` - Permeability (H/m), use `MU_0` for non-magnetic conductors
/// * `sigma` - Conductivity (S/m)
///
/// # Returns
/// Skin depth in meters. Returns `f64::INFINITY` if frequency or conductivity is zero.
pub fn skin_depth(frequency_hz: f64, mu: f64, sigma: f64) -> f64 {
    let omega = angular_frequency(frequency_hz);
    let denominator = omega * mu * sigma;
    if denominator <= 0.0 {
        return f64::INFINITY;
    }
    (2.0 / denominator).sqrt()
}

/// Compute the intrinsic impedance η = √(μ/ε) for a lossless medium.
///
/// # Arguments
/// * `mu` - Permeability (H/m)
/// * `epsilon` - Permittivity (F/m)
///
/// # Returns
/// Intrinsic impedance in Ohms.
pub fn intrinsic_impedance(mu: f64, epsilon: f64) -> f64 {
    (mu / epsilon).sqrt()
}

/// Compute the phase velocity in a medium: v_p = 1/√(με)
pub fn phase_velocity(mu: f64, epsilon: f64) -> f64 {
    1.0 / (mu * epsilon).sqrt()
}

/// Compute permittivity from relative permittivity: ε = ε_r · ε₀
#[inline]
pub fn permittivity(epsilon_r: f64) -> f64 {
    epsilon_r * EPSILON_0
}

/// Compute permeability from relative permeability: μ = μ_r · μ₀
#[inline]
pub fn permeability(mu_r: f64) -> f64 {
    mu_r * MU_0
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use std::f64::consts::PI;

    // ================================================================
    // Fundamental constant value tests
    // ================================================================

    #[test]
    fn speed_of_light_is_exact_si_value() {
        assert_eq!(C_0, 299_792_458.0);
    }

    #[test]
    fn mu_0_is_4pi_times_1e_minus_7() {
        assert_relative_eq!(MU_0, 4.0e-7 * PI, epsilon = 1e-20);
    }

    #[test]
    fn epsilon_0_satisfies_relation_with_mu_0_and_c() {
        // ε₀ = 1/(μ₀·c₀²)
        let computed = 1.0 / (MU_0 * C_0 * C_0);
        assert_relative_eq!(EPSILON_0, computed, max_relative = 1e-6);
    }

    #[test]
    fn eta_0_equals_sqrt_mu_0_over_epsilon_0() {
        let computed = (MU_0 / EPSILON_0).sqrt();
        assert_relative_eq!(ETA_0, computed, max_relative = 1e-6);
    }

    #[test]
    fn eta_0_approximately_377_ohms() {
        assert_relative_eq!(ETA_0, 376.73, max_relative = 1e-4);
    }

    // ================================================================
    // Derived function tests
    // ================================================================

    #[test]
    fn wavelength_at_1ghz() {
        let lambda = wavelength(1.0e9);
        assert_relative_eq!(lambda, 0.2998, max_relative = 1e-3);
    }

    #[test]
    fn wavelength_at_300mhz_is_1_meter() {
        let lambda = wavelength(C_0); // f = c₀ → λ = 1m
        assert_relative_eq!(lambda, 1.0, epsilon = 1e-10);
    }

    #[test]
    fn wavelength_zero_frequency_is_infinity() {
        assert!(wavelength(0.0).is_infinite());
    }

    #[test]
    fn angular_frequency_at_1ghz() {
        let omega = angular_frequency(1.0e9);
        assert_relative_eq!(omega, 2.0 * PI * 1.0e9, epsilon = 1.0);
    }

    #[test]
    fn wavenumber_at_1ghz() {
        let k = wavenumber(1.0e9);
        let expected = 2.0 * PI * 1.0e9 / C_0;
        assert_relative_eq!(k, expected, epsilon = 1e-6);
    }

    #[test]
    fn wavenumber_relation_to_wavelength() {
        let f = 2.4e9;
        let k = wavenumber(f);
        let lambda = wavelength(f);
        assert_relative_eq!(k * lambda, 2.0 * PI, max_relative = 1e-10);
    }

    #[test]
    fn skin_depth_copper_at_1mhz() {
        // Copper: σ = 5.8e7 S/m, μ = μ₀
        // δ ≈ 66 μm at 1 MHz
        let delta = skin_depth(1.0e6, MU_0, 5.8e7);
        assert_relative_eq!(delta, 66.1e-6, max_relative = 0.02);
    }

    #[test]
    fn skin_depth_zero_frequency_is_infinity() {
        assert!(skin_depth(0.0, MU_0, 5.8e7).is_infinite());
    }

    #[test]
    fn skin_depth_zero_conductivity_is_infinity() {
        assert!(skin_depth(1.0e9, MU_0, 0.0).is_infinite());
    }

    #[test]
    fn intrinsic_impedance_free_space() {
        let eta = intrinsic_impedance(MU_0, EPSILON_0);
        assert_relative_eq!(eta, ETA_0, max_relative = 1e-6);
    }

    #[test]
    fn intrinsic_impedance_dielectric() {
        // For εr = 4 (glass-like), η = η₀/√εr ≈ 188.4 Ω
        let eta = intrinsic_impedance(MU_0, 4.0 * EPSILON_0);
        assert_relative_eq!(eta, ETA_0 / 2.0, max_relative = 1e-6);
    }

    #[test]
    fn phase_velocity_free_space_equals_c() {
        let v = phase_velocity(MU_0, EPSILON_0);
        assert_relative_eq!(v, C_0, max_relative = 1e-6);
    }

    #[test]
    fn phase_velocity_dielectric_slower_than_c() {
        // v = c/√εr, for εr=9 → v = c/3
        let v = phase_velocity(MU_0, 9.0 * EPSILON_0);
        assert_relative_eq!(v, C_0 / 3.0, max_relative = 1e-6);
    }

    #[test]
    fn permittivity_from_relative() {
        assert_relative_eq!(permittivity(1.0), EPSILON_0, epsilon = 1e-25);
        assert_relative_eq!(permittivity(4.0), 4.0 * EPSILON_0, epsilon = 1e-25);
    }

    #[test]
    fn permeability_from_relative() {
        assert_relative_eq!(permeability(1.0), MU_0, epsilon = 1e-20);
        assert_relative_eq!(permeability(100.0), 100.0 * MU_0, epsilon = 1e-18);
    }
}

//! Unit conversion utilities for electromagnetic quantities.
//!
//! Provides conversions between common EM units (dB, Np, degrees, radians, etc.)
//! used throughout the simulation modules.

use std::f64::consts::PI;

// ============================================================================
// Angle conversions
// ============================================================================

/// Convert degrees to radians.
#[inline]
pub fn deg_to_rad(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

/// Convert radians to degrees.
#[inline]
pub fn rad_to_deg(radians: f64) -> f64 {
    radians * 180.0 / PI
}

// ============================================================================
// Decibel conversions
// ============================================================================

/// Convert a power ratio to decibels: dB = 10·log₁₀(ratio).
///
/// # Arguments
/// * `ratio` - Power ratio (must be > 0)
///
/// # Returns
/// Value in dB. Returns `f64::NEG_INFINITY` for ratio = 0.
#[inline]
pub fn power_to_db(ratio: f64) -> f64 {
    10.0 * ratio.log10()
}

/// Convert decibels to a power ratio: ratio = 10^(dB/10).
#[inline]
pub fn db_to_power(db: f64) -> f64 {
    10.0_f64.powf(db / 10.0)
}

/// Convert a voltage/field amplitude ratio to decibels: dB = 20·log₁₀(ratio).
///
/// # Arguments
/// * `ratio` - Voltage or field amplitude ratio (must be > 0)
#[inline]
pub fn amplitude_to_db(ratio: f64) -> f64 {
    20.0 * ratio.log10()
}

/// Convert decibels to a voltage/field amplitude ratio: ratio = 10^(dB/20).
#[inline]
pub fn db_to_amplitude(db: f64) -> f64 {
    10.0_f64.powf(db / 20.0)
}

// ============================================================================
// Neper conversions
// ============================================================================

/// Convert nepers to decibels: dB = 8.686·Np.
#[inline]
pub fn neper_to_db(nepers: f64) -> f64 {
    nepers * 8.685_889_638_065_037
}

/// Convert decibels to nepers: Np = dB / 8.686.
#[inline]
pub fn db_to_neper(db: f64) -> f64 {
    db / 8.685_889_638_065_037
}

// ============================================================================
// Frequency conversions
// ============================================================================

/// Convert frequency in Hz to GHz.
#[inline]
pub fn hz_to_ghz(hz: f64) -> f64 {
    hz * 1e-9
}

/// Convert frequency in GHz to Hz.
#[inline]
pub fn ghz_to_hz(ghz: f64) -> f64 {
    ghz * 1e9
}

/// Convert frequency in Hz to MHz.
#[inline]
pub fn hz_to_mhz(hz: f64) -> f64 {
    hz * 1e-6
}

/// Convert frequency in MHz to Hz.
#[inline]
pub fn mhz_to_hz(mhz: f64) -> f64 {
    mhz * 1e6
}

// ============================================================================
// Length conversions
// ============================================================================

/// Convert meters to millimeters.
#[inline]
pub fn m_to_mm(meters: f64) -> f64 {
    meters * 1000.0
}

/// Convert millimeters to meters.
#[inline]
pub fn mm_to_m(mm: f64) -> f64 {
    mm * 0.001
}

/// Convert meters to micrometers.
#[inline]
pub fn m_to_um(meters: f64) -> f64 {
    meters * 1e6
}

/// Convert micrometers to meters.
#[inline]
pub fn um_to_m(um: f64) -> f64 {
    um * 1e-6
}

/// Express a length as a fraction of wavelength.
///
/// # Arguments
/// * `length` - Length in meters
/// * `wavelength` - Wavelength in meters
///
/// # Returns
/// Length expressed in wavelengths (e.g., 0.25 for quarter-wave).
#[inline]
pub fn length_in_wavelengths(length: f64, wavelength: f64) -> f64 {
    length / wavelength
}

/// Convert electrical length in wavelengths to physical length in meters.
#[inline]
pub fn wavelengths_to_meters(wavelengths: f64, wavelength: f64) -> f64 {
    wavelengths * wavelength
}

/// Convert electrical length in wavelengths to radians: βl = 2π · (l/λ).
#[inline]
pub fn wavelengths_to_radians(wavelengths: f64) -> f64 {
    2.0 * PI * wavelengths
}

/// Convert electrical length in radians to wavelengths: l/λ = βl / (2π).
#[inline]
pub fn radians_to_wavelengths(radians: f64) -> f64 {
    radians / (2.0 * PI)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use std::f64::consts::PI;

    // ================================================================
    // Angle conversion tests
    // ================================================================

    #[test]
    fn deg_to_rad_90_degrees() {
        assert_relative_eq!(deg_to_rad(90.0), PI / 2.0, epsilon = 1e-12);
    }

    #[test]
    fn deg_to_rad_360_degrees() {
        assert_relative_eq!(deg_to_rad(360.0), 2.0 * PI, epsilon = 1e-12);
    }

    #[test]
    fn rad_to_deg_pi() {
        assert_relative_eq!(rad_to_deg(PI), 180.0, epsilon = 1e-12);
    }

    #[test]
    fn angle_roundtrip() {
        assert_relative_eq!(rad_to_deg(deg_to_rad(45.0)), 45.0, epsilon = 1e-12);
    }

    // ================================================================
    // Decibel conversion tests
    // ================================================================

    #[test]
    fn power_to_db_unity_is_zero() {
        assert_relative_eq!(power_to_db(1.0), 0.0, epsilon = 1e-12);
    }

    #[test]
    fn power_to_db_10x_is_10db() {
        assert_relative_eq!(power_to_db(10.0), 10.0, epsilon = 1e-12);
    }

    #[test]
    fn power_to_db_100x_is_20db() {
        assert_relative_eq!(power_to_db(100.0), 20.0, epsilon = 1e-12);
    }

    #[test]
    fn power_to_db_half_is_minus_3db() {
        assert_relative_eq!(power_to_db(0.5), -3.0103, max_relative = 1e-3);
    }

    #[test]
    fn db_to_power_roundtrip() {
        let ratio = 42.7;
        assert_relative_eq!(db_to_power(power_to_db(ratio)), ratio, max_relative = 1e-12);
    }

    #[test]
    fn amplitude_to_db_2x_is_6db() {
        assert_relative_eq!(amplitude_to_db(2.0), 6.0206, max_relative = 1e-3);
    }

    #[test]
    fn amplitude_to_db_10x_is_20db() {
        assert_relative_eq!(amplitude_to_db(10.0), 20.0, epsilon = 1e-12);
    }

    #[test]
    fn db_to_amplitude_roundtrip() {
        let ratio = 3.5;
        assert_relative_eq!(
            db_to_amplitude(amplitude_to_db(ratio)),
            ratio,
            max_relative = 1e-12
        );
    }

    // ================================================================
    // Neper conversion tests
    // ================================================================

    #[test]
    fn neper_to_db_1_neper() {
        assert_relative_eq!(neper_to_db(1.0), 8.686, max_relative = 1e-3);
    }

    #[test]
    fn neper_db_roundtrip() {
        let np = 2.5;
        assert_relative_eq!(db_to_neper(neper_to_db(np)), np, max_relative = 1e-12);
    }

    // ================================================================
    // Frequency conversion tests
    // ================================================================

    #[test]
    fn hz_to_ghz_1ghz() {
        assert_relative_eq!(hz_to_ghz(1.0e9), 1.0, epsilon = 1e-12);
    }

    #[test]
    fn ghz_to_hz_2_4ghz() {
        assert_relative_eq!(ghz_to_hz(2.4), 2.4e9, epsilon = 1.0);
    }

    #[test]
    fn hz_ghz_roundtrip() {
        let f = 5.8e9;
        assert_relative_eq!(ghz_to_hz(hz_to_ghz(f)), f, max_relative = 1e-12);
    }

    #[test]
    fn hz_to_mhz_100mhz() {
        assert_relative_eq!(hz_to_mhz(100.0e6), 100.0, epsilon = 1e-12);
    }

    // ================================================================
    // Length conversion tests
    // ================================================================

    #[test]
    fn m_to_mm_1_meter() {
        assert_relative_eq!(m_to_mm(1.0), 1000.0, epsilon = 1e-12);
    }

    #[test]
    fn mm_to_m_roundtrip() {
        let m = 0.125;
        assert_relative_eq!(mm_to_m(m_to_mm(m)), m, max_relative = 1e-12);
    }

    #[test]
    fn m_to_um_1_meter() {
        assert_relative_eq!(m_to_um(1.0), 1e6, epsilon = 1e-6);
    }

    // ================================================================
    // Electrical length tests
    // ================================================================

    #[test]
    fn quarter_wave_is_0_25_wavelengths() {
        let lambda = 0.3; // 1 GHz in free space
        let length = 0.075; // λ/4
        assert_relative_eq!(
            length_in_wavelengths(length, lambda),
            0.25,
            epsilon = 1e-12
        );
    }

    #[test]
    fn wavelengths_to_meters_half_wave() {
        let lambda = 0.5;
        assert_relative_eq!(wavelengths_to_meters(0.5, lambda), 0.25, epsilon = 1e-12);
    }

    #[test]
    fn wavelengths_to_radians_quarter_wave_is_pi_over_2() {
        assert_relative_eq!(wavelengths_to_radians(0.25), PI / 2.0, epsilon = 1e-12);
    }

    #[test]
    fn radians_wavelengths_roundtrip() {
        let wl = 0.37;
        assert_relative_eq!(
            radians_to_wavelengths(wavelengths_to_radians(wl)),
            wl,
            max_relative = 1e-12
        );
    }
}

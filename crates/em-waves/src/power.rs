//! Power flow and Poynting vector calculations.

use std::f64::consts::PI;

/// Time-average Poynting vector magnitude for a plane wave.
/// S_avg = |E₀|² / (2η)
pub fn poynting_average_magnitude(e0: f64, eta: f64) -> f64 {
    e0 * e0 / (2.0 * eta)
}

/// Power density at distance r from isotropic radiator.
/// S = P_rad / (4π r²)
pub fn isotropic_power_density(p_rad: f64, r: f64) -> f64 {
    p_rad / (4.0 * PI * r * r)
}

/// Power intercepted by an aperture of effective area A_e at distance r.
/// P_r = S · A_e = P_t G_t A_e / (4π r²)
pub fn received_power(p_tx: f64, g_tx: f64, a_eff: f64, distance: f64) -> f64 {
    p_tx * g_tx * a_eff / (4.0 * PI * distance * distance)
}

/// Convert dBm to Watts.
pub fn dbm_to_watts(dbm: f64) -> f64 {
    10f64.powf((dbm - 30.0) / 10.0)
}

/// Convert Watts to dBm.
pub fn watts_to_dbm(watts: f64) -> f64 {
    10.0 * watts.log10() + 30.0
}

/// Convert dBW to Watts.
pub fn dbw_to_watts(dbw: f64) -> f64 {
    10f64.powf(dbw / 10.0)
}

/// Convert Watts to dBW.
pub fn watts_to_dbw(watts: f64) -> f64 {
    10.0 * watts.log10()
}

/// EIRP (Effective Isotropic Radiated Power).
pub fn eirp(p_tx: f64, g_tx: f64) -> f64 {
    p_tx * g_tx
}

/// Effective aperture from gain.
/// A_e = G λ² / (4π)
pub fn effective_aperture(gain: f64, wavelength: f64) -> f64 {
    gain * wavelength * wavelength / (4.0 * PI)
}

/// Radiation intensity U(θ) for normalized pattern.
/// U = S · r² (W/sr)
pub fn radiation_intensity(power_density: f64, distance: f64) -> f64 {
    power_density * distance * distance
}

/// Power delivered to load with impedance mismatch.
/// P_L = P_inc (1 - |Γ|²)
pub fn power_delivered(p_inc: f64, gamma_mag: f64) -> f64 {
    p_inc * (1.0 - gamma_mag * gamma_mag)
}

/// Return loss in dB.
/// RL = -20 log₁₀(|Γ|)
pub fn return_loss_db(gamma_mag: f64) -> f64 {
    -20.0 * gamma_mag.log10()
}

/// Mismatch loss in dB.
/// ML = -10 log₁₀(1 - |Γ|²)
pub fn mismatch_loss_db(gamma_mag: f64) -> f64 {
    -10.0 * (1.0 - gamma_mag * gamma_mag).log10()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_poynting_free_space() {
        // 1 V/m in free space (η₀ ≈ 377 Ω)
        let s = poynting_average_magnitude(1.0, 377.0);
        assert!((s - 1.0 / 754.0).abs() < 1e-6);
    }

    #[test]
    fn test_dbm_watts() {
        assert!((dbm_to_watts(0.0) - 0.001).abs() < 1e-10);
        assert!((dbm_to_watts(30.0) - 1.0).abs() < 1e-10);
        assert!((watts_to_dbm(1.0) - 30.0).abs() < 1e-10);
        assert!((watts_to_dbm(0.001) - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_dbw_watts() {
        assert!((dbw_to_watts(0.0) - 1.0).abs() < 1e-10);
        assert!((watts_to_dbw(1.0) - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_power_delivered() {
        // Perfect match
        assert!((power_delivered(1.0, 0.0) - 1.0).abs() < 1e-10);
        // Total reflection
        assert!((power_delivered(1.0, 1.0) - 0.0).abs() < 1e-10);
        // 50% reflection
        let p = power_delivered(1.0, 0.5);
        assert!((p - 0.75).abs() < 1e-10);
    }

    #[test]
    fn test_return_loss() {
        let rl = return_loss_db(0.1);
        assert!((rl - 20.0).abs() < 0.01);
    }

    #[test]
    fn test_mismatch_loss() {
        let ml = mismatch_loss_db(0.0);
        assert!((ml - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_eirp() {
        assert!((eirp(10.0, 100.0) - 1000.0).abs() < 1e-10);
    }

    #[test]
    fn test_effective_aperture() {
        let ae = effective_aperture(1.0, 0.03); // isotropic at 10 GHz
        assert!(ae > 0.0);
        let expected = 0.03 * 0.03 / (4.0 * PI);
        assert!((ae - expected).abs() < 1e-10);
    }
}

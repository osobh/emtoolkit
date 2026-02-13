//! Friis transmission equation and link budget analysis.
//!
//! P_r/P_t = G_t · G_r · (λ/(4πR))²

use em_core::constants::C_0;
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

/// Friis transmission link parameters.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct FriisLink {
    /// Transmit power (W)
    pub p_tx: f64,
    /// Transmit antenna gain (linear, not dB)
    pub g_tx: f64,
    /// Receive antenna gain (linear)
    pub g_rx: f64,
    /// Frequency (Hz)
    pub frequency: f64,
    /// Distance between antennas (m)
    pub distance: f64,
}

impl FriisLink {
    pub fn new(p_tx: f64, g_tx: f64, g_rx: f64, frequency: f64, distance: f64) -> Self {
        Self {
            p_tx,
            g_tx,
            g_rx,
            frequency,
            distance,
        }
    }

    /// Wavelength λ = c/f.
    pub fn wavelength(&self) -> f64 {
        C_0 / self.frequency
    }

    /// Free-space path loss (linear): L = (4πR/λ)²
    pub fn path_loss(&self) -> f64 {
        let x = 4.0 * PI * self.distance / self.wavelength();
        x * x
    }

    /// Free-space path loss in dB.
    pub fn path_loss_db(&self) -> f64 {
        10.0 * self.path_loss().log10()
    }

    /// Received power (W).
    ///
    /// P_r = P_t · G_t · G_r / L
    pub fn received_power(&self) -> f64 {
        self.p_tx * self.g_tx * self.g_rx / self.path_loss()
    }

    /// Received power in dBW.
    pub fn received_power_dbw(&self) -> f64 {
        10.0 * self.received_power().log10()
    }

    /// Received power in dBm.
    pub fn received_power_dbm(&self) -> f64 {
        10.0 * (self.received_power() * 1000.0).log10()
    }

    /// EIRP (Effective Isotropic Radiated Power) in watts.
    pub fn eirp(&self) -> f64 {
        self.p_tx * self.g_tx
    }

    /// EIRP in dBW.
    pub fn eirp_dbw(&self) -> f64 {
        10.0 * self.eirp().log10()
    }

    /// Power density at distance R (W/m²).
    pub fn power_density(&self) -> f64 {
        self.eirp() / (4.0 * PI * self.distance * self.distance)
    }

    /// Sample received power vs distance for range analysis.
    pub fn sample_vs_distance(
        &self,
        r_min: f64,
        r_max: f64,
        num_points: usize,
    ) -> (Vec<f64>, Vec<f64>) {
        assert!(num_points >= 2);
        let dr = (r_max - r_min) / (num_points - 1) as f64;
        let distances: Vec<f64> = (0..num_points).map(|i| r_min + i as f64 * dr).collect();
        let powers: Vec<f64> = distances.iter().map(|&r| {
            let link = FriisLink::new(self.p_tx, self.g_tx, self.g_rx, self.frequency, r);
            link.received_power_dbm()
        }).collect();
        (distances, powers)
    }
}

/// Convert linear gain to dB.
pub fn to_db(linear: f64) -> f64 {
    10.0 * linear.log10()
}

/// Convert dB to linear gain.
pub fn from_db(db: f64) -> f64 {
    10.0_f64.powf(db / 10.0)
}

/// Convert dBm to watts.
pub fn dbm_to_watts(dbm: f64) -> f64 {
    10.0_f64.powf((dbm - 30.0) / 10.0)
}

/// Convert watts to dBm.
pub fn watts_to_dbm(watts: f64) -> f64 {
    10.0 * watts.log10() + 30.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn friis_isotropic_antennas() {
        // P_r = P_t · (λ/(4πR))²
        let link = FriisLink::new(1.0, 1.0, 1.0, 1e9, 1000.0);
        let lambda = C_0 / 1e9;
        let expected = (lambda / (4.0 * PI * 1000.0)).powi(2);
        assert_relative_eq!(link.received_power(), expected, max_relative = 1e-10);
    }

    #[test]
    fn friis_power_inverse_square() {
        let link1 = FriisLink::new(1.0, 1.0, 1.0, 1e9, 100.0);
        let link2 = FriisLink::new(1.0, 1.0, 1.0, 1e9, 200.0);
        assert_relative_eq!(
            link1.received_power() / link2.received_power(),
            4.0,
            max_relative = 1e-10
        );
    }

    #[test]
    fn friis_gain_multiplies() {
        let link1 = FriisLink::new(1.0, 1.0, 1.0, 1e9, 1000.0);
        let link2 = FriisLink::new(1.0, 10.0, 10.0, 1e9, 1000.0);
        assert_relative_eq!(
            link2.received_power() / link1.received_power(),
            100.0,
            max_relative = 1e-10
        );
    }

    #[test]
    fn path_loss_increases_with_distance() {
        let link1 = FriisLink::new(1.0, 1.0, 1.0, 1e9, 100.0);
        let link2 = FriisLink::new(1.0, 1.0, 1.0, 1e9, 1000.0);
        assert!(link2.path_loss_db() > link1.path_loss_db());
    }

    #[test]
    fn path_loss_increases_with_frequency() {
        let link1 = FriisLink::new(1.0, 1.0, 1.0, 1e9, 1000.0);
        let link2 = FriisLink::new(1.0, 1.0, 1.0, 10e9, 1000.0);
        assert!(link2.path_loss_db() > link1.path_loss_db());
    }

    #[test]
    fn eirp_value() {
        let link = FriisLink::new(10.0, 20.0, 1.0, 1e9, 1000.0);
        assert_relative_eq!(link.eirp(), 200.0, epsilon = 1e-10);
    }

    #[test]
    fn dbm_watts_roundtrip() {
        let w = 0.001; // 1 mW = 0 dBm
        assert_relative_eq!(watts_to_dbm(w), 0.0, epsilon = 1e-10);
        assert_relative_eq!(dbm_to_watts(0.0), 0.001, max_relative = 1e-10);
    }

    #[test]
    fn db_conversions() {
        assert_relative_eq!(to_db(100.0), 20.0, epsilon = 1e-10);
        assert_relative_eq!(from_db(20.0), 100.0, max_relative = 1e-10);
    }

    #[test]
    fn sample_vs_distance_dimensions() {
        let link = FriisLink::new(1.0, 10.0, 10.0, 1e9, 1000.0);
        let (ds, ps) = link.sample_vs_distance(100.0, 10000.0, 50);
        assert_eq!(ds.len(), 50);
        assert_eq!(ps.len(), 50);
    }

    #[test]
    fn sample_vs_distance_decreasing() {
        let link = FriisLink::new(1.0, 10.0, 10.0, 1e9, 1000.0);
        let (_, ps) = link.sample_vs_distance(100.0, 10000.0, 50);
        for i in 1..ps.len() {
            assert!(ps[i] <= ps[i - 1], "power should decrease with distance");
        }
    }
}

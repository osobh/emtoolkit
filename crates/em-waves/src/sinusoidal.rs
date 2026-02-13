//! Module 1.1: Sinusoidal Waveforms
//!
//! Generates time-domain sinusoidal signals with configurable amplitude,
//! frequency, phase, and optional exponential damping.
//!
//! y(t) = A · e^(-αt) · cos(2πft + φ)

use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

/// Parameters for a sinusoidal waveform.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SinusoidalParams {
    /// Peak amplitude
    pub amplitude: f64,
    /// Frequency in Hz
    pub frequency: f64,
    /// Phase offset in radians
    pub phase_rad: f64,
    /// Exponential damping factor (Np/s). Zero for undamped.
    pub damping: f64,
}

impl SinusoidalParams {
    /// Create a simple undamped sinusoid.
    pub fn new(amplitude: f64, frequency: f64, phase_rad: f64) -> Self {
        Self {
            amplitude,
            frequency,
            phase_rad,
            damping: 0.0,
        }
    }

    /// Create a damped sinusoid.
    pub fn damped(amplitude: f64, frequency: f64, phase_rad: f64, damping: f64) -> Self {
        Self {
            amplitude,
            frequency,
            phase_rad,
            damping,
        }
    }

    /// Evaluate the waveform at time t (seconds).
    ///
    /// y(t) = A · e^(-α·t) · cos(2πf·t + φ)
    pub fn evaluate(&self, t: f64) -> f64 {
        let envelope = if self.damping == 0.0 {
            self.amplitude
        } else {
            self.amplitude * (-self.damping * t).exp()
        };
        envelope * (2.0 * PI * self.frequency * t + self.phase_rad).cos()
    }

    /// Generate a waveform sampled at uniform time steps.
    ///
    /// # Arguments
    /// * `t_start` - Start time (s)
    /// * `t_end` - End time (s)
    /// * `num_samples` - Number of sample points (must be ≥ 2)
    ///
    /// # Returns
    /// Tuple of (time_values, waveform_values) as `Vec<f64>`.
    pub fn sample(&self, t_start: f64, t_end: f64, num_samples: usize) -> (Vec<f64>, Vec<f64>) {
        assert!(num_samples >= 2, "need at least 2 samples");
        let dt = (t_end - t_start) / (num_samples - 1) as f64;
        let times: Vec<f64> = (0..num_samples).map(|i| t_start + i as f64 * dt).collect();
        let values: Vec<f64> = times.iter().map(|&t| self.evaluate(t)).collect();
        (times, values)
    }

    /// Period of the waveform (s).
    pub fn period(&self) -> f64 {
        1.0 / self.frequency
    }

    /// Angular frequency ω = 2πf (rad/s).
    pub fn omega(&self) -> f64 {
        2.0 * PI * self.frequency
    }

    /// Wavelength in a medium with given phase velocity (m).
    pub fn wavelength(&self, phase_velocity: f64) -> f64 {
        phase_velocity / self.frequency
    }
}

/// Superpose multiple sinusoidal waveforms by summing their values at each time step.
///
/// # Arguments
/// * `waveforms` - Slice of sinusoidal parameter sets
/// * `t_start` - Start time (s)
/// * `t_end` - End time (s)
/// * `num_samples` - Number of sample points
///
/// # Returns
/// Tuple of (time_values, superposed_values).
pub fn superpose(
    waveforms: &[SinusoidalParams],
    t_start: f64,
    t_end: f64,
    num_samples: usize,
) -> (Vec<f64>, Vec<f64>) {
    assert!(num_samples >= 2, "need at least 2 samples");
    assert!(!waveforms.is_empty(), "need at least 1 waveform");

    let dt = (t_end - t_start) / (num_samples - 1) as f64;
    let times: Vec<f64> = (0..num_samples).map(|i| t_start + i as f64 * dt).collect();
    let values: Vec<f64> = times
        .iter()
        .map(|&t| waveforms.iter().map(|w| w.evaluate(t)).sum())
        .collect();
    (times, values)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use std::f64::consts::PI;

    #[test]
    fn evaluate_at_t_zero_with_zero_phase() {
        let s = SinusoidalParams::new(5.0, 1.0, 0.0);
        // y(0) = 5·cos(0) = 5
        assert_relative_eq!(s.evaluate(0.0), 5.0, epsilon = 1e-12);
    }

    #[test]
    fn evaluate_at_quarter_period() {
        let s = SinusoidalParams::new(3.0, 1.0, 0.0);
        // y(T/4) = 3·cos(π/2) = 0
        assert_relative_eq!(s.evaluate(0.25), 0.0, epsilon = 1e-12);
    }

    #[test]
    fn evaluate_at_half_period() {
        let s = SinusoidalParams::new(2.0, 1.0, 0.0);
        // y(T/2) = 2·cos(π) = -2
        assert_relative_eq!(s.evaluate(0.5), -2.0, epsilon = 1e-12);
    }

    #[test]
    fn evaluate_with_phase_offset() {
        let s = SinusoidalParams::new(1.0, 1.0, PI / 2.0);
        // y(0) = cos(π/2) = 0
        assert_relative_eq!(s.evaluate(0.0), 0.0, epsilon = 1e-12);
    }

    #[test]
    fn evaluate_damped_decays() {
        let s = SinusoidalParams::damped(10.0, 1.0, 0.0, 1.0);
        let y0 = s.evaluate(0.0);
        let y1 = s.evaluate(1.0);
        // At t=0: y = 10·cos(0) = 10
        assert_relative_eq!(y0, 10.0, epsilon = 1e-12);
        // At t=1: y = 10·e^(-1)·cos(2π) = 10/e ≈ 3.679
        assert_relative_eq!(y1, 10.0 * (-1.0_f64).exp(), epsilon = 1e-10);
    }

    #[test]
    fn damping_zero_is_undamped() {
        let undamped = SinusoidalParams::new(5.0, 100.0, 0.3);
        let damped_zero = SinusoidalParams::damped(5.0, 100.0, 0.3, 0.0);
        for t in [0.0, 0.001, 0.01, 0.1] {
            assert_relative_eq!(
                undamped.evaluate(t),
                damped_zero.evaluate(t),
                epsilon = 1e-12
            );
        }
    }

    #[test]
    fn period_is_inverse_frequency() {
        let s = SinusoidalParams::new(1.0, 60.0, 0.0);
        assert_relative_eq!(s.period(), 1.0 / 60.0, epsilon = 1e-15);
    }

    #[test]
    fn omega_is_2pi_f() {
        let s = SinusoidalParams::new(1.0, 1e9, 0.0);
        assert_relative_eq!(s.omega(), 2.0 * PI * 1e9, epsilon = 1.0);
    }

    #[test]
    fn wavelength_in_free_space() {
        let s = SinusoidalParams::new(1.0, 1e9, 0.0);
        let lambda = s.wavelength(em_core::constants::C_0);
        assert_relative_eq!(lambda, 0.2998, max_relative = 1e-3);
    }

    #[test]
    fn sample_returns_correct_length() {
        let s = SinusoidalParams::new(1.0, 1.0, 0.0);
        let (t, y) = s.sample(0.0, 1.0, 100);
        assert_eq!(t.len(), 100);
        assert_eq!(y.len(), 100);
    }

    #[test]
    fn sample_endpoints_correct() {
        let s = SinusoidalParams::new(1.0, 1.0, 0.0);
        let (t, _y) = s.sample(0.0, 2.0, 201);
        assert_relative_eq!(t[0], 0.0, epsilon = 1e-12);
        assert_relative_eq!(t[200], 2.0, epsilon = 1e-12);
    }

    #[test]
    fn sample_values_match_evaluate() {
        let s = SinusoidalParams::new(3.0, 5.0, 0.7);
        let (t, y) = s.sample(0.0, 1.0, 50);
        for (ti, yi) in t.iter().zip(y.iter()) {
            assert_relative_eq!(*yi, s.evaluate(*ti), epsilon = 1e-12);
        }
    }

    #[test]
    fn superpose_single_waveform_matches_original() {
        let s = SinusoidalParams::new(2.0, 3.0, 0.5);
        let (t1, y1) = s.sample(0.0, 1.0, 100);
        let (t2, y2) = superpose(&[s], 0.0, 1.0, 100);
        assert_eq!(t1, t2);
        for (a, b) in y1.iter().zip(y2.iter()) {
            assert_relative_eq!(a, b, epsilon = 1e-12);
        }
    }

    #[test]
    fn superpose_two_equal_waveforms_doubles_amplitude() {
        let s = SinusoidalParams::new(1.0, 1.0, 0.0);
        let (_t, y) = superpose(&[s, s], 0.0, 1.0, 100);
        let (_t, y_single) = s.sample(0.0, 1.0, 100);
        for (ys, y2) in y_single.iter().zip(y.iter()) {
            assert_relative_eq!(y2, &(2.0 * ys), epsilon = 1e-12);
        }
    }

    #[test]
    fn superpose_opposite_phases_cancel() {
        let s1 = SinusoidalParams::new(1.0, 1.0, 0.0);
        let s2 = SinusoidalParams::new(1.0, 1.0, PI);
        let (_t, y) = superpose(&[s1, s2], 0.0, 1.0, 100);
        for val in &y {
            assert_relative_eq!(*val, 0.0, epsilon = 1e-12);
        }
    }
}

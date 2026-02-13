//! Module 1.3: Phase Lead/Lag
//!
//! Compares two sinusoidal waveforms, computing their phase difference,
//! determining lead/lag relationship, and providing phasor diagram data.

use em_core::complex::Phasor;
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

/// The phase relationship between two waveforms.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PhaseRelation {
    /// First waveform leads the second (reaches peak earlier in time).
    Leading,
    /// First waveform lags the second.
    Lagging,
    /// Waveforms are in phase.
    InPhase,
    /// Waveforms are exactly 180° out of phase.
    AntiPhase,
}

/// Parameters for one waveform in the phase comparison.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct WaveformParams {
    pub amplitude: f64,
    pub frequency: f64,
    pub phase_rad: f64,
}

impl WaveformParams {
    pub fn new(amplitude: f64, frequency: f64, phase_rad: f64) -> Self {
        Self {
            amplitude,
            frequency,
            phase_rad,
        }
    }

    /// Evaluate at time t.
    pub fn evaluate(&self, t: f64) -> f64 {
        self.amplitude * (2.0 * PI * self.frequency * t + self.phase_rad).cos()
    }

    /// Convert to a phasor representation.
    pub fn to_phasor(&self) -> Phasor {
        Phasor::new(self.amplitude, self.phase_rad)
    }

    /// Sample the waveform over a time range.
    pub fn sample(&self, t_start: f64, t_end: f64, num_samples: usize) -> (Vec<f64>, Vec<f64>) {
        assert!(num_samples >= 2);
        let dt = (t_end - t_start) / (num_samples - 1) as f64;
        let ts: Vec<f64> = (0..num_samples).map(|i| t_start + i as f64 * dt).collect();
        let ys: Vec<f64> = ts.iter().map(|&t| self.evaluate(t)).collect();
        (ts, ys)
    }
}

/// Result of a phase comparison between two waveforms.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PhaseComparison {
    /// Phase difference φ₁ - φ₂ in radians, normalized to (-π, π].
    pub phase_difference_rad: f64,
    /// Phase difference in degrees.
    pub phase_difference_deg: f64,
    /// The relationship: leading, lagging, in-phase, or anti-phase.
    pub relation: PhaseRelation,
    /// Phasor of waveform 1.
    pub phasor_1: Phasor,
    /// Phasor of waveform 2.
    pub phasor_2: Phasor,
    /// Time delay Δt = Δφ / ω (positive means waveform 1 peaks first).
    /// Only meaningful when both waveforms have the same frequency.
    pub time_delay: f64,
}

/// Compare two waveforms and compute their phase relationship.
///
/// Phase difference is defined as φ₁ - φ₂, normalized to (-π, π].
/// - Positive → waveform 1 leads (peaks earlier)
/// - Negative → waveform 1 lags (peaks later)
///
/// # Arguments
/// * `w1` - First waveform
/// * `w2` - Second waveform
///
/// # Note
/// Time delay is computed using the frequency of waveform 1. For meaningful
/// time delay comparison, both waveforms should have the same frequency.
pub fn compare(w1: &WaveformParams, w2: &WaveformParams) -> PhaseComparison {
    let raw_diff = w1.phase_rad - w2.phase_rad;
    let normalized = em_core::complex::normalize_angle(raw_diff);
    let deg = normalized.to_degrees();

    let relation = if normalized.abs() < 1e-12 {
        PhaseRelation::InPhase
    } else if (normalized.abs() - PI).abs() < 1e-12 {
        PhaseRelation::AntiPhase
    } else if normalized > 0.0 {
        PhaseRelation::Leading
    } else {
        PhaseRelation::Lagging
    };

    let omega = 2.0 * PI * w1.frequency;
    let time_delay = if omega > 0.0 {
        normalized / omega
    } else {
        0.0
    };

    PhaseComparison {
        phase_difference_rad: normalized,
        phase_difference_deg: deg,
        relation,
        phasor_1: w1.to_phasor(),
        phasor_2: w2.to_phasor(),
        time_delay,
    }
}

/// Compute the phasor sum of two waveforms (only valid if same frequency).
///
/// Returns the resultant phasor and its waveform parameters.
pub fn phasor_sum(w1: &WaveformParams, w2: &WaveformParams) -> WaveformParams {
    let z1 = w1.to_phasor().to_complex();
    let z2 = w2.to_phasor().to_complex();
    let sum = z1 + z2;
    let p = Phasor::from_complex(sum);
    WaveformParams {
        amplitude: p.magnitude,
        frequency: w1.frequency, // assume same frequency
        phase_rad: p.phase_rad,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use std::f64::consts::PI;

    #[test]
    fn in_phase_waveforms() {
        let w1 = WaveformParams::new(1.0, 1.0, 0.0);
        let w2 = WaveformParams::new(2.0, 1.0, 0.0);
        let cmp = compare(&w1, &w2);
        assert_eq!(cmp.relation, PhaseRelation::InPhase);
        assert_relative_eq!(cmp.phase_difference_rad, 0.0, epsilon = 1e-12);
    }

    #[test]
    fn anti_phase_waveforms() {
        let w1 = WaveformParams::new(1.0, 1.0, 0.0);
        let w2 = WaveformParams::new(1.0, 1.0, PI);
        let cmp = compare(&w1, &w2);
        assert_eq!(cmp.relation, PhaseRelation::AntiPhase);
    }

    #[test]
    fn leading_waveform() {
        let w1 = WaveformParams::new(1.0, 1.0, PI / 4.0);
        let w2 = WaveformParams::new(1.0, 1.0, 0.0);
        let cmp = compare(&w1, &w2);
        assert_eq!(cmp.relation, PhaseRelation::Leading);
        assert_relative_eq!(cmp.phase_difference_deg, 45.0, epsilon = 1e-10);
    }

    #[test]
    fn lagging_waveform() {
        let w1 = WaveformParams::new(1.0, 1.0, 0.0);
        let w2 = WaveformParams::new(1.0, 1.0, PI / 3.0);
        let cmp = compare(&w1, &w2);
        assert_eq!(cmp.relation, PhaseRelation::Lagging);
        assert_relative_eq!(cmp.phase_difference_deg, -60.0, epsilon = 1e-10);
    }

    #[test]
    fn time_delay_positive_for_leading() {
        let f = 1000.0; // 1 kHz
        let w1 = WaveformParams::new(1.0, f, PI / 2.0);
        let w2 = WaveformParams::new(1.0, f, 0.0);
        let cmp = compare(&w1, &w2);
        // Δt = (π/2) / (2π·1000) = 0.25ms
        assert!(cmp.time_delay > 0.0);
        assert_relative_eq!(cmp.time_delay, 0.25e-3, max_relative = 1e-10);
    }

    #[test]
    fn phase_wraps_correctly() {
        // φ₁ = 350°, φ₂ = 10° → diff should be -20° (not 340°)
        let w1 = WaveformParams::new(1.0, 1.0, 350.0_f64.to_radians());
        let w2 = WaveformParams::new(1.0, 1.0, 10.0_f64.to_radians());
        let cmp = compare(&w1, &w2);
        assert_relative_eq!(cmp.phase_difference_deg, -20.0, epsilon = 1e-8);
        assert_eq!(cmp.relation, PhaseRelation::Lagging);
    }

    #[test]
    fn phasor_data_in_comparison() {
        let w1 = WaveformParams::new(3.0, 1.0, PI / 6.0);
        let w2 = WaveformParams::new(4.0, 1.0, PI / 3.0);
        let cmp = compare(&w1, &w2);
        assert_relative_eq!(cmp.phasor_1.magnitude, 3.0, epsilon = 1e-12);
        assert_relative_eq!(cmp.phasor_2.magnitude, 4.0, epsilon = 1e-12);
        assert_relative_eq!(cmp.phasor_1.phase_rad, PI / 6.0, epsilon = 1e-12);
        assert_relative_eq!(cmp.phasor_2.phase_rad, PI / 3.0, epsilon = 1e-12);
    }

    #[test]
    fn waveform_evaluate_matches_cos() {
        let w = WaveformParams::new(5.0, 100.0, 0.3);
        let t = 0.002;
        let expected = 5.0 * (2.0 * PI * 100.0 * t + 0.3).cos();
        assert_relative_eq!(w.evaluate(t), expected, epsilon = 1e-12);
    }

    #[test]
    fn waveform_sample_consistent() {
        let w = WaveformParams::new(2.0, 50.0, 0.0);
        let (ts, ys) = w.sample(0.0, 0.02, 100);
        for (ti, yi) in ts.iter().zip(ys.iter()) {
            assert_relative_eq!(*yi, w.evaluate(*ti), epsilon = 1e-12);
        }
    }

    #[test]
    fn phasor_sum_same_phase_adds_amplitudes() {
        let w1 = WaveformParams::new(3.0, 1.0, 0.0);
        let w2 = WaveformParams::new(4.0, 1.0, 0.0);
        let sum = phasor_sum(&w1, &w2);
        assert_relative_eq!(sum.amplitude, 7.0, epsilon = 1e-12);
        assert_relative_eq!(sum.phase_rad, 0.0, epsilon = 1e-12);
    }

    #[test]
    fn phasor_sum_opposite_phase_cancels() {
        let w1 = WaveformParams::new(5.0, 1.0, 0.0);
        let w2 = WaveformParams::new(5.0, 1.0, PI);
        let sum = phasor_sum(&w1, &w2);
        assert_relative_eq!(sum.amplitude, 0.0, epsilon = 1e-10);
    }

    #[test]
    fn phasor_sum_orthogonal() {
        let w1 = WaveformParams::new(3.0, 1.0, 0.0);
        let w2 = WaveformParams::new(4.0, 1.0, PI / 2.0);
        let sum = phasor_sum(&w1, &w2);
        assert_relative_eq!(sum.amplitude, 5.0, epsilon = 1e-10);
    }

    #[test]
    fn phasor_sum_waveform_matches_superposition() {
        // The phasor sum evaluated at any time should equal w1(t) + w2(t)
        let w1 = WaveformParams::new(2.0, 100.0, 0.3);
        let w2 = WaveformParams::new(3.0, 100.0, 1.2);
        let sum = phasor_sum(&w1, &w2);

        for i in 0..50 {
            let t = i as f64 * 1e-4;
            let direct = w1.evaluate(t) + w2.evaluate(t);
            let via_sum = sum.evaluate(t);
            assert_relative_eq!(direct, via_sum, epsilon = 1e-10);
        }
    }
}

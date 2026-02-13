//! Module 1.2: Traveling Waves
//!
//! Simulates plane wave propagation in the ±x direction with configurable
//! medium properties, frequency, and optional loss.
//!
//! y(x, t) = A · e^(-αx) · cos(ωt - βx + φ)  [+x direction]
//! y(x, t) = A · e^(+αx) · cos(ωt + βx + φ)  [-x direction]

use em_core::constants;
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

/// Direction of wave propagation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Direction {
    PositiveX,
    NegativeX,
}

/// Parameters for a traveling wave.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct TravelingWaveParams {
    /// Peak amplitude
    pub amplitude: f64,
    /// Frequency in Hz
    pub frequency: f64,
    /// Phase offset in radians
    pub phase_rad: f64,
    /// Propagation direction
    pub direction: Direction,
    /// Phase constant β (rad/m). Computed from medium if not set directly.
    pub beta: f64,
    /// Attenuation constant α (Np/m). Zero for lossless.
    pub alpha: f64,
}

impl TravelingWaveParams {
    /// Create a lossless traveling wave in free space.
    pub fn in_free_space(amplitude: f64, frequency: f64, phase_rad: f64, direction: Direction) -> Self {
        let beta = 2.0 * PI * frequency / constants::C_0;
        Self {
            amplitude,
            frequency,
            phase_rad,
            direction,
            beta,
            alpha: 0.0,
        }
    }

    /// Create a traveling wave in a lossless dielectric with relative permittivity ε_r.
    pub fn in_dielectric(
        amplitude: f64,
        frequency: f64,
        phase_rad: f64,
        direction: Direction,
        epsilon_r: f64,
    ) -> Self {
        let v_p = constants::C_0 / epsilon_r.sqrt();
        let beta = 2.0 * PI * frequency / v_p;
        Self {
            amplitude,
            frequency,
            phase_rad,
            direction,
            beta,
            alpha: 0.0,
        }
    }

    /// Create a traveling wave with explicit propagation parameters.
    pub fn with_propagation(
        amplitude: f64,
        frequency: f64,
        phase_rad: f64,
        direction: Direction,
        alpha: f64,
        beta: f64,
    ) -> Self {
        Self {
            amplitude,
            frequency,
            phase_rad,
            direction,
            alpha,
            beta,
        }
    }

    /// Evaluate the wave at position x (meters) and time t (seconds).
    pub fn evaluate(&self, x: f64, t: f64) -> f64 {
        let omega = 2.0 * PI * self.frequency;
        match self.direction {
            Direction::PositiveX => {
                let envelope = self.amplitude * (-self.alpha * x).exp();
                envelope * (omega * t - self.beta * x + self.phase_rad).cos()
            }
            Direction::NegativeX => {
                let envelope = self.amplitude * (self.alpha * x).exp();
                envelope * (omega * t + self.beta * x + self.phase_rad).cos()
            }
        }
    }

    /// Sample the wave in space at a fixed time.
    ///
    /// # Returns
    /// (x_values, wave_values)
    pub fn sample_space(
        &self,
        x_start: f64,
        x_end: f64,
        num_points: usize,
        t: f64,
    ) -> (Vec<f64>, Vec<f64>) {
        assert!(num_points >= 2);
        let dx = (x_end - x_start) / (num_points - 1) as f64;
        let xs: Vec<f64> = (0..num_points).map(|i| x_start + i as f64 * dx).collect();
        let ys: Vec<f64> = xs.iter().map(|&x| self.evaluate(x, t)).collect();
        (xs, ys)
    }

    /// Sample the wave in time at a fixed position.
    ///
    /// # Returns
    /// (t_values, wave_values)
    pub fn sample_time(
        &self,
        t_start: f64,
        t_end: f64,
        num_points: usize,
        x: f64,
    ) -> (Vec<f64>, Vec<f64>) {
        assert!(num_points >= 2);
        let dt = (t_end - t_start) / (num_points - 1) as f64;
        let ts: Vec<f64> = (0..num_points).map(|i| t_start + i as f64 * dt).collect();
        let ys: Vec<f64> = ts.iter().map(|&t| self.evaluate(x, t)).collect();
        (ts, ys)
    }

    /// Wavelength λ = 2π/β (m).
    pub fn wavelength(&self) -> f64 {
        2.0 * PI / self.beta
    }

    /// Phase velocity v_p = ω/β (m/s).
    pub fn phase_velocity(&self) -> f64 {
        2.0 * PI * self.frequency / self.beta
    }

    /// Skin depth δ = 1/α (m). Returns infinity for lossless.
    pub fn skin_depth(&self) -> f64 {
        if self.alpha == 0.0 {
            f64::INFINITY
        } else {
            1.0 / self.alpha
        }
    }
}

/// Superpose multiple traveling waves at given (x, t) points.
///
/// Evaluates each wave at each spatial point at time t and sums them.
///
/// # Returns
/// (x_values, superposed_values)
pub fn superpose_spatial(
    waves: &[TravelingWaveParams],
    x_start: f64,
    x_end: f64,
    num_points: usize,
    t: f64,
) -> (Vec<f64>, Vec<f64>) {
    assert!(num_points >= 2);
    assert!(!waves.is_empty());

    let dx = (x_end - x_start) / (num_points - 1) as f64;
    let xs: Vec<f64> = (0..num_points).map(|i| x_start + i as f64 * dx).collect();
    let ys: Vec<f64> = xs
        .iter()
        .map(|&x| waves.iter().map(|w| w.evaluate(x, t)).sum())
        .collect();
    (xs, ys)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use std::f64::consts::PI;

    #[test]
    fn free_space_wave_at_origin_and_t0() {
        let w = TravelingWaveParams::in_free_space(5.0, 1e9, 0.0, Direction::PositiveX);
        // y(0, 0) = 5·cos(0) = 5
        assert_relative_eq!(w.evaluate(0.0, 0.0), 5.0, epsilon = 1e-12);
    }

    #[test]
    fn free_space_wavelength_at_1ghz() {
        let w = TravelingWaveParams::in_free_space(1.0, 1e9, 0.0, Direction::PositiveX);
        assert_relative_eq!(w.wavelength(), constants::C_0 / 1e9, max_relative = 1e-10);
    }

    #[test]
    fn free_space_phase_velocity_is_c() {
        let w = TravelingWaveParams::in_free_space(1.0, 1e9, 0.0, Direction::PositiveX);
        assert_relative_eq!(w.phase_velocity(), constants::C_0, max_relative = 1e-10);
    }

    #[test]
    fn dielectric_slows_wave() {
        let epsilon_r = 4.0;
        let w = TravelingWaveParams::in_dielectric(1.0, 1e9, 0.0, Direction::PositiveX, epsilon_r);
        assert_relative_eq!(
            w.phase_velocity(),
            constants::C_0 / epsilon_r.sqrt(),
            max_relative = 1e-10
        );
    }

    #[test]
    fn dielectric_shortens_wavelength() {
        let epsilon_r = 9.0;
        let w = TravelingWaveParams::in_dielectric(1.0, 1e9, 0.0, Direction::PositiveX, epsilon_r);
        let lambda_0 = constants::C_0 / 1e9;
        assert_relative_eq!(
            w.wavelength(),
            lambda_0 / epsilon_r.sqrt(),
            max_relative = 1e-10
        );
    }

    #[test]
    fn positive_x_wave_propagates_right() {
        // At t=0, peak is at x=0. After time t = T/4, peak should have moved to x = λ/4
        let w = TravelingWaveParams::in_free_space(1.0, 1e9, 0.0, Direction::PositiveX);
        let t = 0.25 / w.frequency; // T/4
        let x_peak = w.wavelength() / 4.0; // λ/4
        // y(λ/4, T/4) = cos(ωT/4 - βλ/4) = cos(π/2 - π/2) = cos(0) = 1
        assert_relative_eq!(w.evaluate(x_peak, t), 1.0, epsilon = 1e-10);
    }

    #[test]
    fn negative_x_wave_propagates_left() {
        let w = TravelingWaveParams::in_free_space(1.0, 1e9, 0.0, Direction::NegativeX);
        let t = 0.25 / w.frequency; // T/4
        let x_peak = -w.wavelength() / 4.0; // -λ/4
        // y(-λ/4, T/4) = cos(ωT/4 + β(-λ/4)) = cos(π/2 - π/2) = 1
        assert_relative_eq!(w.evaluate(x_peak, t), 1.0, epsilon = 1e-10);
    }

    #[test]
    fn lossy_wave_decays_in_propagation_direction() {
        let w = TravelingWaveParams::with_propagation(
            1.0, 1e9, 0.0,
            Direction::PositiveX,
            0.1, // α = 0.1 Np/m
            2.0 * PI * 1e9 / constants::C_0,
        );
        let y_near = w.evaluate(0.0, 0.0).abs();
        let y_far = w.evaluate(10.0, 0.0).abs();
        assert!(y_far < y_near, "wave must decay in +x for +x propagation");
    }

    #[test]
    fn skin_depth_lossless_is_infinity() {
        let w = TravelingWaveParams::in_free_space(1.0, 1e9, 0.0, Direction::PositiveX);
        assert!(w.skin_depth().is_infinite());
    }

    #[test]
    fn skin_depth_lossy() {
        let w = TravelingWaveParams::with_propagation(
            1.0, 1e9, 0.0, Direction::PositiveX, 2.0, 20.0,
        );
        assert_relative_eq!(w.skin_depth(), 0.5, epsilon = 1e-12);
    }

    #[test]
    fn sample_space_length_correct() {
        let w = TravelingWaveParams::in_free_space(1.0, 1e9, 0.0, Direction::PositiveX);
        let (xs, ys) = w.sample_space(0.0, 1.0, 200, 0.0);
        assert_eq!(xs.len(), 200);
        assert_eq!(ys.len(), 200);
    }

    #[test]
    fn sample_space_values_match_evaluate() {
        let w = TravelingWaveParams::in_free_space(3.0, 1e9, 0.5, Direction::NegativeX);
        let t = 1e-10;
        let (xs, ys) = w.sample_space(0.0, 1.0, 50, t);
        for (xi, yi) in xs.iter().zip(ys.iter()) {
            assert_relative_eq!(*yi, w.evaluate(*xi, t), epsilon = 1e-12);
        }
    }

    #[test]
    fn sample_time_values_match_evaluate() {
        let w = TravelingWaveParams::in_free_space(2.0, 1e9, 0.0, Direction::PositiveX);
        let x = 0.1;
        let (ts, ys) = w.sample_time(0.0, 1e-9, 50, x);
        for (ti, yi) in ts.iter().zip(ys.iter()) {
            assert_relative_eq!(*yi, w.evaluate(x, *ti), epsilon = 1e-12);
        }
    }

    #[test]
    fn superpose_incident_and_reflected_creates_standing_wave() {
        // Incident + reflected (same amplitude) → standing wave with nodes
        let f = 1e9;
        let fwd = TravelingWaveParams::in_free_space(1.0, f, 0.0, Direction::PositiveX);
        let bwd = TravelingWaveParams::in_free_space(1.0, f, 0.0, Direction::NegativeX);

        // At t=0: y = cos(-βx) + cos(βx) = 2cos(βx)
        // Node at x = λ/4 where cos(π/2) = 0
        let lambda = fwd.wavelength();
        let (xs, ys) = superpose_spatial(&[fwd, bwd], 0.0, lambda, 1001, 0.0);

        // Check antinode at x=0
        assert_relative_eq!(ys[0], 2.0, epsilon = 1e-10);

        // Check node near x = λ/4
        let idx_quarter = (0.25 * 1000.0) as usize;
        assert_relative_eq!(ys[idx_quarter], 0.0, epsilon = 1e-4);
    }
}

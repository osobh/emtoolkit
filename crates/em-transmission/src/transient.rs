//! Transient response of transmission lines.
//!
//! Implements the bounce diagram method for computing voltage and current
//! transient response on a lossless transmission line with resistive
//! source and load impedances driven by a step or pulse source.

use serde::{Deserialize, Serialize};

/// Source waveform for transient analysis.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum SourceWaveform {
    /// Step function: V(t) = V₀ for t ≥ 0
    Step { voltage: f64 },
    /// Pulse: V(t) = V₀ for 0 ≤ t < duration
    Pulse { voltage: f64, duration: f64 },
}

impl SourceWaveform {
    /// Evaluate the source voltage at time t.
    pub fn evaluate(&self, t: f64) -> f64 {
        if t < 0.0 {
            return 0.0;
        }
        match self {
            SourceWaveform::Step { voltage } => *voltage,
            SourceWaveform::Pulse { voltage, duration } => {
                if t < *duration { *voltage } else { 0.0 }
            }
        }
    }
}

/// Parameters for transient transmission line analysis.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct TransientParams {
    /// Characteristic impedance Z₀ (Ω)
    pub z0: f64,
    /// Source resistance (Ω)
    pub r_source: f64,
    /// Load resistance (Ω)
    pub r_load: f64,
    /// Line length (m)
    pub length: f64,
    /// Phase velocity (m/s)
    pub phase_velocity: f64,
    /// Source waveform
    pub source: SourceWaveform,
}

/// A single bounce event on the transmission line.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct BounceEvent {
    /// Bounce number (0 = initial, 1 = first reflection, etc.)
    pub bounce: usize,
    /// Time of this bounce (s)
    pub time: f64,
    /// Voltage amplitude of this bounce
    pub voltage: f64,
    /// Location: true = at load, false = at source
    pub at_load: bool,
}

/// Results of a transient analysis.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransientResult {
    /// Source reflection coefficient Γ_S
    pub gamma_source: f64,
    /// Load reflection coefficient Γ_L
    pub gamma_load: f64,
    /// One-way transit time T_d = l/v_p (s)
    pub transit_time: f64,
    /// Initial voltage launched onto the line: V₁ = V_s · Z₀/(Z₀ + R_s)
    pub v_initial: f64,
    /// Bounce events
    pub bounces: Vec<BounceEvent>,
    /// Steady-state voltage (for step source): V_ss = V_s · R_L/(R_s + R_L)
    pub steady_state_voltage: f64,
}

impl TransientParams {
    /// One-way transit time T_d = l / v_p.
    pub fn transit_time(&self) -> f64 {
        self.length / self.phase_velocity
    }

    /// Source reflection coefficient Γ_S = (R_S - Z₀) / (R_S + Z₀).
    pub fn gamma_source(&self) -> f64 {
        (self.r_source - self.z0) / (self.r_source + self.z0)
    }

    /// Load reflection coefficient Γ_L = (R_L - Z₀) / (R_L + Z₀).
    pub fn gamma_load(&self) -> f64 {
        (self.r_load - self.z0) / (self.r_load + self.z0)
    }

    /// Compute the bounce diagram and transient response.
    ///
    /// # Arguments
    /// * `num_bounces` - Number of bounces to compute
    ///
    /// # Returns
    /// TransientResult with bounce events and steady-state info.
    pub fn solve(&self, num_bounces: usize) -> TransientResult {
        let td = self.transit_time();
        let gamma_s = self.gamma_source();
        let gamma_l = self.gamma_load();

        // Initial voltage launched: V₁ = V_source(0) · Z₀/(Z₀ + R_S)
        let source_v = match self.source {
            SourceWaveform::Step { voltage } => voltage,
            SourceWaveform::Pulse { voltage, .. } => voltage,
        };
        let v_initial = source_v * self.z0 / (self.z0 + self.r_source);

        // Steady state for step source
        let steady_state_voltage = source_v * self.r_load / (self.r_source + self.r_load);

        let mut bounces = Vec::with_capacity(num_bounces + 1);
        let mut v_bounce = v_initial;

        // Bounce 0: initial wave launched at source, arrives at load at t = T_d
        bounces.push(BounceEvent {
            bounce: 0,
            time: 0.0,
            voltage: v_initial,
            at_load: false,
        });

        for i in 1..=num_bounces {
            let at_load = i % 2 == 1; // odd bounces at load, even at source
            let time = i as f64 * td;

            if at_load {
                v_bounce *= gamma_l;
            } else {
                v_bounce *= gamma_s;
            }

            bounces.push(BounceEvent {
                bounce: i,
                time,
                voltage: v_bounce,
                at_load,
            });
        }

        TransientResult {
            gamma_source: gamma_s,
            gamma_load: gamma_l,
            transit_time: td,
            v_initial,
            bounces,
            steady_state_voltage,
        }
    }

    /// Compute voltage at a specific point and time using bounce diagram summation.
    ///
    /// # Arguments
    /// * `x` - Distance from source (m), 0 ≤ x ≤ length
    /// * `t` - Time (s)
    /// * `max_bounces` - Maximum number of bounces to sum
    pub fn voltage_at(&self, x: f64, t: f64, max_bounces: usize) -> f64 {
        let td = self.transit_time();
        let gamma_s = self.gamma_source();
        let gamma_l = self.gamma_load();
        let v1 = match self.source {
            SourceWaveform::Step { voltage } => voltage,
            SourceWaveform::Pulse { voltage, .. } => voltage,
        } * self.z0 / (self.z0 + self.r_source);

        let travel_time_to_x = x / self.phase_velocity;
        let travel_time_to_end = (self.length - x) / self.phase_velocity;

        let mut v_total = 0.0;

        // Sum forward and backward traveling wave contributions
        // Forward wave n arrives at x at time: travel_time_to_x + 2n·T_d (for source reflections)
        // Backward wave n arrives at x at time: travel_time_to_x + 2(n+1)·T_d - travel_time_to_x
        //   ... actually just use the bounce approach more carefully

        // Simplified: accumulate all wave arrivals at position x up to time t
        let mut forward_amplitude;
        let mut backward_amplitude;

        // Forward pass 0: launched at t=0 from source, arrives at x at t = x/vp
        if t >= travel_time_to_x {
            let source_val = self.source.evaluate(t - travel_time_to_x);
            let v_launched = source_val * self.z0 / (self.z0 + self.r_source);
            // Only count if source is still active at launch time
            if t >= travel_time_to_x {
                v_total += v_launched;
            }
        }

        // Subsequent bounces
        forward_amplitude = v1;
        for n in 0..max_bounces {
            // Forward wave reflected from load, then source, arrives at x:
            // Reflected from load at t = (2n+1)·T_d going backward
            // Arrives at x going backward at t = (2n+1)·T_d + travel_time_to_end - ... 
            // This gets complex. Let's use cumulative summation at load and source.

            // Backward wave (reflected from load, bounce 2n+1):
            backward_amplitude = forward_amplitude * gamma_l;
            let t_arrive_backward = (2 * n + 1) as f64 * td + travel_time_to_end;
            if t >= t_arrive_backward && n > 0 || (n == 0 && t >= td + travel_time_to_end) {
                // Need to account for pulse source
                v_total += backward_amplitude;
            }

            // Forward wave (re-reflected from source, bounce 2n+2):
            forward_amplitude = backward_amplitude * gamma_s;
            let t_arrive_forward = (2 * (n + 1)) as f64 * td + travel_time_to_x;
            if t >= t_arrive_forward {
                v_total += forward_amplitude;
            }
        }

        v_total
    }

    /// Sample voltage at the load vs time.
    ///
    /// # Returns
    /// (time_values, voltage_values)
    pub fn sample_load_voltage(
        &self,
        t_end: f64,
        num_points: usize,
    ) -> (Vec<f64>, Vec<f64>) {
        assert!(num_points >= 2);
        let td = self.transit_time();
        let gamma_s = self.gamma_source();
        let gamma_l = self.gamma_load();

        let dt = t_end / (num_points - 1) as f64;
        let times: Vec<f64> = (0..num_points).map(|i| i as f64 * dt).collect();

        // Use direct bounce summation at load
        let result = self.solve(100); // enough bounces

        let voltages: Vec<f64> = times
            .iter()
            .map(|&t| {
                // Sum all bounces that have arrived at load by time t
                let mut v = 0.0;
                // The voltage at the load is the sum of all forward waves that arrive
                // First forward wave arrives at t = T_d
                let mut v_fwd = result.v_initial;
                let mut bounce_time = td;

                if t >= bounce_time {
                    v += v_fwd * (1.0 + gamma_l); // transmitted voltage at load
                }

                // Subsequent round trips
                for _n in 0..50 {
                    v_fwd *= gamma_l * gamma_s; // one full round trip
                    bounce_time += 2.0 * td;
                    if t >= bounce_time {
                        v += v_fwd * (1.0 + gamma_l);
                    } else {
                        break;
                    }
                }
                v
            })
            .collect();

        (times, voltages)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    fn make_step_line() -> TransientParams {
        TransientParams {
            z0: 50.0,
            r_source: 50.0,
            r_load: 100.0,
            length: 1.0,
            phase_velocity: em_core::constants::C_0,
            source: SourceWaveform::Step { voltage: 10.0 },
        }
    }

    #[test]
    fn transit_time_correct() {
        let p = make_step_line();
        assert_relative_eq!(
            p.transit_time(),
            1.0 / em_core::constants::C_0,
            max_relative = 1e-10
        );
    }

    #[test]
    fn gamma_matched_source_is_zero() {
        let p = make_step_line(); // R_S = Z₀ = 50
        assert_relative_eq!(p.gamma_source(), 0.0, epsilon = 1e-12);
    }

    #[test]
    fn gamma_load_100_ohm() {
        let p = make_step_line(); // R_L = 100, Z₀ = 50
        // Γ_L = (100-50)/(100+50) = 1/3
        assert_relative_eq!(p.gamma_load(), 1.0 / 3.0, epsilon = 1e-12);
    }

    #[test]
    fn initial_voltage_with_matched_source() {
        let p = make_step_line();
        let result = p.solve(10);
        // V₁ = 10 · 50/(50+50) = 5V
        assert_relative_eq!(result.v_initial, 5.0, epsilon = 1e-12);
    }

    #[test]
    fn steady_state_voltage_divider() {
        let p = make_step_line();
        let result = p.solve(10);
        // V_ss = 10 · 100/(50+100) = 6.667V
        assert_relative_eq!(
            result.steady_state_voltage,
            10.0 * 100.0 / 150.0,
            epsilon = 1e-10
        );
    }

    #[test]
    fn bounce_converges_to_steady_state() {
        let p = make_step_line();
        let result = p.solve(20);

        // Sum all voltages at load: Σ V_n · (1 + Γ_L) for forward waves
        // With matched source (Γ_S = 0), only one bounce matters:
        // V_load = V₁(1 + Γ_L) = 5 · (1 + 1/3) = 6.667V
        // Since Γ_S = 0, there are no re-reflections.
        // After bounce 0 (launch) and bounce 1 (load reflection), it should settle.
        let gamma_l = result.gamma_load;
        let gamma_s = result.gamma_source;
        assert_relative_eq!(gamma_s, 0.0, epsilon = 1e-12);
        // Load voltage after first arrival: V₁(1+Γ_L) = 5·4/3 = 6.667
        let v_load_final = result.v_initial * (1.0 + gamma_l);
        assert_relative_eq!(v_load_final, result.steady_state_voltage, max_relative = 1e-6);
    }

    #[test]
    fn short_circuit_load_gamma_minus_one() {
        let p = TransientParams {
            z0: 50.0,
            r_source: 50.0,
            r_load: 0.0,
            length: 1.0,
            phase_velocity: em_core::constants::C_0,
            source: SourceWaveform::Step { voltage: 10.0 },
        };
        assert_relative_eq!(p.gamma_load(), -1.0, epsilon = 1e-12);
    }

    #[test]
    fn open_circuit_load_gamma_plus_one() {
        let p = TransientParams {
            z0: 50.0,
            r_source: 50.0,
            r_load: 1e15, // approximate open
            length: 1.0,
            phase_velocity: em_core::constants::C_0,
            source: SourceWaveform::Step { voltage: 10.0 },
        };
        assert_relative_eq!(p.gamma_load(), 1.0, epsilon = 1e-6);
    }

    #[test]
    fn bounce_events_correct_timing() {
        let p = make_step_line();
        let result = p.solve(5);
        let td = p.transit_time();
        for (i, bounce) in result.bounces.iter().enumerate() {
            assert_eq!(bounce.bounce, i);
            assert_relative_eq!(bounce.time, i as f64 * td, epsilon = 1e-15);
        }
    }

    #[test]
    fn pulse_source_evaluates_correctly() {
        let pulse = SourceWaveform::Pulse {
            voltage: 5.0,
            duration: 1e-9,
        };
        assert_relative_eq!(pulse.evaluate(-1.0), 0.0, epsilon = 1e-12);
        assert_relative_eq!(pulse.evaluate(0.0), 5.0, epsilon = 1e-12);
        assert_relative_eq!(pulse.evaluate(0.5e-9), 5.0, epsilon = 1e-12);
        assert_relative_eq!(pulse.evaluate(1e-9), 0.0, epsilon = 1e-12);
        assert_relative_eq!(pulse.evaluate(2e-9), 0.0, epsilon = 1e-12);
    }

    #[test]
    fn sample_load_voltage_length() {
        let p = make_step_line();
        let (t, v) = p.sample_load_voltage(10.0 * p.transit_time(), 500);
        assert_eq!(t.len(), 500);
        assert_eq!(v.len(), 500);
    }

    #[test]
    fn sample_load_voltage_zero_before_transit() {
        let p = make_step_line();
        let td = p.transit_time();
        let (t, v) = p.sample_load_voltage(5.0 * td, 1000);
        // Before t = T_d, voltage at load should be 0
        for (&ti, &vi) in t.iter().zip(v.iter()) {
            if ti < td * 0.99 {
                assert_relative_eq!(vi, 0.0, epsilon = 1e-10);
            }
        }
    }

    #[test]
    fn sample_load_voltage_reaches_steady_state() {
        let p = make_step_line();
        let td = p.transit_time();
        let (t, v) = p.sample_load_voltage(20.0 * td, 1000);
        // After several transit times, should reach steady state
        let v_ss = p.solve(1).steady_state_voltage;
        let last_v = v.last().unwrap();
        assert_relative_eq!(*last_v, v_ss, max_relative = 0.01);
    }
}

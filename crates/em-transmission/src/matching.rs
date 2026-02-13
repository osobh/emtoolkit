//! Impedance matching network design.
//!
//! Implements quarter-wave transformer and lumped-element (L-network) matching.

use num_complex::Complex64;
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

/// Quarter-wave transformer design result.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct QuarterWaveTransformer {
    /// Required transformer impedance Z_T = √(Z₀·R_L) (Ω)
    pub z_transformer: f64,
    /// Physical length of transformer (m)
    pub length: f64,
    /// Operating frequency (Hz)
    pub frequency: f64,
    /// Fractional bandwidth for max VSWR
    pub bandwidth_fractional: f64,
}

/// Design a single-section quarter-wave transformer.
///
/// Matches a real load R_L to a real Z₀ at frequency f.
/// Z_T = √(Z₀ · R_L), length = λ/4.
///
/// # Arguments
/// * `z0` - Source characteristic impedance (Ω)
/// * `r_load` - Load resistance (must be real, Ω)
/// * `frequency` - Design frequency (Hz)
/// * `phase_velocity` - Phase velocity in the transformer section (m/s)
/// * `max_vswr` - Maximum acceptable VSWR for bandwidth calculation
pub fn quarter_wave_single(
    z0: f64,
    r_load: f64,
    frequency: f64,
    phase_velocity: f64,
    max_vswr: f64,
) -> QuarterWaveTransformer {
    let z_t = (z0 * r_load).sqrt();
    let wavelength = phase_velocity / frequency;
    let length = wavelength / 4.0;

    // Bandwidth: Δf/f₀ = (4/π)·arccos(Γ_m·2·√(Z₀·R_L) / |Z₀ - R_L|)
    // where Γ_m = (VSWR - 1)/(VSWR + 1)
    let gamma_m = (max_vswr - 1.0) / (max_vswr + 1.0);
    let cos_arg = gamma_m * 2.0 * z_t / (z0 - r_load).abs();
    let bandwidth = if cos_arg.abs() <= 1.0 {
        2.0 - (4.0 / PI) * cos_arg.acos()
    } else {
        2.0 // all frequencies match (load already close to z0)
    };

    QuarterWaveTransformer {
        z_transformer: z_t,
        length,
        frequency,
        bandwidth_fractional: bandwidth,
    }
}

/// Multi-section quarter-wave transformer design result.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MultiSectionTransformer {
    /// Impedance of each section (Ω), from source to load
    pub section_impedances: Vec<f64>,
    /// Physical length of each section (all λ/4 at design frequency) (m)
    pub section_length: f64,
    /// Design frequency (Hz)
    pub frequency: f64,
}

/// Design an N-section binomial (maximally flat) quarter-wave transformer.
///
/// The binomial design provides maximally flat response at the design frequency.
/// Section impedances follow: ln(Z_{n+1}/Z_n) = 2^(-N) · C(N,n) · ln(R_L/Z₀)
pub fn quarter_wave_binomial(
    z0: f64,
    r_load: f64,
    frequency: f64,
    phase_velocity: f64,
    num_sections: usize,
) -> MultiSectionTransformer {
    let n = num_sections;
    let ln_ratio = (r_load / z0).ln();
    let section_length = phase_velocity / (4.0 * frequency);

    let mut impedances = Vec::with_capacity(n);
    let mut z_prev = z0;

    for i in 0..n {
        let binom_coeff = binomial(n, i) as f64;
        let two_neg_n = 2.0_f64.powi(-(n as i32));
        let ln_step = two_neg_n * binom_coeff * ln_ratio;
        let z_next = z_prev * ln_step.exp();
        impedances.push(z_next);
        z_prev = z_next;
    }

    MultiSectionTransformer {
        section_impedances: impedances,
        section_length,
        frequency,
    }
}

/// Compute binomial coefficient C(n, k).
fn binomial(n: usize, k: usize) -> usize {
    if k > n {
        return 0;
    }
    let k = k.min(n - k);
    let mut result = 1usize;
    for i in 0..k {
        result = result * (n - i) / (i + 1);
    }
    result
}

/// L-network matching topology.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LNetworkTopology {
    /// Series element first (closer to source), then shunt element
    SeriesShunt,
    /// Shunt element first, then series element
    ShuntSeries,
}

/// L-network matching result.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct LNetworkMatch {
    pub topology: LNetworkTopology,
    /// Series reactance (Ω). Positive = inductor, negative = capacitor.
    pub x_series: f64,
    /// Shunt susceptance (S). Positive = capacitor, negative = inductor.
    pub b_shunt: f64,
    /// Series component value: inductance (H) if positive, capacitance (F) if negative x.
    pub series_component: ComponentValue,
    /// Shunt component value: capacitance (F) if positive b, inductance (H) if negative b.
    pub shunt_component: ComponentValue,
}

/// A reactive component value.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ComponentValue {
    Inductor { henries: f64 },
    Capacitor { farads: f64 },
}

impl ComponentValue {
    fn from_reactance(x: f64, omega: f64) -> Self {
        if x >= 0.0 {
            ComponentValue::Inductor {
                henries: x / omega,
            }
        } else {
            ComponentValue::Capacitor {
                farads: -1.0 / (omega * x),
            }
        }
    }

    fn from_susceptance(b: f64, omega: f64) -> Self {
        if b >= 0.0 {
            ComponentValue::Capacitor {
                farads: b / omega,
            }
        } else {
            ComponentValue::Inductor {
                henries: -1.0 / (omega * b),
            }
        }
    }
}

/// Design an L-network to match a complex load to a real source impedance.
///
/// Returns up to 2 solutions (two topologies may each have a solution).
///
/// # Arguments
/// * `z0` - Real source impedance (Ω)
/// * `z_load` - Complex load impedance (Ω)
/// * `frequency` - Operating frequency (Hz)
pub fn l_network(z0: f64, z_load: Complex64, frequency: f64) -> Vec<LNetworkMatch> {
    let omega = 2.0 * PI * frequency;
    let r_l = z_load.re;
    let x_l = z_load.im;
    let mut solutions = Vec::new();

    // Case 1: R_L > Z₀ → use shunt-series topology
    // Case 2: R_L < Z₀ → use series-shunt topology
    // Both cases can produce two solutions (±)

    if r_l != z0 {
        let q_sq = if r_l > z0 {
            // Shunt-series: need to transform R_L down to Z₀
            (r_l / z0 - 1.0).max(0.0)
        } else {
            // Series-shunt: need to transform Z₀ down to R_L
            (z0 / r_l - 1.0).max(0.0)
        };

        if q_sq >= 0.0 {
            let q = q_sq.sqrt();

            if r_l > z0 {
                // Shunt-series topology
                for sign in [1.0, -1.0] {
                    let _b_shunt = sign * q / r_l;
                    // Account for load reactance
                    let _x_series = -(x_l + sign * r_l * q) + z0 * q * sign;
                    // Correct: x_series should make the total reactance cancel
                    let _x_s = sign * z0 * q - x_l; // simplified estimate

                    // More precise: after shunting, the real part should be Z₀
                    // Shunt element: B = ±√((R_L - Z₀)/(Z₀·R_L²))... use direct formula
                    let b = sign * ((r_l - z0) / (z0 * r_l * r_l + z0 * x_l * x_l)).sqrt();
                    if b.is_nan() {
                        continue;
                    }
                    let x_s_val = 1.0 / b + x_l * z0 / r_l - z0 / (b * r_l);

                    solutions.push(LNetworkMatch {
                        topology: LNetworkTopology::ShuntSeries,
                        x_series: x_s_val,
                        b_shunt: b,
                        series_component: ComponentValue::from_reactance(x_s_val, omega),
                        shunt_component: ComponentValue::from_susceptance(b, omega),
                    });
                }
            } else {
                // Series-shunt topology
                for sign in [1.0, -1.0] {
                    let x_s = sign * (z0 * (z0 - r_l)).sqrt() - x_l;
                    if x_s.is_nan() {
                        continue;
                    }
                    let _x_total = x_s + x_l;
                    let b = -sign * (z0 - r_l).sqrt() / (z0 * r_l.sqrt());
                    if b.is_nan() {
                        continue;
                    }

                    solutions.push(LNetworkMatch {
                        topology: LNetworkTopology::SeriesShunt,
                        x_series: x_s,
                        b_shunt: b,
                        series_component: ComponentValue::from_reactance(x_s, omega),
                        shunt_component: ComponentValue::from_susceptance(b, omega),
                    });
                }
            }
        }
    }

    solutions
}

/// Compute the reflection coefficient of a matching network at a given frequency.
///
/// For a quarter-wave transformer section between Z₀ and R_L.
pub fn quarter_wave_gamma_vs_frequency(
    z_transformer: f64,
    z0: f64,
    r_load: f64,
    design_freq: f64,
    eval_freq: f64,
) -> Complex64 {
    let theta = PI / 2.0 * eval_freq / design_freq; // electrical length
    let z_in = em_core::complex::input_impedance_lossless(z_transformer, Complex64::new(r_load, 0.0), theta);
    em_core::complex::reflection_coefficient(z_in, Complex64::new(z0, 0.0))
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    // ================================================================
    // Quarter-wave transformer
    // ================================================================

    #[test]
    fn qw_transformer_impedance_geometric_mean() {
        let t = quarter_wave_single(50.0, 100.0, 1e9, em_core::constants::C_0, 2.0);
        assert_relative_eq!(t.z_transformer, (50.0 * 100.0_f64).sqrt(), epsilon = 1e-10);
    }

    #[test]
    fn qw_transformer_length_is_quarter_wave() {
        let f = 1e9;
        let vp = em_core::constants::C_0;
        let t = quarter_wave_single(50.0, 100.0, f, vp, 2.0);
        assert_relative_eq!(t.length, vp / (4.0 * f), epsilon = 1e-10);
    }

    #[test]
    fn qw_transformer_perfect_match_at_design_freq() {
        let z0 = 50.0;
        let rl = 200.0;
        let f0 = 1e9;
        let t = quarter_wave_single(z0, rl, f0, em_core::constants::C_0, 2.0);
        let gamma = quarter_wave_gamma_vs_frequency(t.z_transformer, z0, rl, f0, f0);
        assert_relative_eq!(gamma.norm(), 0.0, epsilon = 1e-10);
    }

    #[test]
    fn qw_transformer_nonzero_reflection_off_frequency() {
        let z0 = 50.0;
        let rl = 200.0;
        let f0 = 1e9;
        let t = quarter_wave_single(z0, rl, f0, em_core::constants::C_0, 2.0);
        let gamma = quarter_wave_gamma_vs_frequency(t.z_transformer, z0, rl, f0, 0.8 * f0);
        assert!(gamma.norm() > 0.01, "should have mismatch off frequency");
    }

    #[test]
    fn qw_transformer_bandwidth_positive() {
        let t = quarter_wave_single(50.0, 200.0, 1e9, em_core::constants::C_0, 2.0);
        assert!(t.bandwidth_fractional > 0.0);
        assert!(t.bandwidth_fractional <= 2.0);
    }

    // ================================================================
    // Multi-section transformer
    // ================================================================

    #[test]
    fn binomial_single_section_matches_simple() {
        let z0 = 50.0;
        let rl = 100.0;
        let f = 1e9;
        let vp = em_core::constants::C_0;
        let multi = quarter_wave_binomial(z0, rl, f, vp, 1);
        let single = quarter_wave_single(z0, rl, f, vp, 2.0);
        assert_eq!(multi.section_impedances.len(), 1);
        assert_relative_eq!(
            multi.section_impedances[0],
            single.z_transformer,
            max_relative = 1e-6
        );
    }

    #[test]
    fn binomial_two_section_impedances_bracket_geometric_mean() {
        let z0 = 50.0;
        let rl = 200.0;
        let multi = quarter_wave_binomial(z0, rl, 1e9, em_core::constants::C_0, 2);
        assert_eq!(multi.section_impedances.len(), 2);
        let z1 = multi.section_impedances[0];
        let z2 = multi.section_impedances[1];
        // First section should be between Z₀ and √(Z₀·R_L)
        assert!(z1 > z0 && z1 < rl);
        // Second section should be between first and R_L
        assert!(z2 > z1 && z2 <= rl * 1.01);
    }

    #[test]
    fn binomial_coefficient_known_values() {
        assert_eq!(binomial(4, 0), 1);
        assert_eq!(binomial(4, 1), 4);
        assert_eq!(binomial(4, 2), 6);
        assert_eq!(binomial(4, 3), 4);
        assert_eq!(binomial(4, 4), 1);
    }

    // ================================================================
    // L-network matching
    // ================================================================

    #[test]
    fn l_network_returns_solutions_for_mismatch() {
        let solutions = l_network(50.0, Complex64::new(100.0, 0.0), 1e9);
        assert!(!solutions.is_empty(), "should find at least one solution");
    }

    #[test]
    fn l_network_matched_returns_empty() {
        let solutions = l_network(50.0, Complex64::new(50.0, 0.0), 1e9);
        assert!(solutions.is_empty(), "matched load needs no network");
    }

    #[test]
    fn l_network_components_have_correct_types() {
        let solutions = l_network(50.0, Complex64::new(100.0, 0.0), 1e9);
        for sol in &solutions {
            // Every solution should have a series and shunt component
            match sol.series_component {
                ComponentValue::Inductor { henries } => assert!(henries > 0.0),
                ComponentValue::Capacitor { farads } => assert!(farads > 0.0),
            }
            match sol.shunt_component {
                ComponentValue::Inductor { henries } => assert!(henries > 0.0),
                ComponentValue::Capacitor { farads } => assert!(farads > 0.0),
            }
        }
    }
}

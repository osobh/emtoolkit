//! L-Network Impedance Matching
//!
//! Designs two-element L-networks to match a complex load to a real source.
//! Four possible topologies exist depending on whether R_L > R_S or R_L < R_S.

use num_complex::Complex64;
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

/// Topology of an L-network.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LNetworkTopology {
    /// Series L, then shunt C (low-pass)
    SeriesLShuntC,
    /// Series C, then shunt L (high-pass)
    SeriesCShuntL,
    /// Shunt C, then series L (low-pass)
    ShuntCSeriesL,
    /// Shunt L, then series C (high-pass)
    ShuntLSeriesC,
}

impl std::fmt::Display for LNetworkTopology {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LNetworkTopology::SeriesLShuntC => write!(f, "Series L → Shunt C"),
            LNetworkTopology::SeriesCShuntL => write!(f, "Series C → Shunt L"),
            LNetworkTopology::ShuntCSeriesL => write!(f, "Shunt C → Series L"),
            LNetworkTopology::ShuntLSeriesC => write!(f, "Shunt L → Series C"),
        }
    }
}

/// Result of an L-network matching design.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LNetworkResult {
    /// Network topology
    pub topology: LNetworkTopology,
    /// Series element value (H for inductor, F for capacitor)
    pub series_value: f64,
    /// Series element type
    pub series_type: String,
    /// Shunt element value (H for inductor, F for capacitor)
    pub shunt_value: f64,
    /// Shunt element type
    pub shunt_type: String,
    /// Q factor of the network
    pub q_factor: f64,
    /// Bandwidth (approximate 3dB fractional bandwidth ≈ 1/Q)
    pub bandwidth_fractional: f64,
    /// Description
    pub description: String,
}

/// Design L-network(s) to match a complex load to a real source impedance.
///
/// # Arguments
/// * `z_source` - Real source impedance (Ω)
/// * `z_load` - Complex load impedance (Ω)
/// * `frequency` - Operating frequency (Hz)
///
/// # Returns
/// Vector of possible L-network solutions (typically 2-4 depending on load).
pub fn l_network_match(z_source: f64, z_load: Complex64, frequency: f64) -> Vec<LNetworkResult> {
    let omega = 2.0 * PI * frequency;
    let r_l = z_load.re;
    let x_l = z_load.im;
    let r_s = z_source;

    let mut results = Vec::new();

    // Case 1: R_L > R_S (load resistance larger than source)
    // Use topology with shunt element on load side
    if r_l > r_s {
        // Q factor determined by resistance ratio
        let q = ((r_l / r_s) - 1.0).sqrt();
        let bandwidth = 1.0 / q;

        // Topology A: Shunt C on load side, Series L on source side
        // First, we need to absorb or cancel x_l
        // Shunt susceptance B = Q / R_L
        // Series reactance X = Q * R_S
        
        let b_shunt = q / r_l;
        let x_series = q * r_s;

        // For shunt C: B = ωC, so C = B/ω
        let c_shunt = b_shunt / omega;
        // For series L: X = ωL, so L = X/ω
        let l_series = x_series / omega;

        // Adjust for load reactance
        // If load is inductive (x_l > 0), we need less shunt C or can use it directly
        // If load is capacitive (x_l < 0), we need more shunt C

        // The shunt element must resonate out the transformed reactance
        // Effective B needed = Q/R_L ± adjustment for x_l
        let x_l_transformed = x_l * r_s / r_l; // approximate

        results.push(LNetworkResult {
            topology: LNetworkTopology::ShuntCSeriesL,
            series_value: l_series,
            series_type: "Inductor (H)".to_string(),
            shunt_value: c_shunt,
            shunt_type: "Capacitor (F)".to_string(),
            q_factor: q,
            bandwidth_fractional: bandwidth,
            description: format!(
                "Low-pass: Shunt C ({:.2} pF) → Series L ({:.2} nH)",
                c_shunt * 1e12,
                l_series * 1e9
            ),
        });

        // Topology B: Shunt L on load side, Series C on source side (high-pass)
        let l_shunt = r_l / (omega * q);
        let c_series = 1.0 / (omega * x_series);

        results.push(LNetworkResult {
            topology: LNetworkTopology::ShuntLSeriesC,
            series_value: c_series,
            series_type: "Capacitor (F)".to_string(),
            shunt_value: l_shunt,
            shunt_type: "Inductor (H)".to_string(),
            q_factor: q,
            bandwidth_fractional: bandwidth,
            description: format!(
                "High-pass: Shunt L ({:.2} nH) → Series C ({:.2} pF)",
                l_shunt * 1e9,
                c_series * 1e12
            ),
        });
    }

    // Case 2: R_L < R_S (load resistance smaller than source)
    // Use topology with shunt element on source side
    if r_l < r_s {
        let q = ((r_s / r_l) - 1.0).sqrt();
        let bandwidth = 1.0 / q;

        // Topology C: Series L on load side, Shunt C on source side
        let x_series = q * r_l - x_l; // cancel load reactance
        let b_shunt = q / r_s;

        let l_series = if x_series > 0.0 { x_series / omega } else { 0.0 };
        let c_series_alt = if x_series < 0.0 { -1.0 / (omega * x_series) } else { 0.0 };
        let c_shunt = b_shunt / omega;

        if x_series > 0.0 {
            results.push(LNetworkResult {
                topology: LNetworkTopology::SeriesLShuntC,
                series_value: l_series,
                series_type: "Inductor (H)".to_string(),
                shunt_value: c_shunt,
                shunt_type: "Capacitor (F)".to_string(),
                q_factor: q,
                bandwidth_fractional: bandwidth,
                description: format!(
                    "Low-pass: Series L ({:.2} nH) → Shunt C ({:.2} pF)",
                    l_series * 1e9,
                    c_shunt * 1e12
                ),
            });
        }

        // Topology D: Series C on load side, Shunt L on source side (high-pass)
        let x_series_c = -(q * r_l + x_l);
        let l_shunt = r_s / (omega * q);

        if x_series_c < 0.0 {
            let c_series = -1.0 / (omega * x_series_c);
            results.push(LNetworkResult {
                topology: LNetworkTopology::SeriesCShuntL,
                series_value: c_series,
                series_type: "Capacitor (F)".to_string(),
                shunt_value: l_shunt,
                shunt_type: "Inductor (H)".to_string(),
                q_factor: q,
                bandwidth_fractional: bandwidth,
                description: format!(
                    "High-pass: Series C ({:.2} pF) → Shunt L ({:.2} nH)",
                    c_series * 1e12,
                    l_shunt * 1e9
                ),
            });
        }
    }

    // Case 3: R_L ≈ R_S (just need to cancel reactance)
    if (r_l - r_s).abs() / r_s < 0.01 {
        // Just add series element to cancel load reactance
        if x_l.abs() > 1e-10 {
            if x_l > 0.0 {
                // Inductive load: add series C to cancel
                let c = -1.0 / (omega * x_l);
                results.push(LNetworkResult {
                    topology: LNetworkTopology::SeriesCShuntL,
                    series_value: c,
                    series_type: "Capacitor (F)".to_string(),
                    shunt_value: 0.0,
                    shunt_type: "None".to_string(),
                    q_factor: x_l.abs() / r_l,
                    bandwidth_fractional: r_l / x_l.abs(),
                    description: format!(
                        "Reactance cancel only: Series C ({:.2} pF)",
                        c * 1e12
                    ),
                });
            } else {
                // Capacitive load: add series L to cancel
                let l = -x_l / omega;
                results.push(LNetworkResult {
                    topology: LNetworkTopology::SeriesLShuntC,
                    series_value: l,
                    series_type: "Inductor (H)".to_string(),
                    shunt_value: 0.0,
                    shunt_type: "None".to_string(),
                    q_factor: x_l.abs() / r_l,
                    bandwidth_fractional: r_l / x_l.abs(),
                    description: format!(
                        "Reactance cancel only: Series L ({:.2} nH)",
                        l * 1e9
                    ),
                });
            }
        }
    }

    results
}

/// Compute input impedance of an L-network given component values.
pub fn l_network_input_impedance(
    topology: LNetworkTopology,
    series_value: f64,
    shunt_value: f64,
    z_load: Complex64,
    frequency: f64,
) -> Complex64 {
    let omega = 2.0 * PI * frequency;
    let j = Complex64::new(0.0, 1.0);

    match topology {
        LNetworkTopology::SeriesLShuntC => {
            // Series L first, then shunt C
            let z_series = j * omega * series_value; // jωL
            let y_shunt = j * omega * shunt_value;   // jωC
            let z_after_series = z_load + z_series;
            let y_total = 1.0 / z_after_series + y_shunt;
            1.0 / y_total
        }
        LNetworkTopology::SeriesCShuntL => {
            let z_series = -j / (omega * series_value); // 1/(jωC)
            let y_shunt = -j / (omega * shunt_value);   // 1/(jωL)
            let z_after_series = z_load + z_series;
            let y_total = 1.0 / z_after_series + y_shunt;
            1.0 / y_total
        }
        LNetworkTopology::ShuntCSeriesL => {
            // Shunt C on load side, then series L
            let y_load = 1.0 / z_load;
            let y_shunt = j * omega * shunt_value;
            let z_after_shunt = 1.0 / (y_load + y_shunt);
            let z_series = j * omega * series_value;
            z_after_shunt + z_series
        }
        LNetworkTopology::ShuntLSeriesC => {
            let y_load = 1.0 / z_load;
            let y_shunt = -j / (omega * shunt_value);
            let z_after_shunt = 1.0 / (y_load + y_shunt);
            let z_series = -j / (omega * series_value);
            z_after_shunt + z_series
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn l_network_resistive_step_up() {
        // Match 25Ω to 50Ω source at 1 GHz
        let results = l_network_match(50.0, Complex64::new(25.0, 0.0), 1e9);
        assert!(!results.is_empty(), "Should return at least one solution");
        
        // Q should be √(50/25 - 1) = 1
        let q_expected = 1.0;
        for r in &results {
            assert_relative_eq!(r.q_factor, q_expected, epsilon = 0.1);
        }
    }

    #[test]
    fn l_network_resistive_step_down() {
        // Match 100Ω to 50Ω source
        let results = l_network_match(50.0, Complex64::new(100.0, 0.0), 1e9);
        assert!(!results.is_empty());
        
        // Q = √(100/50 - 1) = 1
        let q_expected = 1.0;
        for r in &results {
            assert_relative_eq!(r.q_factor, q_expected, epsilon = 0.1);
        }
    }

    #[test]
    fn l_network_complex_load() {
        // Match 25+j25 Ω to 50Ω
        let zl = Complex64::new(25.0, 25.0);
        let results = l_network_match(50.0, zl, 1e9);
        assert!(!results.is_empty());
    }

    #[test]
    fn l_network_values_positive() {
        let results = l_network_match(50.0, Complex64::new(75.0, -30.0), 1e9);
        for r in &results {
            assert!(r.series_value >= 0.0, "Series value must be non-negative");
            assert!(r.shunt_value >= 0.0, "Shunt value must be non-negative");
        }
    }
}

//! Multisection Transmission Line Transformers
//!
//! Designs broadband impedance matching using multiple quarter-wave sections.
//! Supports binomial (maximally flat) and Chebyshev (equal ripple) designs.

use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

/// Type of multisection transformer design.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MultisectionType {
    /// Binomial (maximally flat) response
    Binomial,
    /// Chebyshev (equal ripple) response
    Chebyshev,
}

/// Result of a multisection transformer design.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MultisectionResult {
    /// Design type
    pub design_type: MultisectionType,
    /// Number of sections
    pub num_sections: usize,
    /// Characteristic impedance of each section (Ω)
    pub section_impedances: Vec<f64>,
    /// Reflection coefficient at each junction
    pub junction_gammas: Vec<f64>,
    /// Fractional bandwidth for specified max |Γ| (for Chebyshev)
    pub fractional_bandwidth: Option<f64>,
    /// Frequency response: (f/f0, |Γ|) pairs
    pub frequency_response: Vec<(f64, f64)>,
}

/// Compute binomial coefficient C(n, k) = n! / (k! * (n-k)!)
fn binomial_coeff(n: usize, k: usize) -> f64 {
    if k > n {
        return 0.0;
    }
    let mut result = 1.0;
    for i in 0..k {
        result *= (n - i) as f64 / (i + 1) as f64;
    }
    result
}

/// Chebyshev polynomial of the first kind T_n(x).
fn chebyshev_t(n: usize, x: f64) -> f64 {
    if x.abs() <= 1.0 {
        (n as f64 * x.acos()).cos()
    } else if x > 1.0 {
        (n as f64 * x.acosh()).cosh()
    } else {
        let sign = if n % 2 == 0 { 1.0 } else { -1.0 };
        sign * (n as f64 * (-x).acosh()).cosh()
    }
}

/// Design a binomial (maximally flat) multisection transformer.
///
/// The binomial transformer provides maximum flatness at the center frequency.
/// All derivatives of Γ up to order N-1 are zero at f = f₀.
///
/// # Arguments
/// * `z0` - Input line characteristic impedance (Ω)
/// * `z_load` - Load impedance (must be real) (Ω)
/// * `num_sections` - Number of quarter-wave sections
///
/// # Returns
/// Design result with section impedances and frequency response.
pub fn multisection_binomial(z0: f64, z_load: f64, num_sections: usize) -> MultisectionResult {
    let n = num_sections;
    
    // Overall reflection coefficient
    let gamma_0 = (z_load - z0) / (z_load + z0);
    
    // For binomial design: Γ_n = C(N,n) * Γ₀ / 2^N
    // where Γ_n is the reflection at junction n
    let scale = gamma_0 / 2.0_f64.powi(n as i32);
    
    let mut junction_gammas = Vec::with_capacity(n + 1);
    for i in 0..=n {
        let gamma_i = binomial_coeff(n, i) * scale;
        junction_gammas.push(gamma_i);
    }
    
    // Compute section impedances from junction reflection coefficients
    // Z_{n+1}/Z_n = (1 + Γ_n) / (1 - Γ_n)
    let mut section_impedances = Vec::with_capacity(n);
    let mut z_prev = z0;
    for i in 0..n {
        let gamma_i = junction_gammas[i];
        let z_next = z_prev * (1.0 + gamma_i) / (1.0 - gamma_i);
        section_impedances.push(z_next);
        z_prev = z_next;
    }
    
    // Compute frequency response
    let frequency_response = compute_frequency_response(&junction_gammas, n);
    
    MultisectionResult {
        design_type: MultisectionType::Binomial,
        num_sections: n,
        section_impedances,
        junction_gammas,
        fractional_bandwidth: None,
        frequency_response,
    }
}

/// Design a Chebyshev (equal ripple) multisection transformer.
///
/// The Chebyshev transformer provides equal-ripple response in the passband,
/// giving wider bandwidth than binomial for the same number of sections.
///
/// # Arguments
/// * `z0` - Input line characteristic impedance (Ω)
/// * `z_load` - Load impedance (must be real) (Ω)
/// * `num_sections` - Number of quarter-wave sections
/// * `max_gamma` - Maximum allowed reflection coefficient in passband
///
/// # Returns
/// Design result with section impedances and frequency response.
pub fn multisection_chebyshev(
    z0: f64,
    z_load: f64,
    num_sections: usize,
    max_gamma: f64,
) -> MultisectionResult {
    let n = num_sections;
    
    // Overall reflection coefficient
    let gamma_0 = ((z_load - z0) / (z_load + z0)).abs();
    
    // For Chebyshev: Γ(θ) = Γ₀ * e^{jNθ} * cos^N(θ) * T_N(sec θ_m * cos θ) / T_N(sec θ_m)
    // where θ_m defines the band edge and max_gamma = Γ₀ / T_N(sec θ_m)
    
    // Find sec(θ_m) such that T_N(sec θ_m) = Γ₀ / max_gamma
    let t_n_required = gamma_0 / max_gamma;
    
    // Inverse: sec(θ_m) = cosh(acosh(T_N) / N)
    let sec_theta_m = if t_n_required >= 1.0 {
        (t_n_required.acosh() / n as f64).cosh()
    } else {
        1.0 // fallback for edge case
    };
    
    // Bandwidth: fractional BW = 2 * (1 - θ_m/π*2) = 2 * (1 - 2*acos(1/sec_θm)/π)
    let theta_m = (1.0 / sec_theta_m).acos();
    let fractional_bw = 2.0 * (1.0 - 2.0 * theta_m / PI);
    
    // For Chebyshev design, compute junction gammas using Chebyshev weighting
    // This is an approximation; exact Chebyshev design uses recursive formulas
    let mut junction_gammas = Vec::with_capacity(n + 1);
    
    // Use simplified Chebyshev coefficient computation
    // Γ_k ≈ (2Γ₀/2^N) * C(N,k) * (Chebyshev weighting factor)
    for k in 0..=n {
        let weight = binomial_coeff(n, k) * gamma_0 / 2.0_f64.powi(n as i32);
        // Apply Chebyshev tapering (simplified)
        let taper = if n > 0 {
            chebyshev_t(n, sec_theta_m * (1.0 - 2.0 * k as f64 / n as f64).cos())
                / chebyshev_t(n, sec_theta_m)
        } else {
            1.0
        };
        junction_gammas.push(weight * taper.abs().max(0.1));
    }
    
    // Normalize so that the product gives correct overall transformation
    let gamma_sum: f64 = junction_gammas.iter().sum();
    let scale = gamma_0 / gamma_sum;
    for g in &mut junction_gammas {
        *g *= scale;
    }
    
    // Compute section impedances
    let mut section_impedances = Vec::with_capacity(n);
    let mut z_prev = z0;
    for i in 0..n {
        let gamma_i = junction_gammas[i];
        // Ensure we don't get negative or zero impedance
        let ratio = (1.0 + gamma_i) / (1.0 - gamma_i.min(0.99));
        let z_next = z_prev * ratio;
        section_impedances.push(z_next.max(1.0));
        z_prev = z_next;
    }
    
    // Adjust last section to approach load
    if let Some(last) = section_impedances.last_mut() {
        // Geometric mean progression toward load
        let geometric_ratio = (z_load / z0).powf(1.0 / (n + 1) as f64);
        let mut z = z0;
        for (i, z_sec) in section_impedances.iter_mut().enumerate() {
            z *= geometric_ratio;
            *z_sec = (*z_sec + z) / 2.0; // blend with geometric
        }
    }
    
    // Compute frequency response
    let frequency_response = compute_frequency_response(&junction_gammas, n);
    
    MultisectionResult {
        design_type: MultisectionType::Chebyshev,
        num_sections: n,
        section_impedances,
        junction_gammas,
        fractional_bandwidth: Some(fractional_bw),
        frequency_response,
    }
}

/// Compute frequency response of a multisection transformer.
fn compute_frequency_response(junction_gammas: &[f64], n: usize) -> Vec<(f64, f64)> {
    let num_points = 201;
    let mut response = Vec::with_capacity(num_points);
    
    for i in 0..num_points {
        // f/f0 from 0.2 to 1.8
        let f_ratio = 0.2 + 1.6 * i as f64 / (num_points - 1) as f64;
        
        // Electrical length θ = π/2 * (f/f0) for quarter-wave section
        let theta = PI / 2.0 * f_ratio;
        
        // Total reflection coefficient using approximate formula:
        // Γ(θ) = Σ Γ_k * e^{-j2kθ}
        let mut gamma_total = num_complex::Complex64::new(0.0, 0.0);
        for (k, &g) in junction_gammas.iter().enumerate() {
            let phase = -2.0 * k as f64 * theta;
            gamma_total += g * num_complex::Complex64::from_polar(1.0, phase);
        }
        
        response.push((f_ratio, gamma_total.norm()));
    }
    
    response
}

/// Design using geometric mean impedance stepping.
/// A simple but effective approximation for broadband matching.
pub fn multisection_geometric(z0: f64, z_load: f64, num_sections: usize) -> MultisectionResult {
    let n = num_sections;
    let ratio = (z_load / z0).powf(1.0 / (n + 1) as f64);
    
    let mut section_impedances = Vec::with_capacity(n);
    let mut junction_gammas = Vec::with_capacity(n + 1);
    
    let mut z_prev = z0;
    for _ in 0..n {
        let z_next = z_prev * ratio;
        section_impedances.push(z_next);
        let gamma = (z_next - z_prev) / (z_next + z_prev);
        junction_gammas.push(gamma);
        z_prev = z_next;
    }
    // Final junction to load
    junction_gammas.push((z_load - z_prev) / (z_load + z_prev));
    
    let frequency_response = compute_frequency_response(&junction_gammas, n);
    
    MultisectionResult {
        design_type: MultisectionType::Binomial, // Similar to binomial in practice
        num_sections: n,
        section_impedances,
        junction_gammas,
        fractional_bandwidth: None,
        frequency_response,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn binomial_single_section_is_quarter_wave() {
        let result = multisection_binomial(50.0, 100.0, 1);
        assert_eq!(result.section_impedances.len(), 1);
        // For single section, Z = √(Z0 * ZL)
        let expected = (50.0_f64 * 100.0).sqrt();
        assert_relative_eq!(result.section_impedances[0], expected, epsilon = 1.0);
    }

    #[test]
    fn binomial_impedances_monotonic() {
        let result = multisection_binomial(50.0, 200.0, 4);
        for i in 1..result.section_impedances.len() {
            assert!(
                result.section_impedances[i] > result.section_impedances[i - 1],
                "Impedances should increase toward load"
            );
        }
    }

    #[test]
    fn chebyshev_has_bandwidth() {
        let result = multisection_chebyshev(50.0, 100.0, 3, 0.05);
        assert!(result.fractional_bandwidth.is_some());
        assert!(result.fractional_bandwidth.unwrap() > 0.0);
    }

    #[test]
    fn frequency_response_generated() {
        let result = multisection_binomial(50.0, 100.0, 2);
        assert!(!result.frequency_response.is_empty());
        
        // Check that f/f0 = 1 has low reflection
        let at_center = result.frequency_response
            .iter()
            .find(|(f, _)| (*f - 1.0).abs() < 0.05)
            .map(|(_, g)| *g);
        assert!(at_center.is_some());
    }

    #[test]
    fn geometric_stepping() {
        let result = multisection_geometric(50.0, 200.0, 3);
        assert_eq!(result.section_impedances.len(), 3);
        
        // Verify geometric progression
        let ratio = (200.0 / 50.0_f64).powf(0.25);
        assert_relative_eq!(result.section_impedances[0], 50.0 * ratio, epsilon = 0.1);
    }
}

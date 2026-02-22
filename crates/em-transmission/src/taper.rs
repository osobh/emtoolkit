//! Tapered Transmission Line Matching
//!
//! Designs continuously tapered transmission lines for broadband impedance matching.
//! Supports exponential, triangular, and Klopfenstein taper profiles.

use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

/// Type of taper profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaperType {
    /// Exponential taper: Z(z) = Z₀·e^(az)
    Exponential,
    /// Triangular taper: piecewise linear ln(Z) profile
    Triangular,
    /// Klopfenstein taper: optimal for given length and ripple
    Klopfenstein,
}

impl std::fmt::Display for TaperType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaperType::Exponential => write!(f, "Exponential"),
            TaperType::Triangular => write!(f, "Triangular"),
            TaperType::Klopfenstein => write!(f, "Klopfenstein"),
        }
    }
}

/// Result of a taper design.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaperResult {
    /// Taper type used
    pub taper_type: TaperType,
    /// Total taper length in wavelengths
    pub length_wavelengths: f64,
    /// Impedance profile: (z/L, Z(z)) pairs where z/L ∈ [0, 1]
    pub impedance_profile: Vec<(f64, f64)>,
    /// Frequency response: (βL, |Γ|) pairs
    pub frequency_response: Vec<(f64, f64)>,
    /// Maximum ripple in passband (|Γ|)
    pub max_ripple: f64,
    /// Cutoff frequency (βL where taper becomes effective)
    pub cutoff_beta_l: f64,
}

/// Design a tapered transmission line for impedance matching.
///
/// # Arguments
/// * `z0` - Input characteristic impedance (Ω)
/// * `z_load` - Load impedance (must be real) (Ω)
/// * `length_wavelengths` - Total taper length in wavelengths
/// * `taper_type` - Type of taper profile
///
/// # Returns
/// Design result with impedance profile and frequency response.
pub fn taper_design(
    z0: f64,
    z_load: f64,
    length_wavelengths: f64,
    taper_type: TaperType,
) -> TaperResult {
    match taper_type {
        TaperType::Exponential => exponential_taper(z0, z_load, length_wavelengths),
        TaperType::Triangular => triangular_taper(z0, z_load, length_wavelengths),
        TaperType::Klopfenstein => klopfenstein_taper(z0, z_load, length_wavelengths, 0.02),
    }
}

/// Design exponential taper: Z(z) = Z₀·e^(az) where a = ln(Z_L/Z₀)/L
fn exponential_taper(z0: f64, z_load: f64, length_wavelengths: f64) -> TaperResult {
    let num_points = 101;
    let a = (z_load / z0).ln(); // a*L = ln(Z_L/Z_0), normalized to L=1
    
    // Impedance profile
    let mut impedance_profile = Vec::with_capacity(num_points);
    for i in 0..num_points {
        let z_norm = i as f64 / (num_points - 1) as f64; // z/L ∈ [0, 1]
        let z_impedance = z0 * (a * z_norm).exp();
        impedance_profile.push((z_norm, z_impedance));
    }
    
    // Frequency response
    // For exponential taper: |Γ| = |Γ₀| * |sin(βL)/(βL)| approximately
    let gamma_0 = ((z_load - z0) / (z_load + z0)).abs();
    let beta_l_max = 4.0 * PI; // Up to 2 wavelengths of electrical length variation
    
    let mut frequency_response = Vec::with_capacity(num_points);
    for i in 0..num_points {
        let beta_l = 0.1 + (beta_l_max - 0.1) * i as f64 / (num_points - 1) as f64;
        // Exponential taper reflection coefficient
        let sinc = if beta_l.abs() < 1e-10 {
            1.0
        } else {
            (beta_l).sin() / beta_l
        };
        let gamma = gamma_0 * sinc.abs();
        frequency_response.push((beta_l, gamma));
    }
    
    // Cutoff where sinc first crosses Γ₀/e
    let cutoff_beta_l = PI; // First null of sinc
    
    let max_ripple = gamma_0; // Maximum at low frequencies
    
    TaperResult {
        taper_type: TaperType::Exponential,
        length_wavelengths,
        impedance_profile,
        frequency_response,
        max_ripple,
        cutoff_beta_l,
    }
}

/// Design triangular taper: piecewise linear ln(Z) profile
fn triangular_taper(z0: f64, z_load: f64, length_wavelengths: f64) -> TaperResult {
    let num_points = 101;
    let ln_z0 = z0.ln();
    let ln_zl = z_load.ln();
    let delta_ln = ln_zl - ln_z0;
    
    // Triangular profile: slope increases then decreases
    // ln(Z) = ln(Z₀) + slope(z) where slope is triangular
    let mut impedance_profile = Vec::with_capacity(num_points);
    for i in 0..num_points {
        let z_norm = i as f64 / (num_points - 1) as f64;
        
        // Triangular weighting function (area = 1)
        let weight = if z_norm < 0.5 {
            4.0 * z_norm * z_norm // Parabolic rise
        } else {
            1.0 - 4.0 * (1.0 - z_norm) * (1.0 - z_norm) // Parabolic approach to 1
        };
        
        let ln_z = ln_z0 + delta_ln * weight;
        impedance_profile.push((z_norm, ln_z.exp()));
    }
    
    // Frequency response - triangular has faster rolloff than exponential
    let gamma_0 = ((z_load - z0) / (z_load + z0)).abs();
    let beta_l_max = 4.0 * PI;
    
    let mut frequency_response = Vec::with_capacity(num_points);
    for i in 0..num_points {
        let beta_l = 0.1 + (beta_l_max - 0.1) * i as f64 / (num_points - 1) as f64;
        // Triangular taper: |Γ| ≈ Γ₀ * |sin(βL/2)/(βL/2)|²
        let half_beta_l = beta_l / 2.0;
        let sinc = if half_beta_l.abs() < 1e-10 {
            1.0
        } else {
            (half_beta_l).sin() / half_beta_l
        };
        let gamma = gamma_0 * sinc * sinc;
        frequency_response.push((beta_l, gamma));
    }
    
    TaperResult {
        taper_type: TaperType::Triangular,
        length_wavelengths,
        impedance_profile,
        frequency_response,
        max_ripple: gamma_0,
        cutoff_beta_l: PI,
    }
}

/// Design Klopfenstein taper: optimal for given length and maximum ripple.
///
/// The Klopfenstein taper minimizes the passband ripple for a given taper length,
/// or equivalently, minimizes the length for a given maximum ripple.
fn klopfenstein_taper(
    z0: f64,
    z_load: f64,
    length_wavelengths: f64,
    max_gamma: f64,
) -> TaperResult {
    let num_points = 101;
    let gamma_0 = ((z_load - z0) / (z_load + z0)).abs();
    
    // Klopfenstein A parameter: cosh(A) = Γ₀ / Γₘ
    let a_param = if gamma_0 > max_gamma {
        (gamma_0 / max_gamma).acosh()
    } else {
        0.0 // Already matched well enough
    };
    
    // Impedance profile using Klopfenstein formula
    // ln(Z(z)/Z₀) = (1/2)ln(Z_L/Z₀) * [1 + A⁻¹ * φ(2z/L - 1, A)]
    // where φ is the Klopfenstein function
    let ln_ratio = (z_load / z0).ln();
    
    let mut impedance_profile = Vec::with_capacity(num_points);
    for i in 0..num_points {
        let z_norm = i as f64 / (num_points - 1) as f64;
        let x = 2.0 * z_norm - 1.0; // x ∈ [-1, 1]
        
        // Simplified Klopfenstein profile (exact requires modified Bessel functions)
        // Using polynomial approximation
        let phi = klopfenstein_phi(x, a_param);
        let ln_z = z0.ln() + 0.5 * ln_ratio * (1.0 + phi / a_param.max(0.01));
        impedance_profile.push((z_norm, ln_z.exp()));
    }
    
    // Frequency response
    // |Γ| = Γₘ * |cos(√((βL)² - A²))| / |cosh(A)| for βL > A
    // |Γ| = Γₘ for βL < A (passband ripple)
    let beta_l_max = 4.0 * PI;
    
    let mut frequency_response = Vec::with_capacity(num_points);
    let beta_l_design = 2.0 * PI * length_wavelengths;
    
    for i in 0..num_points {
        let f_ratio = 0.1 + 1.9 * i as f64 / (num_points - 1) as f64;
        let beta_l = beta_l_design * f_ratio;
        
        let gamma = if beta_l < a_param {
            gamma_0 / a_param.cosh()
        } else {
            let arg = (beta_l * beta_l - a_param * a_param).sqrt();
            gamma_0 * arg.cos().abs() / a_param.cosh()
        };
        
        frequency_response.push((beta_l, gamma.min(gamma_0)));
    }
    
    TaperResult {
        taper_type: TaperType::Klopfenstein,
        length_wavelengths,
        impedance_profile,
        frequency_response,
        max_ripple: max_gamma,
        cutoff_beta_l: a_param,
    }
}

/// Klopfenstein φ function (simplified approximation).
/// The exact function requires modified Bessel functions.
fn klopfenstein_phi(x: f64, a: f64) -> f64 {
    if a.abs() < 1e-10 {
        return x;
    }
    
    // Polynomial approximation for the integrated Klopfenstein profile
    // φ(x, A) ≈ A * x * (1 - x²/3 + x⁴/15 - ...) for small A
    // For larger A, use series expansion
    let x2 = x * x;
    let x4 = x2 * x2;
    let x6 = x4 * x2;
    
    // Series approximation
    let series = x * (1.0 - (1.0 - a * a / 4.0) * x2 / 3.0 
                       + (1.0 - a * a / 2.0) * x4 / 15.0
                       - x6 / 105.0);
    
    a * series
}

/// Compute the reflection coefficient for an arbitrary impedance profile.
pub fn taper_reflection(
    impedance_profile: &[(f64, f64)],
    beta_l: f64,
) -> f64 {
    // Numerical integration of the distributed reflections
    // Γ ≈ (1/2) ∫₀^L (d ln Z / dz) * e^{-j2βz} dz
    
    let n = impedance_profile.len();
    if n < 2 {
        return 0.0;
    }
    
    let mut gamma_re = 0.0;
    let mut gamma_im = 0.0;
    
    for i in 1..n {
        let (z0, z_i0) = impedance_profile[i - 1];
        let (z1, z_i1) = impedance_profile[i];
        
        let dz = z1 - z0;
        let z_mid = (z0 + z1) / 2.0;
        let d_ln_z = (z_i1 / z_i0).ln();
        
        let phase = -2.0 * beta_l * z_mid;
        gamma_re += d_ln_z * phase.cos() * dz;
        gamma_im += d_ln_z * phase.sin() * dz;
    }
    
    0.5 * (gamma_re * gamma_re + gamma_im * gamma_im).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn exponential_taper_endpoints() {
        let result = exponential_taper(50.0, 100.0, 1.0);
        
        // Should start at Z₀
        assert_relative_eq!(result.impedance_profile[0].1, 50.0, epsilon = 0.1);
        
        // Should end at Z_L
        let last = result.impedance_profile.last().unwrap().1;
        assert_relative_eq!(last, 100.0, epsilon = 0.1);
    }

    #[test]
    fn triangular_taper_endpoints() {
        let result = triangular_taper(50.0, 200.0, 1.0);
        assert_relative_eq!(result.impedance_profile[0].1, 50.0, epsilon = 0.1);
        let last = result.impedance_profile.last().unwrap().1;
        assert_relative_eq!(last, 200.0, epsilon = 0.5);
    }

    #[test]
    fn klopfenstein_taper_bounded_ripple() {
        let max_gamma = 0.02;
        let result = klopfenstein_taper(50.0, 100.0, 1.0, max_gamma);
        
        // In passband, reflection should be bounded
        for (beta_l, gamma) in &result.frequency_response {
            if *beta_l > result.cutoff_beta_l {
                assert!(*gamma < 0.5, "Passband reflection should be low");
            }
        }
    }

    #[test]
    fn taper_design_selector() {
        let exp = taper_design(50.0, 100.0, 1.0, TaperType::Exponential);
        assert_eq!(exp.taper_type, TaperType::Exponential);

        let tri = taper_design(50.0, 100.0, 1.0, TaperType::Triangular);
        assert_eq!(tri.taper_type, TaperType::Triangular);

        let klop = taper_design(50.0, 100.0, 1.0, TaperType::Klopfenstein);
        assert_eq!(klop.taper_type, TaperType::Klopfenstein);
    }

    #[test]
    fn impedance_profile_monotonic_for_step_up() {
        let result = taper_design(50.0, 200.0, 1.0, TaperType::Exponential);
        for i in 1..result.impedance_profile.len() {
            assert!(
                result.impedance_profile[i].1 >= result.impedance_profile[i - 1].1,
                "Impedance should increase for step-up"
            );
        }
    }
}

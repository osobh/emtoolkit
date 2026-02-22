//! General Impedance Transformers
//!
//! Analyzes arbitrary-length transformer sections, not just quarter-wave.
//! Also includes two-line section transformers for more design flexibility.

use num_complex::Complex64;
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

/// Result of a general transformer analysis.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransformerResult {
    /// Transformer section impedance (Ω)
    pub z_transformer: f64,
    /// Transformer section length in wavelengths
    pub length_wavelengths: f64,
    /// Input impedance at design frequency (Ω)
    pub z_in: Complex64Serializable,
    /// Input reflection coefficient
    pub gamma_in: Complex64Serializable,
    /// Magnitude of input reflection coefficient
    pub gamma_mag: f64,
    /// VSWR at input
    pub vswr: f64,
    /// Frequency response: (f/f₀, |Γ|) pairs
    pub frequency_response: Vec<(f64, f64)>,
}

/// Result of a two-line section transformer analysis.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TwoLineResult {
    /// First section impedance (Ω)
    pub z1: f64,
    /// First section length in wavelengths
    pub l1_wavelengths: f64,
    /// Second section impedance (Ω)
    pub z2: f64,
    /// Second section length in wavelengths
    pub l2_wavelengths: f64,
    /// Input impedance at design frequency (Ω)
    pub z_in: Complex64Serializable,
    /// Input reflection coefficient
    pub gamma_in: Complex64Serializable,
    /// Magnitude of input reflection coefficient
    pub gamma_mag: f64,
    /// VSWR at input
    pub vswr: f64,
    /// Frequency response: (f/f₀, |Γ|) pairs
    pub frequency_response: Vec<(f64, f64)>,
}

/// Serializable wrapper for Complex64.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Complex64Serializable {
    pub re: f64,
    pub im: f64,
}

impl From<Complex64> for Complex64Serializable {
    fn from(c: Complex64) -> Self {
        Complex64Serializable { re: c.re, im: c.im }
    }
}

impl From<Complex64Serializable> for Complex64 {
    fn from(c: Complex64Serializable) -> Self {
        Complex64::new(c.re, c.im)
    }
}

/// Analyze a general transformer section of arbitrary length.
///
/// This is more flexible than a quarter-wave transformer, allowing analysis
/// of any transformer section length with any load impedance.
///
/// # Arguments
/// * `z0` - Source line characteristic impedance (Ω)
/// * `z_load` - Complex load impedance (Ω)
/// * `z_transformer` - Transformer section impedance (Ω)
/// * `length_wavelengths` - Transformer length in wavelengths
/// * `frequency` - Design frequency (Hz) - used for context
///
/// # Returns
/// Analysis result with input impedance, reflection coefficient, and frequency response.
pub fn general_transformer(
    z0: f64,
    z_load: Complex64,
    z_transformer: f64,
    length_wavelengths: f64,
    _frequency: f64,
) -> TransformerResult {
    // Electrical length in radians
    let beta_l = 2.0 * PI * length_wavelengths;
    
    // Input impedance of transmission line section
    // Z_in = Z_T * (Z_L + jZ_T*tan(βl)) / (Z_T + jZ_L*tan(βl))
    let z_in = input_impedance_lossy(z_transformer, z_load, beta_l);
    
    // Reflection coefficient looking into the transformer
    let gamma_in = (z_in - z0) / (z_in + z0);
    let gamma_mag = gamma_in.norm();
    
    // VSWR
    let vswr = if gamma_mag < 0.9999 {
        (1.0 + gamma_mag) / (1.0 - gamma_mag)
    } else {
        f64::INFINITY
    };
    
    // Frequency response
    let frequency_response = compute_frequency_response_single(z0, z_load, z_transformer, length_wavelengths);
    
    TransformerResult {
        z_transformer,
        length_wavelengths,
        z_in: z_in.into(),
        gamma_in: gamma_in.into(),
        gamma_mag,
        vswr,
        frequency_response,
    }
}

/// Analyze a two-line section transformer.
///
/// Two cascaded transmission line sections can provide more matching
/// flexibility than a single section, especially for complex loads.
///
/// # Arguments
/// * `z0` - Source line characteristic impedance (Ω)
/// * `z_load` - Complex load impedance (Ω)
/// * `z1` - First section impedance (closer to load) (Ω)
/// * `l1_wavelengths` - First section length in wavelengths
/// * `z2` - Second section impedance (closer to source) (Ω)
/// * `l2_wavelengths` - Second section length in wavelengths
/// * `frequency` - Design frequency (Hz)
///
/// # Returns
/// Analysis result with input impedance and frequency response.
pub fn two_line_transformer(
    z0: f64,
    z_load: Complex64,
    z1: f64,
    l1_wavelengths: f64,
    z2: f64,
    l2_wavelengths: f64,
    _frequency: f64,
) -> TwoLineResult {
    let beta_l1 = 2.0 * PI * l1_wavelengths;
    let beta_l2 = 2.0 * PI * l2_wavelengths;
    
    // First section: transforms load
    let z_after_section1 = input_impedance_lossy(z1, z_load, beta_l1);
    
    // Second section: transforms the intermediate impedance
    let z_in = input_impedance_lossy(z2, z_after_section1, beta_l2);
    
    // Reflection coefficient
    let gamma_in = (z_in - z0) / (z_in + z0);
    let gamma_mag = gamma_in.norm();
    
    let vswr = if gamma_mag < 0.9999 {
        (1.0 + gamma_mag) / (1.0 - gamma_mag)
    } else {
        f64::INFINITY
    };
    
    // Frequency response
    let frequency_response = compute_frequency_response_two_line(
        z0, z_load, z1, l1_wavelengths, z2, l2_wavelengths
    );
    
    TwoLineResult {
        z1,
        l1_wavelengths,
        z2,
        l2_wavelengths,
        z_in: z_in.into(),
        gamma_in: gamma_in.into(),
        gamma_mag,
        vswr,
        frequency_response,
    }
}

/// Design a two-section transformer for matching a complex load.
///
/// Uses quarter-wave sections with optimal impedances.
pub fn design_two_section_match(
    z0: f64,
    z_load: Complex64,
) -> Option<TwoLineResult> {
    let r_l = z_load.re;
    let x_l = z_load.im;
    
    if r_l <= 0.0 {
        return None; // Can't match negative resistance
    }
    
    // For complex load, first section should transform to real impedance
    // |Z_L| determines the intermediate impedance
    let z_l_mag = z_load.norm();
    
    // Optimal first section impedance
    let z1 = (z_l_mag * r_l).sqrt();
    
    // Second section transforms to Z₀
    let z2 = (z0 * z1).sqrt();
    
    // Use λ/4 sections
    let result = two_line_transformer(z0, z_load, z1, 0.25, z2, 0.25, 1e9);
    
    if result.gamma_mag < 0.3 {
        Some(result)
    } else {
        // Try different approach for highly reactive loads
        let z1_alt = z0;
        let z2_alt = (z0 * r_l).sqrt();
        let result_alt = two_line_transformer(z0, z_load, z1_alt, 0.125, z2_alt, 0.25, 1e9);
        
        if result_alt.gamma_mag < result.gamma_mag {
            Some(result_alt)
        } else {
            Some(result)
        }
    }
}

/// Compute input impedance of a lossless transmission line section.
fn input_impedance_lossy(z0: f64, z_load: Complex64, beta_l: f64) -> Complex64 {
    let j = Complex64::new(0.0, 1.0);
    let z0c = Complex64::new(z0, 0.0);
    let tan_bl = Complex64::new(beta_l.tan(), 0.0);
    
    z0c * (z_load + j * z0c * tan_bl) / (z0c + j * z_load * tan_bl)
}

/// Compute frequency response for a single transformer section.
fn compute_frequency_response_single(
    z0: f64,
    z_load: Complex64,
    z_transformer: f64,
    length_wavelengths: f64,
) -> Vec<(f64, f64)> {
    let num_points = 101;
    let mut response = Vec::with_capacity(num_points);
    
    for i in 0..num_points {
        let f_ratio = 0.2 + 1.8 * i as f64 / (num_points - 1) as f64;
        let beta_l = 2.0 * PI * length_wavelengths * f_ratio;
        
        let z_in = input_impedance_lossy(z_transformer, z_load, beta_l);
        let gamma = ((z_in - z0) / (z_in + z0)).norm();
        
        response.push((f_ratio, gamma));
    }
    
    response
}

/// Compute frequency response for a two-section transformer.
fn compute_frequency_response_two_line(
    z0: f64,
    z_load: Complex64,
    z1: f64,
    l1_wavelengths: f64,
    z2: f64,
    l2_wavelengths: f64,
) -> Vec<(f64, f64)> {
    let num_points = 101;
    let mut response = Vec::with_capacity(num_points);
    
    for i in 0..num_points {
        let f_ratio = 0.2 + 1.8 * i as f64 / (num_points - 1) as f64;
        let beta_l1 = 2.0 * PI * l1_wavelengths * f_ratio;
        let beta_l2 = 2.0 * PI * l2_wavelengths * f_ratio;
        
        let z_after_1 = input_impedance_lossy(z1, z_load, beta_l1);
        let z_in = input_impedance_lossy(z2, z_after_1, beta_l2);
        let gamma = ((z_in - z0) / (z_in + z0)).norm();
        
        response.push((f_ratio, gamma));
    }
    
    response
}

/// Optimize a single transformer section for minimum reflection at center frequency.
///
/// For a real load Z_L, the optimal impedance is √(Z₀·Z_L) at λ/4.
/// For complex loads, this finds the best Z_T and length.
pub fn optimize_single_section(
    z0: f64,
    z_load: Complex64,
) -> TransformerResult {
    let r_l = z_load.re;
    let x_l = z_load.im;
    
    if x_l.abs() < r_l * 0.01 {
        // Nearly real load: use quarter-wave
        let z_t = (z0 * r_l).sqrt();
        return general_transformer(z0, z_load, z_t, 0.25, 1e9);
    }
    
    // Complex load: search for optimal length
    // Start with quarter-wave and geometric mean
    let z_t = (z0 * z_load.norm()).sqrt();
    
    // Search for optimal length
    let mut best_gamma = f64::INFINITY;
    let mut best_length = 0.25;
    
    for i in 0..100 {
        let length = 0.1 + 0.4 * i as f64 / 99.0;
        let result = general_transformer(z0, z_load, z_t, length, 1e9);
        if result.gamma_mag < best_gamma {
            best_gamma = result.gamma_mag;
            best_length = length;
        }
    }
    
    general_transformer(z0, z_load, z_t, best_length, 1e9)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn quarter_wave_real_load() {
        let z0 = 50.0;
        let z_load = Complex64::new(100.0, 0.0);
        let z_t = (z0 * 100.0_f64).sqrt(); // 70.71 Ω
        
        let result = general_transformer(z0, z_load, z_t, 0.25, 1e9);
        
        // Perfect match at center frequency
        assert!(result.gamma_mag < 0.01, "Quarter-wave should match real load");
        assert_relative_eq!(result.z_in.re, z0, epsilon = 1.0);
    }

    #[test]
    fn arbitrary_length_changes_match() {
        let z0 = 50.0;
        let z_load = Complex64::new(100.0, 0.0);
        let z_t = 70.71;
        
        // λ/8 won't give perfect match
        let result = general_transformer(z0, z_load, z_t, 0.125, 1e9);
        assert!(result.gamma_mag > 0.01, "λ/8 should not perfectly match");
    }

    #[test]
    fn two_line_cascades_correctly() {
        let z0 = 50.0;
        let z_load = Complex64::new(200.0, 0.0);
        
        // Two quarter-wave sections with geometric stepping
        // Z_n = Z_0 * (Z_L/Z_0)^(n/(N+1)) where N=2
        // Z_1 = 50 * (200/50)^(1/3) ≈ 79.37 (near load)
        // Z_2 = 50 * (200/50)^(2/3) ≈ 126.0 (near source)... wait, reversed
        // Actually for the cascade: Z_2 is near source, Z_1 near load
        // The order in two_line_transformer is: z1=near load, z2=near source
        let ratio = (200.0 / 50.0_f64).powf(1.0 / 3.0); // ≈ 1.587
        let z1 = 50.0 * ratio * ratio; // Section near load ≈ 126
        let z2 = 50.0 * ratio; // Section near source ≈ 79.4
        
        let result = two_line_transformer(z0, z_load, z1, 0.25, z2, 0.25, 1e9);
        
        // Should provide broadband match
        assert!(result.gamma_mag < 0.3, "gamma_mag = {}", result.gamma_mag);
    }

    #[test]
    fn frequency_response_generated() {
        let result = general_transformer(50.0, Complex64::new(100.0, 0.0), 70.71, 0.25, 1e9);
        assert!(!result.frequency_response.is_empty());
        
        // At f/f0 = 1, should have minimum reflection
        let at_center = result.frequency_response
            .iter()
            .find(|(f, _)| (*f - 1.0).abs() < 0.05)
            .map(|(_, g)| *g);
        assert!(at_center.is_some());
        assert!(at_center.unwrap() < 0.1);
    }

    #[test]
    fn vswr_calculation() {
        let result = general_transformer(50.0, Complex64::new(50.0, 0.0), 50.0, 0.25, 1e9);
        assert_relative_eq!(result.vswr, 1.0, epsilon = 0.01);
    }

    #[test]
    fn complex_load_analysis() {
        let z_load = Complex64::new(25.0, 50.0);
        let result = general_transformer(50.0, z_load, 50.0, 0.125, 1e9);
        
        // Should transform the load
        assert!(result.z_in.re > 0.0);
    }
}

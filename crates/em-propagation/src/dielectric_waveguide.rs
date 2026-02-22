//! Dielectric slab waveguide analysis.
//!
//! Symmetric dielectric slab waveguide: core (higher εᵣ) sandwiched by cladding.
//! Solves transcendental equations numerically to find guided modes.

use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

/// Speed of light in vacuum.
const C: f64 = 2.99792458e8;

/// Information about a guided mode in a dielectric slab.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DielectricMode {
    /// Mode number (0, 1, 2, ...)
    pub mode_number: usize,
    /// Mode type: "TE" or "TM"
    pub mode_type: String,
    /// Propagation constant β (rad/m)
    pub beta: f64,
    /// Effective refractive index n_eff = β/k₀
    pub effective_index: f64,
    /// Transverse decay constant in cladding γ (1/m)
    pub gamma: f64,
    /// Transverse wavenumber in core κ (rad/m)
    pub kappa: f64,
    /// Confinement factor (fraction of power in core)
    pub confinement: f64,
}

/// Results from dielectric slab analysis.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DielectricSlabResult {
    /// V-number (normalized frequency)
    pub v_number: f64,
    /// Estimated number of modes
    pub num_modes_estimate: usize,
    /// Core refractive index
    pub n_core: f64,
    /// Cladding refractive index
    pub n_clad: f64,
    /// Numerical aperture NA = √(n₁² - n₂²)
    pub numerical_aperture: f64,
    /// Found modes
    pub modes: Vec<DielectricMode>,
}

/// Symmetric dielectric slab waveguide.
///
/// Geometry: slab of thickness d with refractive index n_core,
/// surrounded by cladding with n_clad (n_core > n_clad).
pub struct DielectricSlab {
    /// Slab thickness (m)
    pub d: f64,
    /// Core permittivity
    pub epsilon_core: f64,
    /// Cladding permittivity
    pub epsilon_clad: f64,
}

impl DielectricSlab {
    pub fn new(d: f64, epsilon_core: f64, epsilon_clad: f64) -> Self {
        assert!(
            epsilon_core > epsilon_clad,
            "Core permittivity must be greater than cladding for guidance"
        );
        Self {
            d,
            epsilon_core,
            epsilon_clad,
        }
    }

    /// Core refractive index.
    pub fn n_core(&self) -> f64 {
        self.epsilon_core.sqrt()
    }

    /// Cladding refractive index.
    pub fn n_clad(&self) -> f64 {
        self.epsilon_clad.sqrt()
    }

    /// Numerical aperture NA = √(n₁² - n₂²).
    pub fn numerical_aperture(&self) -> f64 {
        (self.epsilon_core - self.epsilon_clad).sqrt()
    }

    /// V-number (normalized frequency).
    ///
    /// V = (πd/λ₀) · √(n₁² - n₂²) = (πd/λ₀) · NA
    pub fn v_number(&self, frequency: f64) -> f64 {
        let lambda0 = C / frequency;
        PI * self.d / lambda0 * self.numerical_aperture()
    }

    /// Estimated number of guided modes (single polarization).
    ///
    /// Number of TE (or TM) modes ≈ floor(2V/π) + 1
    pub fn num_modes_estimate(&self, frequency: f64) -> usize {
        let v = self.v_number(frequency);
        (2.0 * v / PI).floor() as usize + 1
    }

    /// Find all TE modes at given frequency.
    pub fn find_te_modes(&self, frequency: f64, max_modes: usize) -> Vec<DielectricMode> {
        self.find_modes(frequency, max_modes, "TE")
    }

    /// Find all TM modes at given frequency.
    pub fn find_tm_modes(&self, frequency: f64, max_modes: usize) -> Vec<DielectricMode> {
        self.find_modes(frequency, max_modes, "TM")
    }

    /// Find all guided modes (TE and TM) at given frequency.
    pub fn find_all_modes(&self, frequency: f64, max_modes: usize) -> Vec<DielectricMode> {
        let mut modes = self.find_te_modes(frequency, max_modes);
        modes.extend(self.find_tm_modes(frequency, max_modes));
        modes.sort_by(|a, b| b.effective_index.partial_cmp(&a.effective_index).unwrap());
        modes
    }

    fn find_modes(&self, frequency: f64, max_modes: usize, mode_type: &str) -> Vec<DielectricMode> {
        let omega = 2.0 * PI * frequency;
        let k0 = omega / C;
        let n1 = self.n_core();

        // β ranges from k0·n_clad to k0·n_core for guided modes
        // (used implicitly in V-number approach)

        // Use normalized variable u = κd/2, where κ² = k₀²n₁² - β²
        // u ranges from 0 to V
        let v = self.v_number(frequency);
        if v <= 0.0 {
            return Vec::new();
        }

        let mut modes = Vec::new();

        // Dispersion equation for symmetric slab:
        // Even TE modes: tan(κd/2) = γ/κ
        // Odd TE modes:  -cot(κd/2) = γ/κ (or tan(κd/2) = -κ/γ)
        // where κ² = k₀²n₁² - β², γ² = β² - k₀²n₂²

        // For TM modes, include the εr ratio:
        // Even TM: tan(κd/2) = (ε_core/ε_clad) · γ/κ
        // Odd TM:  -cot(κd/2) = (ε_core/ε_clad) · γ/κ

        let eps_ratio = if mode_type == "TM" {
            self.epsilon_core / self.epsilon_clad
        } else {
            1.0
        };

        // Search for roots using bisection
        for mode_num in 0..max_modes {
            let is_even = mode_num % 2 == 0;

            // Define the characteristic equation
            let char_eq = |u: f64| -> f64 {
                if u <= 0.0 || u >= v {
                    return f64::NAN;
                }
                let w_sq = v * v - u * u;
                if w_sq <= 0.0 {
                    return f64::NAN;
                }
                let w = w_sq.sqrt();
                // u = κd/2, w = γd/2

                if is_even {
                    // tan(u) - eps_ratio * w/u = 0
                    u.tan() - eps_ratio * w / u
                } else {
                    // -1/tan(u) - eps_ratio * w/u = 0
                    -1.0 / u.tan() - eps_ratio * w / u
                }
            };

            // Determine search interval for this mode
            // For mode m: u is approximately in [(m)π/2, (m+1)π/2]
            let u_start = if is_even {
                (mode_num as f64 / 2.0) * PI
            } else {
                ((mode_num as f64 - 1.0) / 2.0 + 0.5) * PI
            };
            let u_end = u_start + PI / 2.0;

            // Search for root in interval
            let search_start = u_start.max(1e-10);
            let search_end = u_end.min(v - 1e-10);

            if search_start >= search_end {
                continue;
            }

            // Find sign change
            if let Some(u_root) = find_root(char_eq, search_start, search_end, 1e-10) {
                let w = (v * v - u_root * u_root).sqrt();
                let kappa = 2.0 * u_root / self.d;
                let gamma = 2.0 * w / self.d;
                let beta = (k0 * k0 * n1 * n1 - kappa * kappa).sqrt();
                let n_eff = beta / k0;

                // Confinement factor approximation
                let confinement = 1.0 - gamma * self.d / (2.0 * (1.0 + gamma * self.d / 2.0));

                modes.push(DielectricMode {
                    mode_number: mode_num,
                    mode_type: mode_type.to_string(),
                    beta,
                    effective_index: n_eff,
                    gamma,
                    kappa,
                    confinement: confinement.max(0.0).min(1.0),
                });
            }
        }

        modes
    }
}

/// Find dielectric slab modes (convenience function).
pub fn dielectric_slab_modes(
    d: f64,
    epsilon_core: f64,
    epsilon_clad: f64,
    frequency: f64,
    max_modes: usize,
) -> DielectricSlabResult {
    let slab = DielectricSlab::new(d, epsilon_core, epsilon_clad);
    let modes = slab.find_all_modes(frequency, max_modes);

    DielectricSlabResult {
        v_number: slab.v_number(frequency),
        num_modes_estimate: slab.num_modes_estimate(frequency),
        n_core: slab.n_core(),
        n_clad: slab.n_clad(),
        numerical_aperture: slab.numerical_aperture(),
        modes,
    }
}

/// Bisection root finder.
fn find_root<F: Fn(f64) -> f64>(f: F, mut a: f64, mut b: f64, tol: f64) -> Option<f64> {
    let mut fa = f(a);
    let fb = f(b);

    if fa.is_nan() || fb.is_nan() {
        // Try sampling to find a valid interval
        let n = 100;
        let step = (b - a) / n as f64;
        for i in 0..n {
            let x1 = a + i as f64 * step;
            let x2 = x1 + step;
            let f1 = f(x1);
            let f2 = f(x2);
            if !f1.is_nan() && !f2.is_nan() && f1 * f2 < 0.0 {
                a = x1;
                fa = f1;
                b = x2;
                break;
            }
        }
        if f(a).is_nan() || f(b).is_nan() {
            return None;
        }
    }

    if fa * f(b) > 0.0 {
        return None;
    }

    for _ in 0..100 {
        let mid = (a + b) / 2.0;
        let fmid = f(mid);

        if fmid.is_nan() {
            b = mid;
            continue;
        }

        if fmid.abs() < tol {
            return Some(mid);
        }

        if fa * fmid < 0.0 {
            b = mid;
        } else {
            a = mid;
            fa = fmid;
        }

        if (b - a) < tol {
            return Some(mid);
        }
    }

    Some((a + b) / 2.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_v_number() {
        // 1μm slab, n1=1.5, n2=1.45, λ=1.55μm
        let slab = DielectricSlab::new(1e-6, 1.5 * 1.5, 1.45 * 1.45);
        let freq = C / 1.55e-6;
        let v = slab.v_number(freq);
        // V = π·1μm / 1.55μm · √(2.25 - 2.1025) ≈ 0.78
        assert!(v > 0.5 && v < 1.0);
    }

    #[test]
    fn test_single_mode_condition() {
        // Single-mode when V < π/2 ≈ 1.57
        let slab = DielectricSlab::new(1e-6, 1.5 * 1.5, 1.45 * 1.45);
        let freq = C / 1.55e-6;
        let v = slab.v_number(freq);
        assert!(v < PI / 2.0);
        let modes = slab.find_te_modes(freq, 5);
        assert_eq!(modes.len(), 1);
    }

    #[test]
    fn test_effective_index_bounds() {
        let slab = DielectricSlab::new(5e-6, 2.25, 2.0);
        let freq = C / 1.55e-6;
        let modes = slab.find_all_modes(freq, 10);

        for mode in &modes {
            // n_clad < n_eff < n_core
            assert!(mode.effective_index > slab.n_clad());
            assert!(mode.effective_index < slab.n_core());
        }
    }

    #[test]
    fn test_multimode() {
        // Large V-number should support multiple modes
        let slab = DielectricSlab::new(10e-6, 2.25, 2.0);
        let freq = C / 1.55e-6;
        let v = slab.v_number(freq);
        assert!(v > PI); // Should support multiple modes

        let modes = slab.find_te_modes(freq, 10);
        assert!(modes.len() >= 2);
    }

    #[test]
    fn test_numerical_aperture() {
        let slab = DielectricSlab::new(1e-6, 2.25, 2.0);
        let na = slab.numerical_aperture();
        assert_relative_eq!(na, 0.5, epsilon = 0.001);
    }
}

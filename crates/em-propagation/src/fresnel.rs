//! Fresnel coefficients and Snell's law for wave reflection/transmission.
//!
//! Handles normal and oblique incidence at planar boundaries between
//! lossless dielectric media.

use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

/// Normal incidence reflection and transmission coefficients.
///
/// Γ = (η₂ - η₁)/(η₂ + η₁)
/// τ = 2η₂/(η₂ + η₁)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct NormalIncidence {
    /// Intrinsic impedance of medium 1 (Ω)
    pub eta1: f64,
    /// Intrinsic impedance of medium 2 (Ω)
    pub eta2: f64,
}

impl NormalIncidence {
    pub fn new(eta1: f64, eta2: f64) -> Self {
        Self { eta1, eta2 }
    }

    /// From relative permittivities (lossless, μᵣ=1).
    /// η = η₀/√εᵣ
    pub fn from_epsilon_r(er1: f64, er2: f64) -> Self {
        let eta0 = 377.0; // approximate
        Self {
            eta1: eta0 / er1.sqrt(),
            eta2: eta0 / er2.sqrt(),
        }
    }

    /// Reflection coefficient Γ.
    pub fn gamma(&self) -> f64 {
        (self.eta2 - self.eta1) / (self.eta2 + self.eta1)
    }

    /// Transmission coefficient τ.
    pub fn tau(&self) -> f64 {
        2.0 * self.eta2 / (self.eta2 + self.eta1)
    }

    /// Power reflectance |Γ|².
    pub fn reflectance(&self) -> f64 {
        self.gamma() * self.gamma()
    }

    /// Power transmittance 1 - |Γ|².
    pub fn transmittance(&self) -> f64 {
        1.0 - self.reflectance()
    }
}

/// Oblique incidence at a planar boundary between two lossless dielectrics.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ObliqueIncidence {
    /// Relative permittivity of medium 1
    pub er1: f64,
    /// Relative permittivity of medium 2
    pub er2: f64,
    /// Angle of incidence (radians)
    pub theta_i: f64,
}

impl ObliqueIncidence {
    pub fn new(er1: f64, er2: f64, theta_i: f64) -> Self {
        Self { er1, er2, theta_i }
    }

    /// Refractive indices.
    pub fn n1(&self) -> f64 {
        self.er1.sqrt()
    }

    pub fn n2(&self) -> f64 {
        self.er2.sqrt()
    }

    /// Snell's law: sin(θ_t) = (n₁/n₂)·sin(θ_i)
    ///
    /// Returns None if total internal reflection occurs.
    pub fn theta_t(&self) -> Option<f64> {
        let sin_t = self.n1() / self.n2() * self.theta_i.sin();
        if sin_t.abs() > 1.0 {
            None // TIR
        } else {
            Some(sin_t.asin())
        }
    }

    /// Critical angle for total internal reflection.
    ///
    /// θ_c = arcsin(n₂/n₁), only exists when n₁ > n₂.
    pub fn critical_angle(&self) -> Option<f64> {
        if self.n1() > self.n2() {
            Some((self.n2() / self.n1()).asin())
        } else {
            None
        }
    }

    /// Brewster angle (for parallel/TM polarization).
    ///
    /// θ_B = arctan(n₂/n₁)
    pub fn brewster_angle(&self) -> f64 {
        (self.n2() / self.n1()).atan()
    }

    /// Is total internal reflection occurring?
    pub fn is_tir(&self) -> bool {
        self.theta_t().is_none()
    }

    /// Perpendicular (TE/s) polarization reflection coefficient.
    ///
    /// Γ_⊥ = (η₂cosθᵢ - η₁cosθₜ)/(η₂cosθᵢ + η₁cosθₜ)
    pub fn gamma_perp(&self) -> Option<f64> {
        let theta_t = self.theta_t()?;
        let eta1 = 1.0 / self.n1(); // proportional
        let eta2 = 1.0 / self.n2();
        let num = eta2 * self.theta_i.cos() - eta1 * theta_t.cos();
        let den = eta2 * self.theta_i.cos() + eta1 * theta_t.cos();
        Some(num / den)
    }

    /// Parallel (TM/p) polarization reflection coefficient.
    ///
    /// Γ_∥ = (η₂cosθₜ - η₁cosθᵢ)/(η₂cosθₜ + η₁cosθᵢ)
    pub fn gamma_par(&self) -> Option<f64> {
        let theta_t = self.theta_t()?;
        let eta1 = 1.0 / self.n1();
        let eta2 = 1.0 / self.n2();
        let num = eta2 * theta_t.cos() - eta1 * self.theta_i.cos();
        let den = eta2 * theta_t.cos() + eta1 * self.theta_i.cos();
        Some(num / den)
    }

    /// Sample reflection coefficients vs angle for visualization.
    pub fn sample_vs_angle(
        er1: f64,
        er2: f64,
        num_points: usize,
    ) -> FresnelSample {
        let angles: Vec<f64> = (0..num_points)
            .map(|i| i as f64 * (PI / 2.0) / (num_points - 1) as f64)
            .collect();

        let mut gamma_perp = Vec::with_capacity(num_points);
        let mut gamma_par = Vec::with_capacity(num_points);

        for &theta in &angles {
            let oi = ObliqueIncidence::new(er1, er2, theta);
            gamma_perp.push(oi.gamma_perp().unwrap_or(1.0));
            gamma_par.push(oi.gamma_par().unwrap_or(1.0));
        }

        FresnelSample {
            angles,
            gamma_perp,
            gamma_par,
        }
    }
}

/// Sampled Fresnel coefficients for plotting.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FresnelSample {
    pub angles: Vec<f64>,
    pub gamma_perp: Vec<f64>,
    pub gamma_par: Vec<f64>,
}

/// Result from lossy Fresnel calculation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FresnelLossyResult {
    /// TE reflection coefficient (complex) - real part
    pub gamma_te_re: f64,
    /// TE reflection coefficient (complex) - imaginary part
    pub gamma_te_im: f64,
    /// TM reflection coefficient (complex) - real part
    pub gamma_tm_re: f64,
    /// TM reflection coefficient (complex) - imaginary part
    pub gamma_tm_im: f64,
    /// Power reflectance for TE |Γ_TE|²
    pub reflectance_te: f64,
    /// Power reflectance for TM |Γ_TM|²
    pub reflectance_tm: f64,
    /// Phase shift for TE reflection (degrees)
    pub phase_shift_te_deg: f64,
    /// Phase shift for TM reflection (degrees)
    pub phase_shift_tm_deg: f64,
    /// Transmitted power fraction for TE
    pub transmittance_te: f64,
    /// Transmitted power fraction for TM
    pub transmittance_tm: f64,
    /// Complex transmission angle - real part (degrees)
    pub theta_t_re_deg: f64,
    /// Complex transmission angle - imaginary part
    pub theta_t_im: f64,
    /// Is it total internal reflection?
    pub is_pseudo_tir: bool,
}

/// Oblique incidence on a lossy medium.
///
/// Medium 2 has complex permittivity: ε₂ = ε₂' - jε₂'' = ε₂'(1 - jσ/(ωε₂'))
pub fn fresnel_lossy(
    epsilon1: f64,
    epsilon2: f64,
    sigma2: f64,
    frequency: f64,
    angle_deg: f64,
) -> FresnelLossyResult {
    use std::f64::consts::PI;

    let omega = 2.0 * PI * frequency;
    let eps0 = 8.854e-12;

    // Complex permittivity of medium 2
    // ε₂_complex = ε₂ - jσ/ω = ε₀ε₂ᵣ - jσ/ω
    // For relative: ε_r2_complex = ε₂ᵣ - jσ/(ωε₀)
    let eps2_rel_re = epsilon2;
    let eps2_rel_im = -sigma2 / (omega * eps0); // negative for loss

    let theta_i = angle_deg.to_radians();
    let sin_i = theta_i.sin();
    let cos_i = theta_i.cos();

    // Refractive indices (complex for medium 2)
    let n1 = epsilon1.sqrt();
    let n2_sq_re = eps2_rel_re;
    let n2_sq_im = eps2_rel_im;

    // Complex sin(θ_t) from Snell's law: sin(θ_t) = (n1/n2) sin(θ_i)
    // n2² = n2_sq_re + j*n2_sq_im
    // sin²(θ_t) = (n1² / n2²) sin²(θ_i)

    // Complex division: n1²/n2²
    let n1_sq = n1 * n1;
    let denom = n2_sq_re * n2_sq_re + n2_sq_im * n2_sq_im;
    let ratio_re = n1_sq * n2_sq_re / denom;
    let ratio_im = -n1_sq * n2_sq_im / denom;

    // sin²(θ_t) = ratio * sin²(θ_i)
    let sin_i_sq = sin_i * sin_i;
    let sin_t_sq_re = ratio_re * sin_i_sq;
    let sin_t_sq_im = ratio_im * sin_i_sq;

    // cos²(θ_t) = 1 - sin²(θ_t)
    let cos_t_sq_re = 1.0 - sin_t_sq_re;
    let cos_t_sq_im = -sin_t_sq_im;

    // cos(θ_t) = sqrt(cos²(θ_t)) - complex square root
    let (cos_t_re, cos_t_im) = complex_sqrt(cos_t_sq_re, cos_t_sq_im);

    // n2 = sqrt(n2²) - complex square root
    let (n2_re, n2_im) = complex_sqrt(n2_sq_re, n2_sq_im);

    // Fresnel coefficients using complex arithmetic
    // TE (s-pol): Γ_s = (n1·cosθ_i - n2·cosθ_t) / (n1·cosθ_i + n2·cosθ_t)
    // TM (p-pol): Γ_p = (n2·cosθ_i - n1·cosθ_t) / (n2·cosθ_i + n1·cosθ_t)

    // n2 · cos(θ_t) = (n2_re + j·n2_im)(cos_t_re + j·cos_t_im)
    let n2_cos_t_re = n2_re * cos_t_re - n2_im * cos_t_im;
    let n2_cos_t_im = n2_re * cos_t_im + n2_im * cos_t_re;

    // n1 · cos(θ_i) is real
    let n1_cos_i = n1 * cos_i;

    // TE: Γ_s = (n1_cos_i - n2_cos_t) / (n1_cos_i + n2_cos_t)
    let num_te_re = n1_cos_i - n2_cos_t_re;
    let num_te_im = -n2_cos_t_im;
    let den_te_re = n1_cos_i + n2_cos_t_re;
    let den_te_im = n2_cos_t_im;

    let (gamma_te_re, gamma_te_im) = complex_div(num_te_re, num_te_im, den_te_re, den_te_im);

    // TM: Γ_p = (n2·cosθ_i - n1·cosθ_t) / (n2·cosθ_i + n1·cosθ_t)
    // n2 · cos(θ_i) = (n2_re + j·n2_im) · cos_i
    let n2_cos_i_re = n2_re * cos_i;
    let n2_cos_i_im = n2_im * cos_i;

    // n1 · cos(θ_t)
    let n1_cos_t_re = n1 * cos_t_re;
    let n1_cos_t_im = n1 * cos_t_im;

    let num_tm_re = n2_cos_i_re - n1_cos_t_re;
    let num_tm_im = n2_cos_i_im - n1_cos_t_im;
    let den_tm_re = n2_cos_i_re + n1_cos_t_re;
    let den_tm_im = n2_cos_i_im + n1_cos_t_im;

    let (gamma_tm_re, gamma_tm_im) = complex_div(num_tm_re, num_tm_im, den_tm_re, den_tm_im);

    // Magnitudes and phases
    let reflectance_te = gamma_te_re * gamma_te_re + gamma_te_im * gamma_te_im;
    let reflectance_tm = gamma_tm_re * gamma_tm_re + gamma_tm_im * gamma_tm_im;

    let phase_shift_te_deg = gamma_te_im.atan2(gamma_te_re).to_degrees();
    let phase_shift_tm_deg = gamma_tm_im.atan2(gamma_tm_re).to_degrees();

    // For lossy medium, transmitted power is not simply 1 - |Γ|²
    // but we use it as an approximation (actual is more complex due to impedance mismatch)
    let transmittance_te = (1.0 - reflectance_te).max(0.0);
    let transmittance_tm = (1.0 - reflectance_tm).max(0.0);

    // Transmission angle (complex)
    let theta_t_re_deg = cos_t_re.acos().to_degrees();
    let theta_t_im = cos_t_im;

    // Pseudo-TIR occurs when reflectance approaches 1
    let is_pseudo_tir = reflectance_te > 0.99 || reflectance_tm > 0.99;

    FresnelLossyResult {
        gamma_te_re,
        gamma_te_im,
        gamma_tm_re,
        gamma_tm_im,
        reflectance_te,
        reflectance_tm,
        phase_shift_te_deg,
        phase_shift_tm_deg,
        transmittance_te,
        transmittance_tm,
        theta_t_re_deg,
        theta_t_im,
        is_pseudo_tir,
    }
}

/// Sample lossy Fresnel coefficients vs angle.
pub fn fresnel_lossy_vs_angle(
    epsilon1: f64,
    epsilon2: f64,
    sigma2: f64,
    frequency: f64,
    num_points: usize,
) -> FresnelLossySample {
    let mut angles_deg = Vec::with_capacity(num_points);
    let mut reflectance_te = Vec::with_capacity(num_points);
    let mut reflectance_tm = Vec::with_capacity(num_points);
    let mut phase_te = Vec::with_capacity(num_points);
    let mut phase_tm = Vec::with_capacity(num_points);

    for i in 0..num_points {
        let angle = 89.9 * i as f64 / (num_points - 1) as f64;
        angles_deg.push(angle);

        let result = fresnel_lossy(epsilon1, epsilon2, sigma2, frequency, angle);
        reflectance_te.push(result.reflectance_te);
        reflectance_tm.push(result.reflectance_tm);
        phase_te.push(result.phase_shift_te_deg);
        phase_tm.push(result.phase_shift_tm_deg);
    }

    FresnelLossySample {
        angles_deg,
        reflectance_te,
        reflectance_tm,
        phase_te,
        phase_tm,
    }
}

/// Sampled lossy Fresnel data for plotting.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FresnelLossySample {
    pub angles_deg: Vec<f64>,
    pub reflectance_te: Vec<f64>,
    pub reflectance_tm: Vec<f64>,
    pub phase_te: Vec<f64>,
    pub phase_tm: Vec<f64>,
}

/// Complex square root: sqrt(a + jb)
fn complex_sqrt(re: f64, im: f64) -> (f64, f64) {
    let r = (re * re + im * im).sqrt();
    let sqrt_r = r.sqrt();

    if sqrt_r < 1e-15 {
        return (0.0, 0.0);
    }

    // Use the formula: sqrt(z) = sqrt(|z|) * (z + |z|) / |z + |z||
    let re_plus_r = re + r;
    let mag = (re_plus_r * re_plus_r + im * im).sqrt();

    if mag < 1e-15 {
        // z is negative real, return i*sqrt(|z|)
        return (0.0, sqrt_r);
    }

    let scale = sqrt_r / mag;
    (re_plus_r * scale, im * scale)
}

/// Complex division: (a + jb) / (c + jd)
fn complex_div(a: f64, b: f64, c: f64, d: f64) -> (f64, f64) {
    let denom = c * c + d * d;
    if denom < 1e-30 {
        return (0.0, 0.0);
    }
    ((a * c + b * d) / denom, (b * c - a * d) / denom)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    // ================================================================
    // Normal incidence
    // ================================================================

    #[test]
    fn normal_matched_no_reflection() {
        let ni = NormalIncidence::new(377.0, 377.0);
        assert_relative_eq!(ni.gamma(), 0.0, epsilon = 1e-12);
        assert_relative_eq!(ni.tau(), 1.0, epsilon = 1e-12);
    }

    #[test]
    fn normal_power_conservation() {
        let ni = NormalIncidence::new(377.0, 200.0);
        assert_relative_eq!(ni.reflectance() + ni.transmittance(), 1.0, epsilon = 1e-12);
    }

    #[test]
    fn normal_open_circuit_gamma_plus_one() {
        // η₂ → ∞ (PEC boundary from other side)
        let ni = NormalIncidence::new(377.0, 1e10);
        assert_relative_eq!(ni.gamma(), 1.0, max_relative = 1e-6);
    }

    #[test]
    fn normal_short_circuit_gamma_minus_one() {
        // η₂ → 0 (PEC)
        let ni = NormalIncidence::new(377.0, 1e-10);
        assert_relative_eq!(ni.gamma(), -1.0, max_relative = 1e-6);
    }

    #[test]
    fn normal_from_epsilon_r() {
        let ni = NormalIncidence::from_epsilon_r(1.0, 4.0);
        // Γ = (η₀/2 - η₀)/(η₀/2 + η₀) = -1/3
        assert_relative_eq!(ni.gamma(), -1.0 / 3.0, max_relative = 1e-6);
    }

    // ================================================================
    // Oblique incidence - Snell's law
    // ================================================================

    #[test]
    fn snell_normal_incidence() {
        let oi = ObliqueIncidence::new(1.0, 4.0, 0.0);
        assert_relative_eq!(oi.theta_t().unwrap(), 0.0, epsilon = 1e-12);
    }

    #[test]
    fn snell_air_to_glass() {
        // n₁=1, n₂=1.5, θ_i=30° → sin(θ_t) = sin(30°)/1.5 = 1/3
        let oi = ObliqueIncidence::new(1.0, 2.25, PI / 6.0); // n₂ = √2.25 = 1.5
        let theta_t = oi.theta_t().unwrap();
        let expected = (1.0 / 3.0_f64).asin();
        assert_relative_eq!(theta_t, expected, max_relative = 1e-10);
    }

    #[test]
    fn snell_glass_to_air_tir() {
        // n₁=1.5, n₂=1.0, θ_i > θ_c → TIR
        let oi = ObliqueIncidence::new(2.25, 1.0, PI / 3.0); // 60° > θ_c ≈ 41.8°
        assert!(oi.is_tir());
    }

    #[test]
    fn critical_angle_glass_air() {
        let oi = ObliqueIncidence::new(2.25, 1.0, 0.0);
        let theta_c = oi.critical_angle().unwrap();
        let expected = (1.0 / 1.5_f64).asin();
        assert_relative_eq!(theta_c, expected, max_relative = 1e-10);
    }

    #[test]
    fn no_critical_angle_air_to_glass() {
        let oi = ObliqueIncidence::new(1.0, 2.25, 0.0);
        assert!(oi.critical_angle().is_none());
    }

    #[test]
    fn brewster_angle_air_glass() {
        let oi = ObliqueIncidence::new(1.0, 2.25, 0.0);
        let theta_b = oi.brewster_angle();
        let expected = (1.5_f64).atan();
        assert_relative_eq!(theta_b, expected, max_relative = 1e-10);
    }

    #[test]
    fn gamma_par_zero_at_brewster() {
        let er1 = 1.0;
        let er2 = 2.25;
        let ratio: f64 = er2 / er1;
        let theta_b = ratio.sqrt().atan();
        let oi = ObliqueIncidence::new(er1, er2, theta_b);
        assert_relative_eq!(oi.gamma_par().unwrap(), 0.0, epsilon = 1e-6);
    }

    #[test]
    fn gamma_perp_at_normal_matches_normal_incidence() {
        let er1 = 1.0;
        let er2 = 4.0;
        let oi = ObliqueIncidence::new(er1, er2, 0.0);
        let ni = NormalIncidence::from_epsilon_r(er1, er2);
        assert_relative_eq!(oi.gamma_perp().unwrap(), ni.gamma(), max_relative = 1e-6);
    }

    #[test]
    fn sample_vs_angle_dimensions() {
        let s = ObliqueIncidence::sample_vs_angle(1.0, 4.0, 50);
        assert_eq!(s.angles.len(), 50);
        assert_eq!(s.gamma_perp.len(), 50);
        assert_eq!(s.gamma_par.len(), 50);
    }
}

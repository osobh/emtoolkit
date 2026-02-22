//! Rectangular and circular waveguide analysis.

use std::f64::consts::PI;

/// Speed of light in vacuum.
const C: f64 = 2.99792458e8;

/// Rectangular waveguide mode parameters.
#[derive(Debug, Clone)]
pub struct RectWaveguide {
    pub a: f64, // width (broad wall), meters
    pub b: f64, // height (narrow wall), meters
    pub epsilon_r: f64,
    pub mu_r: f64,
}

/// Mode cutoff and propagation info.
#[derive(Debug, Clone)]
pub struct ModeInfo {
    pub m: usize,
    pub n: usize,
    pub f_cutoff: f64,
    pub lambda_cutoff: f64,
    pub propagates: bool,
    pub beta: f64,       // propagation constant (0 if evanescent)
    pub lambda_g: f64,   // guide wavelength (inf if evanescent)
    pub v_phase: f64,    // phase velocity
    pub v_group: f64,    // group velocity
    pub z_mode: f64,     // wave impedance (TE or TM)
    pub mode_type: &'static str,
}

impl RectWaveguide {
    pub fn new(a: f64, b: f64, epsilon_r: f64, mu_r: f64) -> Self {
        Self { a, b, epsilon_r, mu_r }
    }

    /// Speed of light in the filling medium.
    pub fn v_medium(&self) -> f64 {
        C / (self.epsilon_r * self.mu_r).sqrt()
    }

    /// Cutoff frequency for TE_mn or TM_mn mode.
    pub fn cutoff_frequency(&self, m: usize, n: usize) -> f64 {
        let v = self.v_medium();
        0.5 * v * ((m as f64 / self.a).powi(2) + (n as f64 / self.b).powi(2)).sqrt()
    }

    /// Analyze a specific mode at given frequency.
    pub fn mode_at_frequency(&self, m: usize, n: usize, frequency: f64, mode_type: &'static str) -> ModeInfo {
        let fc = self.cutoff_frequency(m, n);
        let lambda_c = self.v_medium() / fc;
        let propagates = frequency > fc;
        let eta = 377.0 / (self.epsilon_r / self.mu_r).sqrt();

        let (beta, lambda_g, v_phase, v_group, z_mode) = if propagates {
            let ratio = fc / frequency;
            let factor = (1.0 - ratio * ratio).sqrt();
            let k = 2.0 * PI * frequency / self.v_medium();
            let b = k * factor;
            let lg = (self.v_medium() / frequency) / factor;
            let vp = self.v_medium() / factor;
            let vg = self.v_medium() * factor;
            let z = if mode_type == "TE" { eta / factor } else { eta * factor };
            (b, lg, vp, vg, z)
        } else {
            (0.0, f64::INFINITY, f64::INFINITY, 0.0, 0.0)
        };

        ModeInfo { m, n, f_cutoff: fc, lambda_cutoff: lambda_c, propagates, beta, lambda_g, v_phase, v_group, z_mode, mode_type }
    }

    /// List all modes up to a given frequency, sorted by cutoff.
    pub fn modes_below(&self, max_freq: f64, max_order: usize) -> Vec<ModeInfo> {
        let mut modes = Vec::new();
        for m in 0..=max_order {
            for n in 0..=max_order {
                if m == 0 && n == 0 { continue; }
                let fc = self.cutoff_frequency(m, n);
                if fc <= max_freq {
                    // TE modes: m,n >= 0 but not both zero
                    modes.push(self.mode_at_frequency(m, n, max_freq, "TE"));
                    // TM modes: m,n >= 1
                    if m >= 1 && n >= 1 {
                        modes.push(self.mode_at_frequency(m, n, max_freq, "TM"));
                    }
                }
            }
        }
        modes.sort_by(|a, b| a.f_cutoff.partial_cmp(&b.f_cutoff).unwrap());
        modes
    }

    /// Dominant mode (TE10) cutoff frequency.
    pub fn dominant_cutoff(&self) -> f64 {
        self.cutoff_frequency(1, 0)
    }

    /// Single-mode bandwidth: f_c(TE10) to f_c(next mode).
    pub fn single_mode_band(&self) -> (f64, f64) {
        let fc10 = self.cutoff_frequency(1, 0);
        let fc20 = self.cutoff_frequency(2, 0);
        let fc01 = self.cutoff_frequency(0, 1);
        let next = fc20.min(fc01);
        (fc10, next)
    }
}

/// Circular waveguide dominant mode (TE11).
pub fn circular_te11_cutoff(radius: f64, epsilon_r: f64, mu_r: f64) -> f64 {
    let v = C / (epsilon_r * mu_r).sqrt();
    // First zero of J1' is p'11 = 1.8412
    1.8412 * v / (2.0 * PI * radius)
}

/// Circular waveguide TM01 cutoff.
pub fn circular_tm01_cutoff(radius: f64, epsilon_r: f64, mu_r: f64) -> f64 {
    let v = C / (epsilon_r * mu_r).sqrt();
    // First zero of J0 is p01 = 2.4049
    2.4049 * v / (2.0 * PI * radius)
}

/// Parallel plate waveguide (two infinite conducting plates).
///
/// Supports TE_n and TM_n modes for n ≥ 1.
#[derive(Debug, Clone, Copy)]
pub struct ParallelPlateWaveguide {
    /// Plate separation, meters
    pub d: f64,
    /// Relative permittivity of fill
    pub epsilon_r: f64,
    /// Relative permeability of fill
    pub mu_r: f64,
}

/// Mode information for parallel plate waveguide.
#[derive(Debug, Clone)]
pub struct ParallelPlateMode {
    pub mode_number: usize,
    pub mode_type: &'static str,
    pub f_cutoff: f64,
    pub propagates: bool,
    pub beta: f64,
    pub lambda_g: f64,
    pub v_phase: f64,
    pub v_group: f64,
}

impl ParallelPlateWaveguide {
    pub fn new(d: f64, epsilon_r: f64, mu_r: f64) -> Self {
        Self { d, epsilon_r, mu_r }
    }

    /// Speed of light in the filling medium.
    pub fn v_medium(&self) -> f64 {
        C / (self.epsilon_r * self.mu_r).sqrt()
    }

    /// Cutoff frequency for TE_n or TM_n mode.
    ///
    /// f_c = n / (2d√(με))
    pub fn cutoff_frequency(&self, mode_n: usize) -> f64 {
        if mode_n == 0 {
            return 0.0; // TEM mode has no cutoff
        }
        mode_n as f64 / (2.0 * self.d * (self.epsilon_r * self.mu_r).sqrt()) * C
    }

    /// Guide wavelength for mode n at frequency f.
    ///
    /// λ_g = λ / √(1 - (f_c/f)²)
    pub fn guide_wavelength(&self, frequency: f64, mode_n: usize) -> f64 {
        let fc = self.cutoff_frequency(mode_n);
        if frequency <= fc {
            return f64::INFINITY;
        }
        let lambda = self.v_medium() / frequency;
        lambda / (1.0 - (fc / frequency).powi(2)).sqrt()
    }

    /// Phase velocity for mode n at frequency f.
    ///
    /// v_p = v / √(1 - (f_c/f)²)
    pub fn phase_velocity(&self, frequency: f64, mode_n: usize) -> f64 {
        let fc = self.cutoff_frequency(mode_n);
        if frequency <= fc {
            return f64::INFINITY;
        }
        self.v_medium() / (1.0 - (fc / frequency).powi(2)).sqrt()
    }

    /// Group velocity for mode n at frequency f.
    ///
    /// v_g = v · √(1 - (f_c/f)²)
    pub fn group_velocity(&self, frequency: f64, mode_n: usize) -> f64 {
        let fc = self.cutoff_frequency(mode_n);
        if frequency <= fc {
            return 0.0;
        }
        self.v_medium() * (1.0 - (fc / frequency).powi(2)).sqrt()
    }

    /// Propagation constant β for mode n at frequency f.
    pub fn beta(&self, frequency: f64, mode_n: usize) -> f64 {
        let fc = self.cutoff_frequency(mode_n);
        if frequency <= fc {
            return 0.0;
        }
        let k = 2.0 * PI * frequency / self.v_medium();
        k * (1.0 - (fc / frequency).powi(2)).sqrt()
    }

    /// Analyze a specific mode at given frequency.
    pub fn mode_at_frequency(&self, mode_n: usize, frequency: f64, mode_type: &'static str) -> ParallelPlateMode {
        let fc = self.cutoff_frequency(mode_n);
        let propagates = frequency > fc;
        let (beta, lambda_g, v_phase, v_group) = if propagates {
            (
                self.beta(frequency, mode_n),
                self.guide_wavelength(frequency, mode_n),
                self.phase_velocity(frequency, mode_n),
                self.group_velocity(frequency, mode_n),
            )
        } else {
            (0.0, f64::INFINITY, f64::INFINITY, 0.0)
        };

        ParallelPlateMode {
            mode_number: mode_n,
            mode_type,
            f_cutoff: fc,
            propagates,
            beta,
            lambda_g,
            v_phase,
            v_group,
        }
    }

    /// List cutoff frequencies for first N modes.
    pub fn first_n_cutoffs(&self, n: usize) -> Vec<(usize, f64)> {
        (1..=n).map(|i| (i, self.cutoff_frequency(i))).collect()
    }

    /// Get all modes (TE and TM) that propagate at given frequency.
    pub fn propagating_modes(&self, frequency: f64, max_order: usize) -> Vec<ParallelPlateMode> {
        let mut modes = Vec::new();
        for n in 1..=max_order {
            let fc = self.cutoff_frequency(n);
            if fc < frequency {
                modes.push(self.mode_at_frequency(n, frequency, "TE"));
                modes.push(self.mode_at_frequency(n, frequency, "TM"));
            }
        }
        modes.sort_by(|a, b| a.f_cutoff.partial_cmp(&b.f_cutoff).unwrap());
        modes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parallel_plate_cutoff() {
        // d = 1cm, air-filled
        let pp = ParallelPlateWaveguide::new(0.01, 1.0, 1.0);
        let fc1 = pp.cutoff_frequency(1);
        // f_c1 = c/(2d) = 3e8 / 0.02 = 15 GHz
        assert!((fc1 / 1e9 - 15.0).abs() < 0.1);
    }

    #[test]
    fn test_parallel_plate_phase_group_product() {
        let pp = ParallelPlateWaveguide::new(0.01, 1.0, 1.0);
        let f = 20e9;
        let vp = pp.phase_velocity(f, 1);
        let vg = pp.group_velocity(f, 1);
        let v = pp.v_medium();
        assert!((vp * vg - v * v).abs() / (v * v) < 0.001);
    }

    #[test]
    fn test_wr90_dominant() {
        // WR-90: a=22.86mm, b=10.16mm (X-band)
        let wg = RectWaveguide::new(0.02286, 0.01016, 1.0, 1.0);
        let fc = wg.dominant_cutoff();
        // TE10 cutoff ≈ 6.56 GHz
        assert!((fc / 1e9 - 6.56).abs() < 0.02);
    }

    #[test]
    fn test_single_mode_band() {
        let wg = RectWaveguide::new(0.02286, 0.01016, 1.0, 1.0);
        let (f1, f2) = wg.single_mode_band();
        // Single mode: 6.56 - 13.12 GHz
        assert!((f1 / 1e9 - 6.56).abs() < 0.02);
        assert!((f2 / 1e9 - 13.12).abs() < 0.05);
    }

    #[test]
    fn test_te10_propagation() {
        let wg = RectWaveguide::new(0.02286, 0.01016, 1.0, 1.0);
        let mode = wg.mode_at_frequency(1, 0, 10e9, "TE");
        assert!(mode.propagates);
        assert!(mode.beta > 0.0);
        assert!(mode.v_phase > C);
        assert!(mode.v_group < C);
        assert!((mode.v_phase * mode.v_group - C * C).abs() / (C * C) < 0.01);
    }

    #[test]
    fn test_evanescent() {
        let wg = RectWaveguide::new(0.02286, 0.01016, 1.0, 1.0);
        let mode = wg.mode_at_frequency(1, 0, 5e9, "TE");
        assert!(!mode.propagates);
        assert_eq!(mode.beta, 0.0);
    }

    #[test]
    fn test_modes_below() {
        let wg = RectWaveguide::new(0.02286, 0.01016, 1.0, 1.0);
        let modes = wg.modes_below(20e9, 3);
        assert!(modes.len() >= 3);
        assert_eq!(modes[0].m, 1);
        assert_eq!(modes[0].n, 0);
    }

    #[test]
    fn test_circular_te11() {
        let fc = circular_te11_cutoff(0.01, 1.0, 1.0);
        assert!(fc > 8e9 && fc < 10e9);
    }

    #[test]
    fn test_phase_group_product() {
        let wg = RectWaveguide::new(0.02286, 0.01016, 1.0, 1.0);
        let mode = wg.mode_at_frequency(1, 0, 10e9, "TE");
        let product = mode.v_phase * mode.v_group;
        let v_sq = C * C;
        assert!((product - v_sq).abs() / v_sq < 0.001);
    }
}

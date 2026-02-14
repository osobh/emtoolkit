//! Inductance calculations for standard geometries.

use std::f64::consts::PI;

const MU_0: f64 = 4.0e-7 * PI;

/// Solenoid inductance.
/// L = μ₀ μᵣ N² A / ℓ
pub fn solenoid(turns: usize, length: f64, radius: f64, mu_r: f64) -> f64 {
    let a = PI * radius * radius;
    MU_0 * mu_r * (turns as f64).powi(2) * a / length
}

/// Toroid inductance.
/// L = μ₀ μᵣ N² h ln(b/a) / (2π)
pub fn toroid(turns: usize, inner_r: f64, outer_r: f64, height: f64, mu_r: f64) -> f64 {
    MU_0 * mu_r * (turns as f64).powi(2) * height * (outer_r / inner_r).ln() / (2.0 * PI)
}

/// Coaxial cable inductance per unit length.
/// L/ℓ = μ₀ ln(b/a) / (2π)  (external only)
pub fn coaxial_per_length(inner_r: f64, outer_r: f64, mu_r: f64) -> f64 {
    MU_0 * mu_r * (outer_r / inner_r).ln() / (2.0 * PI)
}

/// Coaxial cable total inductance.
pub fn coaxial(inner_r: f64, outer_r: f64, mu_r: f64, length: f64) -> f64 {
    coaxial_per_length(inner_r, outer_r, mu_r) * length
}

/// Two parallel wires inductance per unit length.
/// L/ℓ = μ₀/π × ln(d/a) (for d >> a)
pub fn parallel_wires_per_length(wire_radius: f64, separation: f64, mu_r: f64) -> f64 {
    MU_0 * mu_r / PI * (separation / wire_radius).ln()
}

/// Mutual inductance of coaxial loops (Neumann formula approximation).
/// M ≈ μ₀ π a² b² / (2(a² + d²)^(3/2))  for small loops
pub fn mutual_coaxial_loops(radius_a: f64, radius_b: f64, separation: f64) -> f64 {
    let num = MU_0 * PI * radius_a.powi(2) * radius_b.powi(2);
    let denom = 2.0 * (radius_a.powi(2) + separation.powi(2)).powf(1.5);
    num / denom
}

/// Stored energy in an inductor.
/// W = ½ L I²
pub fn energy(inductance: f64, current: f64) -> f64 {
    0.5 * inductance * current * current
}

/// Series combination.
/// L_total = L₁ + L₂ + ... (no mutual coupling)
pub fn series(inductors: &[f64]) -> f64 {
    inductors.iter().sum()
}

/// Parallel combination.
/// 1/L_total = 1/L₁ + 1/L₂ + ...
pub fn parallel(inductors: &[f64]) -> f64 {
    let inv_sum: f64 = inductors.iter().map(|l| 1.0 / l).sum();
    1.0 / inv_sum
}

/// Coupling coefficient from mutual inductance.
/// k = M / √(L₁ L₂)
pub fn coupling_coefficient(m: f64, l1: f64, l2: f64) -> f64 {
    m / (l1 * l2).sqrt()
}

/// Time constant for RL circuit.
/// τ = L/R
pub fn rl_time_constant(inductance: f64, resistance: f64) -> f64 {
    inductance / resistance
}

/// RL step response: i(t) = (V/R)(1 - e^(-t/τ))
pub fn rl_step_response(voltage: f64, resistance: f64, inductance: f64, t_end: f64, n: usize) -> (Vec<f64>, Vec<f64>) {
    let tau = inductance / resistance;
    let i_final = voltage / resistance;
    let mut ts = Vec::with_capacity(n);
    let mut is = Vec::with_capacity(n);
    for i in 0..n {
        let t = t_end * i as f64 / (n - 1) as f64;
        ts.push(t);
        is.push(i_final * (1.0 - (-t / tau).exp()));
    }
    (ts, is)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solenoid() {
        let l = solenoid(100, 0.1, 0.02, 1.0);
        let expected = MU_0 * 10000.0 * PI * 0.0004 / 0.1;
        assert!((l - expected).abs() / expected < 1e-10);
    }

    #[test]
    fn test_toroid() {
        let l = toroid(500, 0.05, 0.08, 0.02, 1.0);
        assert!(l > 0.0);
    }

    #[test]
    fn test_coaxial() {
        let lpl = coaxial_per_length(0.001, 0.004, 1.0);
        assert!(lpl > 0.0);
        let l = coaxial(0.001, 0.004, 1.0, 1.0);
        assert!((l - lpl).abs() < 1e-20);
    }

    #[test]
    fn test_parallel_wires() {
        let lpl = parallel_wires_per_length(0.001, 0.1, 1.0);
        assert!(lpl > 0.0);
    }

    #[test]
    fn test_mutual() {
        let m = mutual_coaxial_loops(0.05, 0.05, 0.1);
        assert!(m > 0.0);
        // Closer should have higher M
        let m_close = mutual_coaxial_loops(0.05, 0.05, 0.01);
        assert!(m_close > m);
    }

    #[test]
    fn test_energy() {
        let w = energy(1e-3, 10.0);
        assert!((w - 0.05).abs() < 1e-10);
    }

    #[test]
    fn test_series_parallel() {
        let l = vec![1e-3, 2e-3];
        assert!((series(&l) - 3e-3).abs() < 1e-15);
        let p = parallel(&l);
        let expected = 1.0 / (1.0 / 1e-3 + 1.0 / 2e-3);
        assert!((p - expected).abs() / expected < 1e-10);
    }

    #[test]
    fn test_rl_step() {
        let (ts, is) = rl_step_response(10.0, 100.0, 0.1, 0.01, 100);
        assert_eq!(ts.len(), 100);
        assert!(is[0] < 0.001); // starts near zero
        assert!((is[99] - 0.1).abs() < 0.005); // approaches V/R = 0.1
    }

    #[test]
    fn test_coupling() {
        let k = coupling_coefficient(1e-3, 2e-3, 2e-3);
        assert!((k - 0.5).abs() < 1e-10);
    }
}

import { useState, useMemo } from 'react';
import Plot from 'react-plotly.js';
import * as wasm from '../lib/em_wasm';

interface FresnelLossyResult {
  gamma_te_re: number;
  gamma_te_im: number;
  gamma_tm_re: number;
  gamma_tm_im: number;
  reflectance_te: number;
  reflectance_tm: number;
  phase_shift_te_deg: number;
  phase_shift_tm_deg: number;
  transmittance_te: number;
  transmittance_tm: number;
  theta_t_re_deg: number;
  is_pseudo_tir: boolean;
}

interface FresnelLossySample {
  angles_deg: number[];
  reflectance_te: number[];
  reflectance_tm: number[];
  phase_te: number[];
  phase_tm: number[];
}

// Common lossy materials with approximate conductivity
const MATERIALS: Record<string, { eps: number; sigma: number; desc: string }> = {
  'Dry Earth': { eps: 4, sigma: 1e-3, desc: 'Dry earth/soil' },
  'Wet Earth': { eps: 30, sigma: 0.01, desc: 'Wet earth' },
  'Fresh Water': { eps: 80, sigma: 0.01, desc: 'Fresh water' },
  'Sea Water': { eps: 80, sigma: 4, desc: 'Sea water' },
  'Copper': { eps: 1, sigma: 5.8e7, desc: 'Copper (conductor)' },
  'Aluminum': { eps: 1, sigma: 3.5e7, desc: 'Aluminum' },
  'Silicon': { eps: 11.7, sigma: 1e-3, desc: 'Doped silicon' },
  'Biological': { eps: 50, sigma: 1, desc: 'Biological tissue' },
  'Custom': { eps: 4, sigma: 0.01, desc: 'Custom' },
};

export function FresnelLossyModule() {
  const [epsilon1, setEpsilon1] = useState(1.0);
  const [material, setMaterial] = useState('Sea Water');
  const [customEps, setCustomEps] = useState(4);
  const [customSigma, setCustomSigma] = useState(0.01);
  const [frequency, setFrequency] = useState(1e9);
  const [angle, setAngle] = useState(45);

  const epsilon2 = material === 'Custom' ? customEps : MATERIALS[material].eps;
  const sigma2 = material === 'Custom' ? customSigma : MATERIALS[material].sigma;

  // Single point result
  const result: FresnelLossyResult | null = useMemo(() => {
    try {
      return wasm.fresnel_lossy_coefficients(
        epsilon1,
        epsilon2,
        sigma2,
        frequency,
        angle
      ) as FresnelLossyResult;
    } catch {
      return null;
    }
  }, [epsilon1, epsilon2, sigma2, frequency, angle]);

  // Angle sweep for plotting
  const sweepResult: FresnelLossySample | null = useMemo(() => {
    try {
      return wasm.fresnel_lossy_vs_angle(epsilon1, epsilon2, sigma2, frequency, 180) as FresnelLossySample;
    } catch {
      return null;
    }
  }, [epsilon1, epsilon2, sigma2, frequency]);

  // Calculate loss tangent for display
  const omega = 2 * Math.PI * frequency;
  const eps0 = 8.854e-12;
  const lossTangent = sigma2 / (omega * eps0 * epsilon2);

  return (
    <div className="module">
      <h2>Oblique Incidence on Lossy Medium</h2>
      <p>
        Calculate Fresnel reflection coefficients for oblique incidence on a lossy medium
        with complex permittivity ε = ε' - jσ/ω.
      </p>

      <div className="controls">
        <label>
          Medium 1 εᵣ (lossless):
          <input
            type="range"
            min={1}
            max={10}
            step={0.1}
            value={epsilon1}
            onChange={e => setEpsilon1(+e.target.value)}
          />
          {epsilon1.toFixed(1)}
        </label>

        <label>
          Medium 2 material:
          <select value={material} onChange={e => setMaterial(e.target.value)}>
            {Object.entries(MATERIALS).map(([k, v]) => (
              <option key={k} value={k}>
                {k} — {v.desc}
              </option>
            ))}
          </select>
        </label>

        {material === 'Custom' && (
          <>
            <label>
              εᵣ:
              <input
                type="number"
                min={1}
                max={100}
                step={0.1}
                value={customEps}
                onChange={e => setCustomEps(+e.target.value)}
              />
            </label>
            <label>
              σ (S/m):
              <input
                type="number"
                min={0}
                step={0.001}
                value={customSigma}
                onChange={e => setCustomSigma(+e.target.value)}
              />
            </label>
          </>
        )}

        <label>
          Frequency:
          <input
            type="range"
            min={6}
            max={11}
            step={0.1}
            value={Math.log10(frequency)}
            onChange={e => setFrequency(Math.pow(10, +e.target.value))}
          />
          {frequency >= 1e9
            ? (frequency / 1e9).toFixed(2) + ' GHz'
            : (frequency / 1e6).toFixed(2) + ' MHz'}
        </label>

        <label>
          Angle of incidence:
          <input
            type="range"
            min={0}
            max={89}
            step={1}
            value={angle}
            onChange={e => setAngle(+e.target.value)}
          />
          {angle}°
        </label>
      </div>

      {result && (
        <>
          <div className="results-grid">
            <div className="result-card">
              <span className="label">Loss tangent</span>
              <span className="value">
                {lossTangent > 1000 ? '∞ (conductor)' : lossTangent.toFixed(4)}
              </span>
            </div>
            <div className="result-card">
              <span className="label">|Γ_TE|²</span>
              <span className="value">{result.reflectance_te.toFixed(4)}</span>
            </div>
            <div className="result-card">
              <span className="label">|Γ_TM|²</span>
              <span className="value">{result.reflectance_tm.toFixed(4)}</span>
            </div>
            <div className="result-card">
              <span className="label">Phase TE</span>
              <span className="value">{result.phase_shift_te_deg.toFixed(1)}°</span>
            </div>
            <div className="result-card">
              <span className="label">Phase TM</span>
              <span className="value">{result.phase_shift_tm_deg.toFixed(1)}°</span>
            </div>
            <div className="result-card">
              <span className="label">θ_t (real)</span>
              <span className="value">{result.theta_t_re_deg.toFixed(1)}°</span>
            </div>
          </div>

          <div className="results-grid">
            <div className="result-card">
              <span className="label">Γ_TE</span>
              <span className="value">
                {result.gamma_te_re.toFixed(4)} {result.gamma_te_im >= 0 ? '+' : '-'}{' '}
                j{Math.abs(result.gamma_te_im).toFixed(4)}
              </span>
            </div>
            <div className="result-card">
              <span className="label">Γ_TM</span>
              <span className="value">
                {result.gamma_tm_re.toFixed(4)} {result.gamma_tm_im >= 0 ? '+' : '-'}{' '}
                j{Math.abs(result.gamma_tm_im).toFixed(4)}
              </span>
            </div>
          </div>
        </>
      )}

      {sweepResult && (
        <>
          <Plot
            data={[
              {
                x: sweepResult.angles_deg,
                y: sweepResult.reflectance_te,
                type: 'scatter',
                mode: 'lines',
                name: '|Γ_TE|² (s-pol)',
                line: { color: '#2196F3', width: 2 },
              },
              {
                x: sweepResult.angles_deg,
                y: sweepResult.reflectance_tm,
                type: 'scatter',
                mode: 'lines',
                name: '|Γ_TM|² (p-pol)',
                line: { color: '#4CAF50', width: 2 },
              },
              {
                x: [angle, angle],
                y: [0, 1],
                type: 'scatter',
                mode: 'lines',
                name: 'Current angle',
                line: { color: '#F44336', width: 1, dash: 'dash' },
              },
            ]}
            layout={{
              title: 'Power Reflectance vs Angle',
              xaxis: { title: 'Angle of incidence (°)', range: [0, 90] },
              yaxis: { title: '|Γ|²', range: [0, 1.05] },
              margin: { t: 40, r: 20, b: 50, l: 60 },
              height: 380,
              legend: { x: 0.02, y: 0.98 },
            }}
            config={{ responsive: true }}
            style={{ width: '100%' }}
          />

          <Plot
            data={[
              {
                x: sweepResult.angles_deg,
                y: sweepResult.phase_te,
                type: 'scatter',
                mode: 'lines',
                name: 'Phase TE',
                line: { color: '#2196F3', width: 2 },
              },
              {
                x: sweepResult.angles_deg,
                y: sweepResult.phase_tm,
                type: 'scatter',
                mode: 'lines',
                name: 'Phase TM',
                line: { color: '#4CAF50', width: 2 },
              },
            ]}
            layout={{
              title: 'Reflection Phase vs Angle',
              xaxis: { title: 'Angle of incidence (°)', range: [0, 90] },
              yaxis: { title: 'Phase shift (°)' },
              margin: { t: 40, r: 20, b: 50, l: 60 },
              height: 350,
              legend: { x: 0.02, y: 0.98 },
            }}
            config={{ responsive: true }}
            style={{ width: '100%' }}
          />
        </>
      )}

      <div className="theory">
        <h3>Theory</h3>
        <p>
          <strong>Complex permittivity:</strong> ε = ε'(1 - jσ/(ωε')) where loss tangent = σ/(ωε')
        </p>
        <p>
          <strong>Complex refractive index:</strong> n̂ = n - jk where k is extinction coefficient
        </p>
        <p>
          <strong>Modified Snell's law:</strong> sin(θ_t) = (n₁/n̂₂)·sin(θ_i) (θ_t is complex)
        </p>
        <p>
          <strong>Fresnel (TE):</strong> Γ_s = (n₁cosθ_i - n̂₂cosθ_t) / (n₁cosθ_i + n̂₂cosθ_t)
        </p>
        <p>
          <strong>Fresnel (TM):</strong> Γ_p = (n̂₂cosθ_i - n₁cosθ_t) / (n̂₂cosθ_i + n₁cosθ_t)
        </p>
        <p>
          <strong>Note:</strong> For good conductors (σ ≫ ωε), |Γ| → 1 at all angles
        </p>
      </div>
    </div>
  );
}

import { useState, useMemo } from 'react';
import Plot from 'react-plotly.js';
import * as wasm from '../lib/em_wasm';

const MATERIALS: Record<string, { epsilon_r: number; sigma: number; label: string }> = {
  copper: { epsilon_r: 1.0, sigma: 5.8e7, label: 'Copper (σ = 5.8×10⁷ S/m)' },
  aluminum: { epsilon_r: 1.0, sigma: 3.5e7, label: 'Aluminum (σ = 3.5×10⁷ S/m)' },
  seawater: { epsilon_r: 81, sigma: 4.0, label: 'Seawater (σ = 4 S/m, εᵣ = 81)' },
  iron: { epsilon_r: 1.0, sigma: 1.0e7, label: 'Iron (σ = 1×10⁷ S/m)' },
  silicon: { epsilon_r: 11.7, sigma: 1.56e-3, label: 'Silicon (σ = 1.56×10⁻³ S/m)' },
  wet_soil: { epsilon_r: 10, sigma: 0.01, label: 'Wet Soil (σ = 0.01 S/m, εᵣ = 10)' },
};

export function SkinDepthModule() {
  const [material, setMaterial] = useState('copper');
  const [frequency, setFrequency] = useState(1e9);
  const [e0, setE0] = useState(1.0);

  const mat = MATERIALS[material];

  const freqData = useMemo(() => {
    try {
      return wasm.skin_depth_vs_frequency(mat.epsilon_r, mat.sigma, 1e3, 1e12, 500) as {
        frequencies: number[]; skin_depths: number[]; alphas: number[];
      };
    } catch { return null; }
  }, [mat]);

  const attenData = useMemo(() => {
    try {
      const props = wasm.medium_properties(mat.epsilon_r, 1.0, mat.sigma, frequency) as { skin_depth: number };
      const zMax = props.skin_depth * 5;
      return wasm.attenuation_profile(mat.epsilon_r, mat.sigma, frequency, e0, zMax, 300) as {
        z: number[]; e_magnitude: number[]; poynting: number[]; skin_depth: number; alpha: number;
      };
    } catch { return null; }
  }, [mat, frequency, e0]);

  const pointProps = useMemo(() => {
    try {
      return wasm.medium_properties(mat.epsilon_r, 1.0, mat.sigma, frequency) as {
        skin_depth: number; alpha: number; beta: number; phase_velocity: number;
        wavelength: number; loss_tangent: number; is_good_conductor: boolean;
      };
    } catch { return null; }
  }, [mat, frequency]);

  return (
    <div className="module">
      <h2>Skin Depth & Attenuation</h2>
      <p>Explore how EM waves attenuate in conductors and lossy media. The skin depth δ is the distance at which fields decay to 1/e (36.8%).</p>

      <div className="controls">
        <label>
          Material:
          <select value={material} onChange={e => setMaterial(e.target.value)}>
            {Object.entries(MATERIALS).map(([k, v]) => (
              <option key={k} value={k}>{v.label}</option>
            ))}
          </select>
        </label>
        <label>
          Frequency: <input type="range" min={3} max={12} step={0.1}
            value={Math.log10(frequency)} onChange={e => setFrequency(10 ** +e.target.value)} />
          {frequency >= 1e9 ? (frequency / 1e9).toFixed(1) + ' GHz'
            : frequency >= 1e6 ? (frequency / 1e6).toFixed(1) + ' MHz'
            : frequency >= 1e3 ? (frequency / 1e3).toFixed(1) + ' kHz'
            : frequency.toFixed(0) + ' Hz'}
        </label>
        <label>
          E₀ (V/m): <input type="range" min={0.1} max={100} step={0.1} value={e0}
            onChange={e => setE0(+e.target.value)} /> {e0.toFixed(1)}
        </label>
      </div>

      {pointProps && (
        <div className="results-grid">
          <div className="result-card">
            <span className="label">Skin Depth δ</span>
            <span className="value">{pointProps.skin_depth < 1e-3
              ? (pointProps.skin_depth * 1e6).toFixed(2) + ' μm'
              : pointProps.skin_depth < 1
              ? (pointProps.skin_depth * 1e3).toFixed(3) + ' mm'
              : pointProps.skin_depth.toFixed(3) + ' m'}</span>
          </div>
          <div className="result-card">
            <span className="label">α (Np/m)</span>
            <span className="value">{pointProps.alpha.toExponential(3)}</span>
          </div>
          <div className="result-card">
            <span className="label">Loss tangent</span>
            <span className="value">{pointProps.loss_tangent.toExponential(3)}</span>
          </div>
          <div className="result-card">
            <span className="label">Classification</span>
            <span className="value">{pointProps.is_good_conductor ? 'Good Conductor' : pointProps.loss_tangent < 0.01 ? 'Low-Loss' : 'Lossy'}</span>
          </div>
        </div>
      )}

      {attenData && (
        <Plot
          data={[
            {
              x: attenData.z.map(z => z * (attenData.skin_depth < 1e-3 ? 1e6 : attenData.skin_depth < 1 ? 1e3 : 1)),
              y: attenData.e_magnitude.map(e => e / e0),
              type: 'scatter',
              mode: 'lines',
              name: '|E(z)|/E₀',
              line: { color: '#F44336', width: 2 },
            },
            {
              x: attenData.z.map(z => z * (attenData.skin_depth < 1e-3 ? 1e6 : attenData.skin_depth < 1 ? 1e3 : 1)),
              y: attenData.poynting.map(s => s / attenData.poynting[0]),
              type: 'scatter',
              mode: 'lines',
              name: 'S(z)/S₀',
              line: { color: '#2196F3', width: 2, dash: 'dash' },
            },
          ]}
          layout={{
            title: 'Field Attenuation in ' + mat.label.split(' (')[0],
            xaxis: { title: `z (${attenData.skin_depth < 1e-3 ? 'μm' : attenData.skin_depth < 1 ? 'mm' : 'm'})` },
            yaxis: { title: 'Normalized amplitude', range: [0, 1.05] },
            margin: { t: 40, r: 20, b: 50, l: 60 },
            height: 350,
            shapes: [{
              type: 'line',
              x0: attenData.skin_depth * (attenData.skin_depth < 1e-3 ? 1e6 : attenData.skin_depth < 1 ? 1e3 : 1),
              x1: attenData.skin_depth * (attenData.skin_depth < 1e-3 ? 1e6 : attenData.skin_depth < 1 ? 1e3 : 1),
              y0: 0, y1: 1, line: { color: '#4CAF50', width: 2, dash: 'dot' },
            }],
            annotations: [{
              x: attenData.skin_depth * (attenData.skin_depth < 1e-3 ? 1e6 : attenData.skin_depth < 1 ? 1e3 : 1),
              y: 0.368, text: 'δ (1/e)', showarrow: true, arrowhead: 2, font: { color: '#4CAF50' },
            }],
            legend: { x: 0.6, y: 0.95 },
          }}
          config={{ responsive: true }}
          style={{ width: '100%' }}
        />
      )}

      {freqData && (
        <Plot
          data={[{
            x: freqData.frequencies,
            y: freqData.skin_depths,
            type: 'scatter',
            mode: 'lines',
            name: 'δ(f)',
            line: { color: '#9C27B0', width: 2 },
          }]}
          layout={{
            title: 'Skin Depth vs Frequency',
            xaxis: { title: 'Frequency (Hz)', type: 'log' },
            yaxis: { title: 'Skin Depth (m)', type: 'log' },
            margin: { t: 40, r: 20, b: 50, l: 60 },
            height: 350,
          }}
          config={{ responsive: true }}
          style={{ width: '100%' }}
        />
      )}

      <div className="theory">
        <h3>Theory</h3>
        <p><strong>Good conductor:</strong> δ = 1/√(πfμσ)</p>
        <p><strong>General:</strong> δ = 1/α, where α = Re(γ)</p>
        <p>At z = δ: |E| = E₀/e ≈ 0.368 E₀. Power drops to 1/e² ≈ 13.5%.</p>
        <p>At z = 5δ: &lt; 1% of original field remains — effectively opaque.</p>
      </div>
    </div>
  );
}

import { useState, useMemo } from 'react';
import Plot from 'react-plotly.js';
import * as wasm from '../lib/em_wasm';

const STANDARDS: Record<string, { a: number; b: number; band: string }> = {
  'WR-284': { a: 72.14, b: 34.04, band: 'S-band (2.6-3.95 GHz)' },
  'WR-187': { a: 47.55, b: 22.15, band: 'C-band (3.95-5.85 GHz)' },
  'WR-137': { a: 34.85, b: 15.80, band: 'C-band (5.85-8.20 GHz)' },
  'WR-90': { a: 22.86, b: 10.16, band: 'X-band (8.2-12.4 GHz)' },
  'WR-62': { a: 15.80, b: 7.90, band: 'Ku-band (12.4-18.0 GHz)' },
  'WR-42': { a: 10.67, b: 4.32, band: 'K-band (18.0-26.5 GHz)' },
  'WR-28': { a: 7.11, b: 3.56, band: 'Ka-band (26.5-40.0 GHz)' },
  'Custom': { a: 22.86, b: 10.16, band: 'Custom' },
};

interface ModeEntry {
  mode: string;
  f_cutoff: number;
  propagates: boolean;
  beta: number;
  lambda_g: number;
  v_phase: number;
  v_group: number;
  z_mode: number;
}

interface WGResult {
  dominant_cutoff: number;
  dominant_propagates: boolean;
  dominant_beta: number;
  dominant_lambda_g: number;
  dominant_v_phase: number;
  dominant_v_group: number;
  dominant_z_te: number;
  single_mode_min: number;
  single_mode_max: number;
  modes: ModeEntry[];
}

export function WaveguideModule() {
  const [standard, setStandard] = useState('WR-90');
  const [customA, setCustomA] = useState(22.86);
  const [customB, setCustomB] = useState(10.16);
  const [epsilonR, setEpsilonR] = useState(1.0);
  const [frequency, setFrequency] = useState(10e9);

  const a = standard === 'Custom' ? customA : STANDARDS[standard].a;
  const b = standard === 'Custom' ? customB : STANDARDS[standard].b;

  const result: WGResult | null = useMemo(() => {
    try {
      return wasm.waveguide_rect(a, b, epsilonR, frequency) as WGResult;
    } catch { return null; }
  }, [a, b, epsilonR, frequency]);

  const dispersionData = useMemo(() => {
    if (!result) return null;
    const fc = result.dominant_cutoff;
    const fMin = fc * 0.5;
    const fMax = fc * 3;
    const nPts = 200;
    const freqs: number[] = [];
    const betas: number[] = [];
    const lightLine: number[] = [];
    for (let i = 0; i < nPts; i++) {
      const f = fMin + (fMax - fMin) * i / (nPts - 1);
      freqs.push(f / 1e9);
      const c = 3e8 / Math.sqrt(epsilonR);
      lightLine.push(2 * Math.PI * f / c);
      if (f > fc) {
        const ratio = fc / f;
        const k = 2 * Math.PI * f / c;
        betas.push(k * Math.sqrt(1 - ratio * ratio));
      } else {
        betas.push(NaN);
      }
    }
    return { freqs, betas, lightLine };
  }, [result, epsilonR]);

  return (
    <div className="module">
      <h2>Rectangular Waveguide</h2>
      <p>Analyze modes, cutoff frequencies, and propagation parameters for rectangular waveguides.</p>

      <div className="controls">
        <label>
          Standard:
          <select value={standard} onChange={e => setStandard(e.target.value)}>
            {Object.entries(STANDARDS).map(([k, v]) => (
              <option key={k} value={k}>{k} — {v.band}</option>
            ))}
          </select>
        </label>

        {standard === 'Custom' && (
          <>
            <label>a (mm): <input type="range" min={5} max={100} step={0.1} value={customA}
              onChange={e => setCustomA(+e.target.value)} /> {customA.toFixed(1)}</label>
            <label>b (mm): <input type="range" min={2} max={50} step={0.1} value={customB}
              onChange={e => setCustomB(+e.target.value)} /> {customB.toFixed(1)}</label>
          </>
        )}

        <label>Fill εᵣ: <input type="range" min={1} max={10} step={0.1} value={epsilonR}
          onChange={e => setEpsilonR(+e.target.value)} /> {epsilonR.toFixed(1)}</label>

        <label>Frequency: <input type="range" min={1e9} max={50e9} step={1e8} value={frequency}
          onChange={e => setFrequency(+e.target.value)} /> {(frequency / 1e9).toFixed(1)} GHz</label>
      </div>

      {result && (
        <>
          <div className="results-grid">
            <div className="result-card">
              <span className="label">TE₁₀ cutoff</span>
              <span className="value">{(result.dominant_cutoff / 1e9).toFixed(3)} GHz</span>
            </div>
            <div className="result-card">
              <span className="label">Single-mode band</span>
              <span className="value">{(result.single_mode_min / 1e9).toFixed(2)} – {(result.single_mode_max / 1e9).toFixed(2)} GHz</span>
            </div>
            <div className="result-card">
              <span className="label">Status</span>
              <span className="value">{result.dominant_propagates ? '✓ Propagating' : '✗ Evanescent'}</span>
            </div>
            {result.dominant_propagates && (
              <>
                <div className="result-card">
                  <span className="label">Guide λ_g</span>
                  <span className="value">{(result.dominant_lambda_g * 1e3).toFixed(2)} mm</span>
                </div>
                <div className="result-card">
                  <span className="label">v_phase</span>
                  <span className="value">{(result.dominant_v_phase / 3e8).toFixed(3)} c</span>
                </div>
                <div className="result-card">
                  <span className="label">v_group</span>
                  <span className="value">{(result.dominant_v_group / 3e8).toFixed(3)} c</span>
                </div>
                <div className="result-card">
                  <span className="label">Z_TE</span>
                  <span className="value">{result.dominant_z_te.toFixed(1)} Ω</span>
                </div>
                <div className="result-card">
                  <span className="label">β</span>
                  <span className="value">{result.dominant_beta.toFixed(2)} rad/m</span>
                </div>
              </>
            )}
          </div>

          {result.modes.length > 0 && (
            <div style={{ marginTop: 20, overflowX: 'auto' }}>
              <h3>Modes at {(frequency / 1e9).toFixed(1)} GHz</h3>
              <table style={{ width: '100%', borderCollapse: 'collapse', fontSize: 13 }}>
                <thead>
                  <tr style={{ borderBottom: '2px solid #ddd' }}>
                    <th style={{ padding: 6, textAlign: 'left' }}>Mode</th>
                    <th style={{ padding: 6 }}>f_c (GHz)</th>
                    <th style={{ padding: 6 }}>Status</th>
                    <th style={{ padding: 6 }}>λ_g (mm)</th>
                    <th style={{ padding: 6 }}>Z (Ω)</th>
                  </tr>
                </thead>
                <tbody>
                  {result.modes.map((m, i) => (
                    <tr key={i} style={{ background: i % 2 ? '#f9f9f9' : 'white' }}>
                      <td style={{ padding: 6, fontWeight: 'bold' }}>{m.mode}</td>
                      <td style={{ padding: 6, textAlign: 'center' }}>{(m.f_cutoff / 1e9).toFixed(3)}</td>
                      <td style={{ padding: 6, textAlign: 'center', color: m.propagates ? '#4CAF50' : '#F44336' }}>
                        {m.propagates ? 'Propagating' : 'Evanescent'}
                      </td>
                      <td style={{ padding: 6, textAlign: 'center' }}>{m.propagates ? (m.lambda_g * 1e3).toFixed(1) : '—'}</td>
                      <td style={{ padding: 6, textAlign: 'center' }}>{m.propagates ? m.z_mode.toFixed(1) : '—'}</td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          )}
        </>
      )}

      {dispersionData && (
        <Plot
          data={[
            {
              x: dispersionData.freqs,
              y: dispersionData.betas,
              type: 'scatter',
              mode: 'lines',
              name: 'TE₁₀ β(f)',
              line: { color: '#2196F3', width: 2 },
            },
            {
              x: dispersionData.freqs,
              y: dispersionData.lightLine,
              type: 'scatter',
              mode: 'lines',
              name: 'Light line (k = ω/c)',
              line: { color: '#999', width: 1, dash: 'dash' },
            },
          ]}
          layout={{
            title: 'Dispersion Diagram',
            xaxis: { title: 'Frequency (GHz)' },
            yaxis: { title: 'β (rad/m)' },
            margin: { t: 40, r: 20, b: 50, l: 60 },
            height: 380,
            legend: { x: 0.02, y: 0.98 },
          }}
          config={{ responsive: true }}
          style={{ width: '100%' }}
        />
      )}

      <div className="theory">
        <h3>Theory</h3>
        <p><strong>Cutoff:</strong> f_c(mn) = (v/2)√((m/a)² + (n/b)²)</p>
        <p><strong>Guide wavelength:</strong> λ_g = λ/√(1 − (f_c/f)²)</p>
        <p><strong>Phase velocity:</strong> v_p = v/√(1 − (f_c/f)²) &gt; c (not energy velocity)</p>
        <p><strong>Group velocity:</strong> v_g = v√(1 − (f_c/f)²), v_p · v_g = v²</p>
        <p><strong>TE impedance:</strong> Z_TE = η/√(1 − (f_c/f)²)</p>
      </div>
    </div>
  );
}

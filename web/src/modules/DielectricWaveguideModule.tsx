import { useState, useMemo } from 'react';
import Plot from 'react-plotly.js';
import * as wasm from '../lib/em_wasm';

interface DielectricMode {
  mode_number: number;
  mode_type: string;
  beta: number;
  effective_index: number;
  gamma: number;
  kappa: number;
  confinement: number;
}

interface SlabResult {
  v_number: number;
  num_modes_estimate: number;
  n_core: number;
  n_clad: number;
  numerical_aperture: number;
  modes: DielectricMode[];
}

// Common optical waveguide materials
const MATERIALS: Record<string, { n: number; desc: string }> = {
  'Si': { n: 3.48, desc: 'Silicon (telecom)' },
  'SiN': { n: 2.0, desc: 'Silicon Nitride' },
  'SiO2': { n: 1.45, desc: 'Silica' },
  'GaAs': { n: 3.4, desc: 'Gallium Arsenide' },
  'InP': { n: 3.17, desc: 'Indium Phosphide' },
  'Polymer': { n: 1.5, desc: 'Polymer/PMMA' },
  'Air': { n: 1.0, desc: 'Air' },
};

export function DielectricWaveguideModule() {
  const [thickness, setThickness] = useState(0.5); // μm
  const [coreMat, setCoreMat] = useState('Si');
  const [cladMat, setCladMat] = useState('SiO2');
  const [customCore, setCustomCore] = useState(3.48);
  const [customClad, setCustomClad] = useState(1.45);
  const [wavelength, setWavelength] = useState(1.55); // μm
  const [maxModes, setMaxModes] = useState(10);

  const nCore = coreMat === 'Custom' ? customCore : MATERIALS[coreMat].n;
  const nClad = cladMat === 'Custom' ? customClad : MATERIALS[cladMat].n;
  const epsCore = nCore * nCore;
  const epsClad = nClad * nClad;

  const result: SlabResult | null = useMemo(() => {
    if (nCore <= nClad) return null;
    try {
      return wasm.dielectric_slab_waveguide(
        thickness,
        epsCore,
        epsClad,
        wavelength,
        maxModes
      ) as SlabResult;
    } catch {
      return null;
    }
  }, [thickness, epsCore, epsClad, wavelength, maxModes, nCore, nClad]);

  const modeIndexPlot = useMemo(() => {
    if (!result || result.modes.length === 0) return null;
    const teModes = result.modes.filter(m => m.mode_type === 'TE');
    const tmModes = result.modes.filter(m => m.mode_type === 'TM');
    return { teModes, tmModes };
  }, [result]);

  return (
    <div className="module">
      <h2>Dielectric Slab Waveguide</h2>
      <p>
        Analyze guided modes in a symmetric dielectric slab waveguide. The slab (core) has
        higher refractive index than the surrounding cladding.
      </p>

      <div className="controls">
        <label>
          Slab thickness (μm):
          <input
            type="range"
            min={0.1}
            max={10}
            step={0.1}
            value={thickness}
            onChange={e => setThickness(+e.target.value)}
          />
          {thickness.toFixed(2)} μm
        </label>

        <label>
          Wavelength (μm):
          <input
            type="range"
            min={0.4}
            max={3.0}
            step={0.01}
            value={wavelength}
            onChange={e => setWavelength(+e.target.value)}
          />
          {wavelength.toFixed(2)} μm
        </label>

        <label>
          Core material:
          <select value={coreMat} onChange={e => setCoreMat(e.target.value)}>
            {Object.entries(MATERIALS).map(([k, v]) => (
              <option key={k} value={k}>
                {k} (n={v.n}) — {v.desc}
              </option>
            ))}
            <option value="Custom">Custom</option>
          </select>
        </label>

        {coreMat === 'Custom' && (
          <label>
            Core n:
            <input
              type="number"
              min={1}
              max={5}
              step={0.01}
              value={customCore}
              onChange={e => setCustomCore(+e.target.value)}
            />
          </label>
        )}

        <label>
          Cladding material:
          <select value={cladMat} onChange={e => setCladMat(e.target.value)}>
            {Object.entries(MATERIALS).map(([k, v]) => (
              <option key={k} value={k}>
                {k} (n={v.n}) — {v.desc}
              </option>
            ))}
            <option value="Custom">Custom</option>
          </select>
        </label>

        {cladMat === 'Custom' && (
          <label>
            Clad n:
            <input
              type="number"
              min={1}
              max={5}
              step={0.01}
              value={customClad}
              onChange={e => setCustomClad(+e.target.value)}
            />
          </label>
        )}
      </div>

      {nCore <= nClad && (
        <div className="warning" style={{ color: '#F44336', padding: 10 }}>
          ⚠️ Core refractive index must be greater than cladding for waveguiding!
        </div>
      )}

      {result && (
        <>
          <div className="results-grid">
            <div className="result-card">
              <span className="label">V-number</span>
              <span className="value">{result.v_number.toFixed(3)}</span>
            </div>
            <div className="result-card">
              <span className="label">Est. # modes (per pol)</span>
              <span className="value">{result.num_modes_estimate}</span>
            </div>
            <div className="result-card">
              <span className="label">Found modes</span>
              <span className="value">{result.modes.length}</span>
            </div>
            <div className="result-card">
              <span className="label">n_core</span>
              <span className="value">{result.n_core.toFixed(4)}</span>
            </div>
            <div className="result-card">
              <span className="label">n_clad</span>
              <span className="value">{result.n_clad.toFixed(4)}</span>
            </div>
            <div className="result-card">
              <span className="label">NA</span>
              <span className="value">{result.numerical_aperture.toFixed(4)}</span>
            </div>
          </div>

          {modeIndexPlot && (modeIndexPlot.teModes.length > 0 || modeIndexPlot.tmModes.length > 0) && (
            <Plot
              data={[
                {
                  x: modeIndexPlot.teModes.map(m => m.mode_number),
                  y: modeIndexPlot.teModes.map(m => m.effective_index),
                  type: 'scatter',
                  mode: 'markers+lines',
                  name: 'TE modes',
                  marker: { size: 10, color: '#2196F3' },
                },
                {
                  x: modeIndexPlot.tmModes.map(m => m.mode_number),
                  y: modeIndexPlot.tmModes.map(m => m.effective_index),
                  type: 'scatter',
                  mode: 'markers+lines',
                  name: 'TM modes',
                  marker: { size: 10, color: '#4CAF50' },
                },
                {
                  x: [-0.5, Math.max(modeIndexPlot.teModes.length, modeIndexPlot.tmModes.length)],
                  y: [result.n_core, result.n_core],
                  type: 'scatter',
                  mode: 'lines',
                  line: { dash: 'dash', color: '#999' },
                  name: 'n_core',
                },
                {
                  x: [-0.5, Math.max(modeIndexPlot.teModes.length, modeIndexPlot.tmModes.length)],
                  y: [result.n_clad, result.n_clad],
                  type: 'scatter',
                  mode: 'lines',
                  line: { dash: 'dash', color: '#999' },
                  name: 'n_clad',
                },
              ]}
              layout={{
                title: 'Effective Index vs Mode Number',
                xaxis: { title: 'Mode number', tickmode: 'linear', tick0: 0, dtick: 1 },
                yaxis: { title: 'Effective index n_eff' },
                margin: { t: 40, r: 20, b: 50, l: 60 },
                height: 380,
                legend: { x: 0.7, y: 0.98 },
              }}
              config={{ responsive: true }}
              style={{ width: '100%' }}
            />
          )}

          {result.modes.length > 0 && (
            <div style={{ marginTop: 20, overflowX: 'auto' }}>
              <h3>Guided Modes</h3>
              <table style={{ width: '100%', borderCollapse: 'collapse', fontSize: 13 }}>
                <thead>
                  <tr style={{ borderBottom: '2px solid #ddd' }}>
                    <th style={{ padding: 6, textAlign: 'left' }}>Mode</th>
                    <th style={{ padding: 6 }}>n_eff</th>
                    <th style={{ padding: 6 }}>β (rad/m)</th>
                    <th style={{ padding: 6 }}>κ (rad/m)</th>
                    <th style={{ padding: 6 }}>γ (1/m)</th>
                    <th style={{ padding: 6 }}>Confinement</th>
                  </tr>
                </thead>
                <tbody>
                  {result.modes.map((m, i) => (
                    <tr key={i} style={{ background: i % 2 ? '#f9f9f9' : 'white' }}>
                      <td style={{ padding: 6, fontWeight: 'bold' }}>
                        {m.mode_type}_{m.mode_number}
                      </td>
                      <td style={{ padding: 6, textAlign: 'center' }}>
                        {m.effective_index.toFixed(4)}
                      </td>
                      <td style={{ padding: 6, textAlign: 'center' }}>
                        {(m.beta / 1e6).toFixed(3)} ×10⁶
                      </td>
                      <td style={{ padding: 6, textAlign: 'center' }}>
                        {(m.kappa / 1e6).toFixed(3)} ×10⁶
                      </td>
                      <td style={{ padding: 6, textAlign: 'center' }}>
                        {(m.gamma / 1e6).toFixed(3)} ×10⁶
                      </td>
                      <td style={{ padding: 6, textAlign: 'center' }}>
                        {(m.confinement * 100).toFixed(1)}%
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          )}
        </>
      )}

      <div className="theory">
        <h3>Theory</h3>
        <p>
          <strong>V-number:</strong> V = (πd/λ) · √(n₁² - n₂²) = (πd/λ) · NA
        </p>
        <p>
          <strong>Guidance condition:</strong> n_clad &lt; n_eff &lt; n_core
        </p>
        <p>
          <strong>Single-mode:</strong> V &lt; π/2 ≈ 1.57 (for symmetric slab)
        </p>
        <p>
          <strong>Dispersion equation (TE):</strong> tan(κd/2) = γ/κ (even), -cot(κd/2) = γ/κ (odd)
        </p>
        <p>
          <strong>Where:</strong> κ² = k₀²n₁² - β², γ² = β² - k₀²n₂²
        </p>
      </div>
    </div>
  );
}

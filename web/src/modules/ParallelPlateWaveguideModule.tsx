import { useState, useMemo } from 'react';
import Plot from 'react-plotly.js';
import * as wasm from '../lib/em_wasm';

interface CutoffEntry {
  mode_n: number;
  f_cutoff: number;
}

interface PropagatingMode {
  mode: string;
  mode_number: number;
  mode_type: string;
  f_cutoff: number;
  beta: number;
  lambda_g: number;
  v_phase: number;
  v_group: number;
}

interface PPResult {
  d_m: number;
  v_medium: number;
  first_cutoff: number;
  mode1_propagates: boolean;
  mode1_beta: number;
  mode1_lambda_g: number;
  mode1_v_phase: number;
  mode1_v_group: number;
  cutoffs: CutoffEntry[];
  propagating_modes: PropagatingMode[];
}

export function ParallelPlateWaveguideModule() {
  const [spacing, setSpacing] = useState(10); // mm
  const [epsilonR, setEpsilonR] = useState(1.0);
  const [frequency, setFrequency] = useState(20e9);
  const [maxModes, setMaxModes] = useState(5);

  const result: PPResult | null = useMemo(() => {
    try {
      return wasm.parallel_plate_waveguide(spacing, epsilonR, frequency, maxModes) as PPResult;
    } catch {
      return null;
    }
  }, [spacing, epsilonR, frequency, maxModes]);

  const cutoffPlotData = useMemo(() => {
    if (!result) return null;
    const modeNumbers = result.cutoffs.map(c => c.mode_n);
    const cutoffs = result.cutoffs.map(c => c.f_cutoff / 1e9);
    return { modeNumbers, cutoffs };
  }, [result]);

  return (
    <div className="module">
      <h2>Parallel Plate Waveguide</h2>
      <p>
        Analyze TE and TM modes in a parallel plate waveguide with infinite plates
        separated by distance d.
      </p>

      <div className="controls">
        <label>
          Plate spacing d (mm):
          <input
            type="range"
            min={1}
            max={50}
            step={0.5}
            value={spacing}
            onChange={e => setSpacing(+e.target.value)}
          />
          {spacing.toFixed(1)} mm
        </label>

        <label>
          Fill εᵣ:
          <input
            type="range"
            min={1}
            max={10}
            step={0.1}
            value={epsilonR}
            onChange={e => setEpsilonR(+e.target.value)}
          />
          {epsilonR.toFixed(1)}
        </label>

        <label>
          Frequency:
          <input
            type="range"
            min={1e9}
            max={100e9}
            step={1e9}
            value={frequency}
            onChange={e => setFrequency(+e.target.value)}
          />
          {(frequency / 1e9).toFixed(1)} GHz
        </label>

        <label>
          Max modes to show:
          <input
            type="number"
            min={1}
            max={20}
            value={maxModes}
            onChange={e => setMaxModes(+e.target.value)}
          />
        </label>
      </div>

      {result && (
        <>
          <div className="results-grid">
            <div className="result-card">
              <span className="label">First cutoff (n=1)</span>
              <span className="value">{(result.first_cutoff / 1e9).toFixed(3)} GHz</span>
            </div>
            <div className="result-card">
              <span className="label">Mode 1 status</span>
              <span className="value">
                {result.mode1_propagates ? '✓ Propagating' : '✗ Evanescent'}
              </span>
            </div>
            {result.mode1_propagates && (
              <>
                <div className="result-card">
                  <span className="label">λ_g (mode 1)</span>
                  <span className="value">{(result.mode1_lambda_g * 1e3).toFixed(2)} mm</span>
                </div>
                <div className="result-card">
                  <span className="label">v_phase</span>
                  <span className="value">{(result.mode1_v_phase / 3e8).toFixed(3)} c</span>
                </div>
                <div className="result-card">
                  <span className="label">v_group</span>
                  <span className="value">{(result.mode1_v_group / 3e8).toFixed(3)} c</span>
                </div>
                <div className="result-card">
                  <span className="label">β (mode 1)</span>
                  <span className="value">{result.mode1_beta.toFixed(2)} rad/m</span>
                </div>
              </>
            )}
          </div>

          <h3>Cutoff Frequencies</h3>
          {cutoffPlotData && (
            <Plot
              data={[
                {
                  x: cutoffPlotData.modeNumbers,
                  y: cutoffPlotData.cutoffs,
                  type: 'bar',
                  marker: { color: '#2196F3' },
                  name: 'Cutoff',
                },
                {
                  x: [0, maxModes + 1],
                  y: [frequency / 1e9, frequency / 1e9],
                  type: 'scatter',
                  mode: 'lines',
                  line: { color: '#F44336', width: 2, dash: 'dash' },
                  name: 'Operating freq',
                },
              ]}
              layout={{
                title: 'Mode Cutoff Frequencies',
                xaxis: { title: 'Mode number n', tickmode: 'linear', tick0: 1, dtick: 1 },
                yaxis: { title: 'Cutoff frequency (GHz)' },
                margin: { t: 40, r: 20, b: 50, l: 60 },
                height: 350,
                showlegend: true,
                legend: { x: 0.02, y: 0.98 },
              }}
              config={{ responsive: true }}
              style={{ width: '100%' }}
            />
          )}

          {result.propagating_modes.length > 0 && (
            <div style={{ marginTop: 20, overflowX: 'auto' }}>
              <h3>Propagating Modes at {(frequency / 1e9).toFixed(1)} GHz</h3>
              <table style={{ width: '100%', borderCollapse: 'collapse', fontSize: 13 }}>
                <thead>
                  <tr style={{ borderBottom: '2px solid #ddd' }}>
                    <th style={{ padding: 6, textAlign: 'left' }}>Mode</th>
                    <th style={{ padding: 6 }}>f_c (GHz)</th>
                    <th style={{ padding: 6 }}>β (rad/m)</th>
                    <th style={{ padding: 6 }}>λ_g (mm)</th>
                    <th style={{ padding: 6 }}>v_p / c</th>
                    <th style={{ padding: 6 }}>v_g / c</th>
                  </tr>
                </thead>
                <tbody>
                  {result.propagating_modes.map((m, i) => (
                    <tr key={i} style={{ background: i % 2 ? '#f9f9f9' : 'white' }}>
                      <td style={{ padding: 6, fontWeight: 'bold' }}>{m.mode}</td>
                      <td style={{ padding: 6, textAlign: 'center' }}>
                        {(m.f_cutoff / 1e9).toFixed(3)}
                      </td>
                      <td style={{ padding: 6, textAlign: 'center' }}>{m.beta.toFixed(2)}</td>
                      <td style={{ padding: 6, textAlign: 'center' }}>
                        {(m.lambda_g * 1e3).toFixed(2)}
                      </td>
                      <td style={{ padding: 6, textAlign: 'center' }}>
                        {(m.v_phase / 3e8).toFixed(3)}
                      </td>
                      <td style={{ padding: 6, textAlign: 'center' }}>
                        {(m.v_group / 3e8).toFixed(3)}
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
          <strong>Cutoff frequency:</strong> f_c = n / (2d√(με)) for TE_n and TM_n modes
        </p>
        <p>
          <strong>Guide wavelength:</strong> λ_g = λ / √(1 − (f_c/f)²)
        </p>
        <p>
          <strong>Phase velocity:</strong> v_p = v / √(1 − (f_c/f)²) &gt; c
        </p>
        <p>
          <strong>Group velocity:</strong> v_g = v · √(1 − (f_c/f)²) &lt; c
        </p>
        <p>
          <strong>Note:</strong> v_p · v_g = v² (product of phase and group velocities equals
          medium velocity squared)
        </p>
      </div>
    </div>
  );
}

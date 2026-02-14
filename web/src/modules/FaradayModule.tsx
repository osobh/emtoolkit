import { useState, useMemo } from 'react';
import Plot from 'react-plotly.js';
import { wasm } from '../wasm';

type Mode = 'emf' | 'generator' | 'transformer';

export function FaradayModule() {
  const [mode, setMode] = useState<Mode>('generator');
  const [bPeak, setBPeak] = useState(0.5);
  const [area, setArea] = useState(0.01);
  const [omega, setOmega] = useState(120 * Math.PI);
  const [turns, setTurns] = useState(100);
  const [rpm, setRpm] = useState(3600);
  // transformer
  const [nPrimary, setNPrimary] = useState(100);
  const [nSecondary, setNSecondary] = useState(50);
  const [vPrimary, setVPrimary] = useState(120);
  const [iPrimary, setIPrimary] = useState(2);

  const emfData = useMemo(
    () => mode === 'emf' ? wasm.sinusoidal_emf(bPeak, area, omega, 0.05, 500) : null,
    [mode, bPeak, area, omega],
  );

  const genResult = useMemo(
    () => mode === 'generator' ? wasm.ac_generator(turns, bPeak, area, rpm) : null,
    [mode, turns, bPeak, area, rpm],
  );

  const txResult = useMemo(
    () => mode === 'transformer' ? wasm.transformer(nPrimary, nSecondary, vPrimary, iPrimary) : null,
    [mode, nPrimary, nSecondary, vPrimary, iPrimary],
  );

  return (
    <div className="module-panel">
      <h2>Faraday's Law & Time-Varying Fields</h2>
      <div className="controls">
        <div className="control-group">
          <label>Mode</label>
          <select value={mode} onChange={e => setMode(e.target.value as Mode)}>
            <option value="emf">Sinusoidal EMF</option>
            <option value="generator">AC Generator</option>
            <option value="transformer">Transformer</option>
          </select>
        </div>
        {(mode === 'emf' || mode === 'generator') && (
          <>
            <div className="control-group"><label>B peak (T)</label><input type="number" value={bPeak} onChange={e => setBPeak(+e.target.value)} step={0.1} min={0.01} /></div>
            <div className="control-group"><label>Area (m²)</label><input type="number" value={area} onChange={e => setArea(+e.target.value)} step={0.001} min={0.001} /></div>
          </>
        )}
        {mode === 'emf' && (
          <div className="control-group"><label>ω (rad/s)</label><input type="number" value={omega} onChange={e => setOmega(+e.target.value)} step={10} min={1} /></div>
        )}
        {mode === 'generator' && (
          <>
            <div className="control-group"><label>Turns</label><input type="number" value={turns} onChange={e => setTurns(+e.target.value)} step={10} min={1} /></div>
            <div className="control-group"><label>RPM</label><input type="number" value={rpm} onChange={e => setRpm(+e.target.value)} step={100} min={100} /></div>
          </>
        )}
        {mode === 'transformer' && (
          <>
            <div className="control-group"><label>N₁</label><input type="number" value={nPrimary} onChange={e => setNPrimary(+e.target.value)} step={10} min={1} /></div>
            <div className="control-group"><label>N₂</label><input type="number" value={nSecondary} onChange={e => setNSecondary(+e.target.value)} step={10} min={1} /></div>
            <div className="control-group"><label>V₁ (V)</label><input type="number" value={vPrimary} onChange={e => setVPrimary(+e.target.value)} step={10} /></div>
            <div className="control-group"><label>I₁ (A)</label><input type="number" value={iPrimary} onChange={e => setIPrimary(+e.target.value)} step={0.5} min={0.01} /></div>
          </>
        )}
      </div>

      {mode === 'emf' && emfData && (
        <Plot
          data={[{ x: emfData.times, y: emfData.emf, mode: 'lines', line: { color: '#e67e22', width: 2 }, name: 'EMF(t)' }]}
          layout={{
            width: 800, height: 400,
            xaxis: { title: { text: 'Time (s)' } }, yaxis: { title: { text: 'EMF (V)' } },
            margin: { t: 20, b: 50, l: 60, r: 20 },
            paper_bgcolor: 'transparent', plot_bgcolor: 'white',
          }}
          config={{ responsive: true }}
        />
      )}

      {mode === 'generator' && genResult && (
        <div className="result-box">
          <div className="result-grid">
            <div className="result-item"><span className="result-label">Peak EMF</span><span className="result-value">{genResult.emf_peak.toFixed(2)} V</span></div>
            <div className="result-item"><span className="result-label">RMS EMF</span><span className="result-value">{genResult.emf_rms.toFixed(2)} V</span></div>
            <div className="result-item"><span className="result-label">Frequency</span><span className="result-value">{genResult.frequency.toFixed(1)} Hz</span></div>
            <div className="result-item"><span className="result-label">ω</span><span className="result-value">{genResult.omega.toFixed(1)} rad/s</span></div>
          </div>
        </div>
      )}

      {mode === 'transformer' && txResult && (
        <div className="result-box">
          <div className="result-grid">
            <div className="result-item"><span className="result-label">Turns Ratio</span><span className="result-value">{txResult.turns_ratio.toFixed(3)}</span></div>
            <div className="result-item"><span className="result-label">V₂</span><span className="result-value">{txResult.v_secondary.toFixed(2)} V</span></div>
            <div className="result-item"><span className="result-label">I₂</span><span className="result-value">{txResult.i_secondary.toFixed(3)} A</span></div>
            <div className="result-item"><span className="result-label">Power</span><span className="result-value">{txResult.power.toFixed(1)} W</span></div>
          </div>
        </div>
      )}
    </div>
  );
}

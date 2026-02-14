import { useState, useMemo } from 'react';
import Plot from 'react-plotly.js';
import * as wasm from '../lib/em_wasm';

interface SolenoidResult {
  b_interior: number;
  inductance: number;
  flux: number;
  energy: number;
  turns_per_meter: number;
}

export function SolenoidModule() {
  const [turns, setTurns] = useState(100);
  const [length, setLength] = useState(0.1);
  const [current, setCurrent] = useState(1.0);
  const [radius, setRadius] = useState(0.02);
  const [muR, setMuR] = useState(1.0);

  const result: SolenoidResult | null = useMemo(() => {
    try {
      return wasm.solenoid_params(turns, length, current, radius, muR) as SolenoidResult;
    } catch { return null; }
  }, [turns, length, current, radius, muR]);

  const axialData = useMemo(() => {
    try {
      return wasm.current_loop_on_axis(radius, current * turns / length, -length, length, 200) as {
        z: number[]; bz: number[];
      };
    } catch { return null; }
  }, [radius, current, turns, length]);

  return (
    <div className="module">
      <h2>Solenoid & Inductor</h2>
      <p>Calculate B-field, inductance, flux, and stored energy for a solenoid with optional magnetic core.</p>

      <div className="controls">
        <label>
          Turns (N): <input type="range" min={10} max={1000} step={10} value={turns}
            onChange={e => setTurns(+e.target.value)} /> {turns}
        </label>
        <label>
          Length (m): <input type="range" min={0.01} max={1.0} step={0.01} value={length}
            onChange={e => setLength(+e.target.value)} /> {length.toFixed(2)}
        </label>
        <label>
          Current (A): <input type="range" min={0.1} max={20} step={0.1} value={current}
            onChange={e => setCurrent(+e.target.value)} /> {current.toFixed(1)}
        </label>
        <label>
          Radius (m): <input type="range" min={0.005} max={0.1} step={0.005} value={radius}
            onChange={e => setRadius(+e.target.value)} /> {radius.toFixed(3)}
        </label>
        <label>
          Core μᵣ: <input type="range" min={1} max={5000} step={1} value={muR}
            onChange={e => setMuR(+e.target.value)} /> {muR}
        </label>
      </div>

      {result && (
        <div className="results-grid">
          <div className="result-card">
            <span className="label">B interior</span>
            <span className="value">{(result.b_interior * 1e3).toFixed(3)} mT</span>
          </div>
          <div className="result-card">
            <span className="label">Inductance</span>
            <span className="value">{result.inductance < 1e-3
              ? (result.inductance * 1e6).toFixed(2) + ' μH'
              : (result.inductance * 1e3).toFixed(3) + ' mH'}</span>
          </div>
          <div className="result-card">
            <span className="label">Flux</span>
            <span className="value">{(result.flux * 1e6).toFixed(3)} μWb</span>
          </div>
          <div className="result-card">
            <span className="label">Stored Energy</span>
            <span className="value">{result.energy < 1e-3
              ? (result.energy * 1e6).toFixed(2) + ' μJ'
              : (result.energy * 1e3).toFixed(3) + ' mJ'}</span>
          </div>
          <div className="result-card">
            <span className="label">n (turns/m)</span>
            <span className="value">{result.turns_per_meter.toFixed(0)}</span>
          </div>
        </div>
      )}

      {axialData && (
        <Plot
          data={[{
            x: axialData.z.map(z => z * 100),
            y: axialData.bz.map(b => b * 1e3),
            type: 'scatter',
            mode: 'lines',
            name: 'Bz on axis',
            line: { color: '#2196F3', width: 2 },
          }]}
          layout={{
            title: 'Axial B-Field Profile',
            xaxis: { title: 'z (cm)' },
            yaxis: { title: 'Bz (mT)' },
            margin: { t: 40, r: 20, b: 50, l: 60 },
            height: 400,
            shapes: [
              { type: 'rect', x0: 0, x1: length * 100, y0: 0, y1: 1e6,
                fillcolor: 'rgba(33,150,243,0.1)', line: { width: 0 } },
            ],
          }}
          config={{ responsive: true }}
          style={{ width: '100%' }}
        />
      )}

      <div className="theory">
        <h3>Theory</h3>
        <p><strong>Interior field:</strong> B = μ₀μᵣ n I, where n = N/ℓ</p>
        <p><strong>Inductance:</strong> L = μ₀μᵣ n² A ℓ = μ₀μᵣ N² π r² / ℓ</p>
        <p><strong>Flux linkage:</strong> Λ = N Φ = L I</p>
        <p><strong>Stored energy:</strong> W = ½ L I²</p>
      </div>
    </div>
  );
}

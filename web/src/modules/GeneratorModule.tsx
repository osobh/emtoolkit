import { useState, useMemo } from 'react';
import Plot from 'react-plotly.js';
import * as wasm from '../lib/em_wasm';

interface GeneratorResult {
  emf_peak: number;
  frequency: number;
  omega: number;
  flux_peak: number;
}

export function GeneratorModule() {
  const [turns, setTurns] = useState(50);
  const [bField, setBField] = useState(0.5);
  const [area, setArea] = useState(0.01);
  const [rpm, setRpm] = useState(3600);

  const result: GeneratorResult | null = useMemo(() => {
    try {
      return wasm.ac_generator(turns, bField, area, rpm) as GeneratorResult;
    } catch { return null; }
  }, [turns, bField, area, rpm]);

  const emfData = useMemo(() => {
    if (!result) return null;
    try {
      const tEnd = 3.0 / result.frequency;
      return wasm.sinusoidal_emf(bField, area * turns, result.omega, tEnd, 500) as {
        t: number[]; emf: number[];
      };
    } catch { return null; }
  }, [result, turns, bField, area]);

  return (
    <div className="module">
      <h2>AC Generator</h2>
      <p>A coil rotating in a uniform B-field generates sinusoidal EMF via Faraday's law.</p>

      <div className="controls">
        <label>
          Turns (N): <input type="range" min={1} max={200} step={1} value={turns}
            onChange={e => setTurns(+e.target.value)} /> {turns}
        </label>
        <label>
          B (T): <input type="range" min={0.01} max={2.0} step={0.01} value={bField}
            onChange={e => setBField(+e.target.value)} /> {bField.toFixed(2)}
        </label>
        <label>
          Area (m²): <input type="range" min={0.001} max={0.1} step={0.001} value={area}
            onChange={e => setArea(+e.target.value)} /> {area.toFixed(3)}
        </label>
        <label>
          Speed (RPM): <input type="range" min={100} max={10000} step={100} value={rpm}
            onChange={e => setRpm(+e.target.value)} /> {rpm}
        </label>
      </div>

      {result && (
        <div className="results-grid">
          <div className="result-card">
            <span className="label">Peak EMF</span>
            <span className="value">{result.emf_peak.toFixed(2)} V</span>
          </div>
          <div className="result-card">
            <span className="label">Frequency</span>
            <span className="value">{result.frequency.toFixed(1)} Hz</span>
          </div>
          <div className="result-card">
            <span className="label">ω</span>
            <span className="value">{result.omega.toFixed(1)} rad/s</span>
          </div>
          <div className="result-card">
            <span className="label">Peak Flux</span>
            <span className="value">{(result.flux_peak * 1e3).toFixed(3)} mWb</span>
          </div>
          <div className="result-card">
            <span className="label">RMS EMF</span>
            <span className="value">{(result.emf_peak / Math.sqrt(2)).toFixed(2)} V</span>
          </div>
        </div>
      )}

      {emfData && (
        <Plot
          data={[{
            x: emfData.t.map(t => t * 1e3),
            y: emfData.emf,
            type: 'scatter',
            mode: 'lines',
            name: 'EMF(t)',
            line: { color: '#F44336', width: 2 },
          }]}
          layout={{
            title: 'Generated EMF',
            xaxis: { title: 'Time (ms)' },
            yaxis: { title: 'EMF (V)' },
            margin: { t: 40, r: 20, b: 50, l: 60 },
            height: 400,
          }}
          config={{ responsive: true }}
          style={{ width: '100%' }}
        />
      )}

      <div className="theory">
        <h3>Theory</h3>
        <p><strong>Flux:</strong> Φ(t) = N B A cos(ωt)</p>
        <p><strong>EMF:</strong> ε = -dΦ/dt = N B A ω sin(ωt)</p>
        <p><strong>Peak EMF:</strong> ε₀ = N B A ω = N B A (2π × RPM/60)</p>
        <p>At 60 Hz (3600 RPM), a standard US generator produces sinusoidal output.</p>
      </div>
    </div>
  );
}

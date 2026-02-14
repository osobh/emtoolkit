import { useState, useMemo } from 'react';
import Plot from 'react-plotly.js';
import * as wasm from '../lib/em_wasm';

interface QWResult {
  z_transformer: number;
  transformer_length: number;
  frequency: number;
  gamma_before: { re: number; im: number; mag: number; phase_deg: number };
  gamma_after: { re: number; im: number; mag: number; phase_deg: number };
  vswr_before: number;
  vswr_after: number;
}

export function QuarterWaveModule() {
  const [zLoad, setZLoad] = useState(100.0);
  const [zLine, setZLine] = useState(50.0);
  const [frequency, setFrequency] = useState(1e9);

  const result: QWResult | null = useMemo(() => {
    try {
      return wasm.quarter_wave_match(zLoad, zLine, frequency) as QWResult;
    } catch { return null; }
  }, [zLoad, zLine, frequency]);

  const standingBefore = useMemo(() => {
    try {
      const lambda = 3e8 / frequency;
      return wasm.standing_wave_pattern(zLine, zLoad, 0, frequency, 2 * lambda, 300) as {
        z: number[]; v_mag: number[]; i_mag: number[];
      };
    } catch { return null; }
  }, [zLoad, zLine, frequency]);

  return (
    <div className="module">
      <h2>Quarter-Wave Transformer</h2>
      <p>Match a real load impedance to a transmission line using a λ/4 section with Z_QW = √(Z₀ × Z_L).</p>

      <div className="controls">
        <label>
          Z_Load (Ω): <input type="range" min={10} max={500} step={5} value={zLoad}
            onChange={e => setZLoad(+e.target.value)} /> {zLoad.toFixed(0)}
        </label>
        <label>
          Z₀ (Ω): <input type="range" min={25} max={150} step={5} value={zLine}
            onChange={e => setZLine(+e.target.value)} /> {zLine.toFixed(0)}
        </label>
        <label>
          Frequency: <input type="range" min={1e8} max={10e9} step={1e8} value={frequency}
            onChange={e => setFrequency(+e.target.value)} /> {(frequency / 1e9).toFixed(1)} GHz
        </label>
      </div>

      {result && (
        <div className="results-grid">
          <div className="result-card">
            <span className="label">Z_QW (transformer)</span>
            <span className="value">{result.z_transformer.toFixed(2)} Ω</span>
          </div>
          <div className="result-card">
            <span className="label">λ/4 Length</span>
            <span className="value">{(result.transformer_length * 100).toFixed(2)} cm</span>
          </div>
          <div className="result-card">
            <span className="label">VSWR (before)</span>
            <span className="value">{result.vswr_before.toFixed(2)}</span>
          </div>
          <div className="result-card">
            <span className="label">VSWR (after)</span>
            <span className="value">{result.vswr_after.toFixed(4)}</span>
          </div>
          <div className="result-card">
            <span className="label">|Γ| before</span>
            <span className="value">{result.gamma_before.mag.toFixed(4)}</span>
          </div>
          <div className="result-card">
            <span className="label">|Γ| after</span>
            <span className="value">{result.gamma_after.mag.toFixed(6)}</span>
          </div>
        </div>
      )}

      {standingBefore && (
        <Plot
          data={[{
            x: standingBefore.z.map(z => z * 100),
            y: standingBefore.v_mag,
            type: 'scatter',
            mode: 'lines',
            name: '|V(z)| unmatched',
            line: { color: '#F44336', width: 2 },
          }]}
          layout={{
            title: 'Standing Wave Pattern (Unmatched)',
            xaxis: { title: 'Distance from load (cm)' },
            yaxis: { title: '|V(z)| (normalized)' },
            margin: { t: 40, r: 20, b: 50, l: 60 },
            height: 350,
          }}
          config={{ responsive: true }}
          style={{ width: '100%' }}
        />
      )}

      {result && (
        <div className="transformer-diagram" style={{ textAlign: 'center', margin: '20px 0' }}>
          <svg viewBox="0 0 500 100" style={{ width: '100%', maxWidth: 600 }}>
            <rect x="20" y="30" width="150" height="40" fill="#E3F2FD" stroke="#2196F3" strokeWidth="2" rx="4" />
            <text x="95" y="55" textAnchor="middle" fontSize="13" fill="#333">Z₀ = {zLine}Ω</text>

            <rect x="170" y="30" width="120" height="40" fill="#FFF3E0" stroke="#FF9800" strokeWidth="2" rx="4" />
            <text x="230" y="48" textAnchor="middle" fontSize="12" fill="#333">Z_QW = {result.z_transformer.toFixed(1)}Ω</text>
            <text x="230" y="63" textAnchor="middle" fontSize="10" fill="#666">λ/4 = {(result.transformer_length * 100).toFixed(1)}cm</text>

            <rect x="290" y="30" width="120" height="40" fill="#FFEBEE" stroke="#F44336" strokeWidth="2" rx="4" />
            <text x="350" y="55" textAnchor="middle" fontSize="13" fill="#333">Z_L = {zLoad}Ω</text>

            <text x="95" y="90" textAnchor="middle" fontSize="11" fill="#666">Main line</text>
            <text x="230" y="90" textAnchor="middle" fontSize="11" fill="#666">Transformer</text>
            <text x="350" y="90" textAnchor="middle" fontSize="11" fill="#666">Load</text>
          </svg>
        </div>
      )}

      <div className="theory">
        <h3>Theory</h3>
        <p><strong>Transformer impedance:</strong> Z_QW = √(Z₀ · Z_L)</p>
        <p><strong>Length:</strong> ℓ = λ/4 = c/(4f)</p>
        <p>The quarter-wave section transforms Z_L to Z₀ at the design frequency:
          Z_in = Z_QW² / Z_L = Z₀</p>
        <p><strong>Limitation:</strong> Only works for real Z_L. For complex loads, first move to a real impedance point on the Smith chart.</p>
      </div>
    </div>
  );
}

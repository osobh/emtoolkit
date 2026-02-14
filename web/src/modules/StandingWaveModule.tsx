import { useState, useMemo } from 'react';
import Plot from 'react-plotly.js';
import { wasm } from '../wasm';

export function StandingWaveModule() {
  const [z0, setZ0] = useState(50);
  const [zlRe, setZlRe] = useState(100);
  const [zlIm, setZlIm] = useState(0);
  const [frequency, setFrequency] = useState(1e9);
  const [length, setLength] = useState(0.6);

  const data = useMemo(
    () => wasm.standing_wave_pattern(z0, zlRe, zlIm, frequency, length, 500),
    [z0, zlRe, zlIm, frequency, length],
  );

  return (
    <div className="module-panel">
      <h2>Standing Wave Pattern</h2>
      <div className="controls">
        <div className="control-group"><label>Z₀ (Ω)</label><input type="number" value={z0} onChange={e => setZ0(+e.target.value)} step={5} min={1} /></div>
        <div className="control-group"><label>Z_L Real (Ω)</label><input type="number" value={zlRe} onChange={e => setZlRe(+e.target.value)} step={5} /></div>
        <div className="control-group"><label>Z_L Imag (Ω)</label><input type="number" value={zlIm} onChange={e => setZlIm(+e.target.value)} step={5} /></div>
        <div className="control-group"><label>Freq (GHz)</label><input type="number" value={frequency / 1e9} onChange={e => setFrequency(+e.target.value * 1e9)} step={0.1} min={0.1} /></div>
        <div className="control-group"><label>Length (m)</label><input type="number" value={length} onChange={e => setLength(+e.target.value)} step={0.05} min={0.01} /></div>
      </div>
      <Plot
        data={[{
          x: data.positions, y: data.voltage_mag, mode: 'lines',
          line: { color: '#2196f3', width: 2 }, name: '|V(d)|',
        }]}
        layout={{
          width: 800, height: 400,
          xaxis: { title: { text: 'Distance from load (m)' } },
          yaxis: { title: { text: '|V| (normalized)' } },
          margin: { t: 20, b: 50, l: 60, r: 20 },
          paper_bgcolor: 'transparent', plot_bgcolor: 'white',
        }}
        config={{ responsive: true }}
      />
      <div className="result-box">
        <div className="result-grid">
          <div className="result-item"><span className="result-label">VSWR</span><span className="result-value">{data.vswr.toFixed(3)}</span></div>
          <div className="result-item"><span className="result-label">|Γ|</span><span className="result-value">{data.gamma_mag.toFixed(4)}</span></div>
          <div className="result-item"><span className="result-label">Vmax/Vmin</span><span className="result-value">{data.v_max.toFixed(3)} / {data.v_min.toFixed(3)}</span></div>
        </div>
      </div>
    </div>
  );
}

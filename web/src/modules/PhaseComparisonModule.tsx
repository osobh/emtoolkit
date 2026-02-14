import { useState, useMemo } from 'react';
import Plot from 'react-plotly.js';
import { wasm } from '../wasm';

export function PhaseComparisonModule() {
  const [a1, setA1] = useState(1.0);
  const [f1, setF1] = useState(1.0);
  const [p1, setP1] = useState(0);
  const [a2, setA2] = useState(1.0);
  const [f2, setF2] = useState(1.0);
  const [p2, setP2] = useState(Math.PI / 2);

  const tEnd = 4 / Math.min(f1, f2);
  const data = useMemo(
    () => wasm.phase_comparison(a1, f1, p1, a2, f2, p2, tEnd, 500),
    [a1, f1, p1, a2, f2, p2, tEnd],
  );

  return (
    <div className="module-panel">
      <h2>Phase Comparison — Two Sinusoids</h2>
      <div className="controls">
        <div className="control-group"><label>A₁</label><input type="number" value={a1} onChange={e => setA1(+e.target.value)} step={0.1} min={0} /></div>
        <div className="control-group"><label>f₁ (Hz)</label><input type="number" value={f1} onChange={e => setF1(+e.target.value)} step={0.5} min={0.1} /></div>
        <div className="control-group">
          <label>φ₁ (rad)</label>
          <input type="range" min={0} max={6.28} step={0.05} value={p1} onChange={e => setP1(+e.target.value)} />
          <span>{p1.toFixed(2)}</span>
        </div>
        <div className="control-group"><label>A₂</label><input type="number" value={a2} onChange={e => setA2(+e.target.value)} step={0.1} min={0} /></div>
        <div className="control-group"><label>f₂ (Hz)</label><input type="number" value={f2} onChange={e => setF2(+e.target.value)} step={0.5} min={0.1} /></div>
        <div className="control-group">
          <label>φ₂ (rad)</label>
          <input type="range" min={0} max={6.28} step={0.05} value={p2} onChange={e => setP2(+e.target.value)} />
          <span>{p2.toFixed(2)}</span>
        </div>
      </div>
      <Plot
        data={[
          { x: data.times, y: data.wave1, mode: 'lines', line: { color: '#2196f3', width: 2 }, name: 'Wave 1' },
          { x: data.times, y: data.wave2, mode: 'lines', line: { color: '#e63946', width: 2 }, name: 'Wave 2' },
        ]}
        layout={{
          width: 800, height: 400,
          xaxis: { title: { text: 'Time (s)' } },
          yaxis: { title: { text: 'Amplitude' } },
          margin: { t: 20, b: 50, l: 60, r: 20 },
          legend: { x: 0.02, y: 0.98 },
          paper_bgcolor: 'transparent', plot_bgcolor: 'white',
        }}
        config={{ responsive: true }}
      />
      <div className="result-box">
        <div className="result-grid">
          <div className="result-item"><span className="result-label">Δφ</span><span className="result-value">{data.phase_difference_deg.toFixed(1)}°</span></div>
          <div className="result-item"><span className="result-label">Lead/Lag</span><span className="result-value">{data.phase_difference_deg > 0 ? 'Wave 1 leads' : data.phase_difference_deg < 0 ? 'Wave 2 leads' : 'In phase'}</span></div>
        </div>
      </div>
    </div>
  );
}

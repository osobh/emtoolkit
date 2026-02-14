import { useState, useMemo } from 'react';
import Plot from 'react-plotly.js';
import { wasm } from '../wasm';

export function MethodOfImagesModule() {
  const [charge, setCharge] = useState(1e-9);
  const [height, setHeight] = useState(0.5);

  const data = useMemo(
    () => wasm.charge_above_plane(charge, height, -2, 2, 200),
    [charge, height],
  );

  return (
    <div className="module-panel">
      <h2>Method of Images — Charge Above Ground Plane</h2>
      <div className="controls">
        <div className="control-group"><label>Q (nC)</label><input type="number" value={charge * 1e9} onChange={e => setCharge(+e.target.value * 1e-9)} step={0.5} min={0.1} /></div>
        <div className="control-group">
          <label>Height (m)</label>
          <input type="range" min={0.1} max={2} step={0.05} value={height}
            onChange={e => setHeight(+e.target.value)} />
          <span>{height.toFixed(2)} m</span>
        </div>
      </div>

      <div style={{ display: 'flex', gap: 20, flexWrap: 'wrap' }}>
        <Plot
          data={[
            { x: data.x_positions, y: data.potential, mode: 'lines', line: { color: '#2196f3', width: 2 }, name: 'V(x, y=0)' },
          ]}
          layout={{
            width: 700, height: 350,
            xaxis: { title: { text: 'x (m)' } },
            yaxis: { title: { text: 'Potential (V)' } },
            margin: { t: 20, b: 50, l: 70, r: 20 },
            paper_bgcolor: 'transparent', plot_bgcolor: 'white',
          }}
          config={{ responsive: true }}
        />

        <div className="result-box" style={{ flex: 1, minWidth: 250 }}>
          <h3 style={{ marginTop: 0 }}>Configuration</h3>
          <p style={{ fontSize: '0.9rem', color: '#555' }}>
            Point charge Q = {(charge * 1e9).toFixed(1)} nC at height d = {height.toFixed(2)} m
            above an infinite conducting plane at y = 0.
          </p>
          <p style={{ fontSize: '0.9rem', color: '#555' }}>
            Image charge: −Q at y = −{height.toFixed(2)} m. The potential at the plane surface is zero (boundary condition satisfied).
          </p>
          <div className="result-grid">
            <div className="result-item"><span className="result-label">Force on Q</span><span className="result-value">{data.force.toExponential(3)} N</span></div>
            <div className="result-item"><span className="result-label">Direction</span><span className="result-value">Attractive (toward plane)</span></div>
          </div>
        </div>
      </div>
    </div>
  );
}

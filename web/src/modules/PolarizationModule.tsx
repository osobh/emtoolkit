import { useState, useMemo } from 'react';
import Plot from 'react-plotly.js';
import { wasm } from '../wasm';

export function PolarizationModule() {
  const [ax, setAx] = useState(1.0);
  const [ay, setAy] = useState(1.0);
  const [delta, setDelta] = useState(-90);

  const state = useMemo(() => {
    return wasm.polarization_state(ax, ay, delta, 200);
  }, [ax, ay, delta]);

  return (
    <div className="module-panel">
      <h2>Wave Polarization</h2>
      <div className="controls">
        <div className="control-group">
          <label>Aₓ</label>
          <input type="number" value={ax} onChange={e => setAx(+e.target.value)} step={0.1} min={0} />
        </div>
        <div className="control-group">
          <label>Aᵧ</label>
          <input type="number" value={ay} onChange={e => setAy(+e.target.value)} step={0.1} min={0} />
        </div>
        <div className="control-group">
          <label>δ (degrees)</label>
          <input type="range" min={-180} max={180} step={1} value={delta}
            onChange={e => setDelta(+e.target.value)} />
          <span>{delta}°</span>
        </div>
      </div>

      <div style={{ display: 'flex', gap: 20, flexWrap: 'wrap' }}>
        <Plot
          data={[{
            x: state.trace_x,
            y: state.trace_y,
            mode: 'lines',
            line: { color: '#e63946', width: 2 },
          }]}
          layout={{
            width: 400, height: 400, title: { text: 'Polarization Ellipse' },
            xaxis: { range: [-2, 2], scaleanchor: 'y', title: { text: 'Eₓ' } },
            yaxis: { range: [-2, 2], title: { text: 'Eᵧ' } },
            margin: { t: 40, b: 50, l: 50, r: 20 },
            paper_bgcolor: 'transparent', plot_bgcolor: 'white',
          }}
          config={{ responsive: true }}
        />

        <div className="result-box" style={{ flex: 1, minWidth: 250 }}>
          <h3 style={{ marginTop: 0 }}>Properties</h3>
          <div className="result-grid">
            <div className="result-item"><span className="result-label">Type</span><span className="result-value">{state.type}</span></div>
            <div className="result-item"><span className="result-label">Rotation</span><span className="result-value">{state.rotation}</span></div>
            <div className="result-item"><span className="result-label">Axial Ratio</span><span className="result-value">{state.axial_ratio === Infinity ? '∞' : state.axial_ratio.toFixed(3)}</span></div>
            <div className="result-item"><span className="result-label">Tilt Angle</span><span className="result-value">{state.tilt_angle_deg.toFixed(1)}°</span></div>
          </div>
          <h3>Stokes Parameters</h3>
          <div className="result-grid">
            <div className="result-item"><span className="result-label">S₀</span><span className="result-value">{state.stokes[0].toFixed(3)}</span></div>
            <div className="result-item"><span className="result-label">S₁</span><span className="result-value">{state.stokes[1].toFixed(3)}</span></div>
            <div className="result-item"><span className="result-label">S₂</span><span className="result-value">{state.stokes[2].toFixed(3)}</span></div>
            <div className="result-item"><span className="result-label">S₃</span><span className="result-value">{state.stokes[3].toFixed(3)}</span></div>
          </div>
        </div>
      </div>
    </div>
  );
}

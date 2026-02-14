import { useState, useMemo } from 'react';
import Plot from 'react-plotly.js';
import * as wasm from '../lib/em_wasm';

const PRESETS: Record<string, { label: string; desc: string; divSign: string; curlSign: string }> = {
  radial: { label: 'Radial (Source)', desc: 'F = r̂·r — outward flow from origin', divSign: '> 0 (source)', curlSign: '= 0 (irrotational)' },
  vortex: { label: 'Vortex (Curl)', desc: 'F = φ̂/r — circular flow', divSign: '= 0 (incompressible)', curlSign: '≠ 0 at origin' },
  uniform: { label: 'Uniform', desc: 'F = x̂ — constant flow', divSign: '= 0', curlSign: '= 0' },
  dipole: { label: 'Dipole', desc: 'Two opposite charges', divSign: '> 0 near +, < 0 near -', curlSign: '= 0 (electrostatic)' },
};

export function DivCurlModule() {
  const [preset, setPreset] = useState('radial');
  const [nx, setNx] = useState(15);

  const fieldData = useMemo(() => {
    try {
      return wasm.vector_field_2d(preset, -3, 3, -3, 3, nx, nx) as {
        x: number[]; y: number[]; fx: number[]; fy: number[];
        div?: number[]; curl?: number[];
      };
    } catch { return null; }
  }, [preset, nx]);

  const scalarData = useMemo(() => {
    try {
      return wasm.scalar_field_2d(preset, -3, 3, -3, 3, 40, 40) as {
        x: number[]; y: number[]; values: number[];
      };
    } catch { return null; }
  }, [preset]);

  const info = PRESETS[preset];

  return (
    <div className="module">
      <h2>Divergence & Curl Visualizer</h2>
      <p>See how divergence measures sources/sinks and curl measures rotation in vector fields.</p>

      <div className="controls">
        <label>
          Field preset:
          <select value={preset} onChange={e => setPreset(e.target.value)}>
            {Object.entries(PRESETS).map(([k, v]) => (
              <option key={k} value={k}>{v.label}</option>
            ))}
          </select>
        </label>
        <label>Grid density: <input type="range" min={8} max={25} step={1} value={nx}
          onChange={e => setNx(+e.target.value)} /> {nx}×{nx}</label>
      </div>

      <div className="results-grid">
        <div className="result-card">
          <span className="label">Field</span>
          <span className="value">{info.desc}</span>
        </div>
        <div className="result-card">
          <span className="label">∇·F (divergence)</span>
          <span className="value">{info.divSign}</span>
        </div>
        <div className="result-card">
          <span className="label">∇×F (curl)</span>
          <span className="value">{info.curlSign}</span>
        </div>
      </div>

      {fieldData && (
        <Plot
          data={[{
            type: 'scatter',
            mode: 'markers',
            x: fieldData.x,
            y: fieldData.y,
            marker: {
              size: 8,
              symbol: 'arrow',
              angle: fieldData.fx.map((fx, i) => -Math.atan2(fieldData.fy[i], fx) * 180 / Math.PI),
              color: fieldData.fx.map((fx, i) =>
                Math.sqrt(fx * fx + fieldData.fy[i] * fieldData.fy[i])
              ),
              colorscale: 'Viridis',
              showscale: true,
              colorbar: { title: '|F|' },
            },
            hoverinfo: 'text',
            text: fieldData.fx.map((fx, i) =>
              `F = (${fx.toFixed(2)}, ${fieldData.fy[i].toFixed(2)})`
            ),
          } as Plotly.Data]}
          layout={{
            title: `Vector Field: ${info.label}`,
            xaxis: { title: 'x', range: [-3.5, 3.5], scaleanchor: 'y' },
            yaxis: { title: 'y', range: [-3.5, 3.5] },
            margin: { t: 40, r: 60, b: 50, l: 50 },
            height: 450,
            width: 500,
          }}
          config={{ responsive: true }}
        />
      )}

      <div className="theory">
        <h3>Divergence & Curl</h3>
        <p><strong>Divergence:</strong> ∇·F = ∂Fx/∂x + ∂Fy/∂y + ∂Fz/∂z</p>
        <p>Measures the net outward flux per unit volume — positive = source, negative = sink, zero = incompressible</p>
        <p><strong>Curl:</strong> ∇×F = (∂Fz/∂y − ∂Fy/∂z)x̂ + ...</p>
        <p>Measures circulation density — nonzero curl means the field has rotational tendency</p>
        <h3>Theorems</h3>
        <p><strong>Divergence theorem:</strong> ∮ F·dS = ∫∫∫ (∇·F) dV</p>
        <p><strong>Stokes' theorem:</strong> ∮ F·dl = ∫∫ (∇×F)·dS</p>
      </div>
    </div>
  );
}

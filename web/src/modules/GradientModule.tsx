import { useState, useMemo } from 'react';
import { wasm } from '../wasm';

const PRESETS = ['gaussian', 'saddle', 'dipole_potential', 'ridge', 'sine_product', 'cone'];

export function GradientModule() {
  const [preset, setPreset] = useState('gaussian');
  const [x, setX] = useState(1.0);
  const [y, setY] = useState(0.5);
  const [z, setZ] = useState(0);

  const grad = useMemo(
    () => wasm.numerical_gradient(preset, x, y, z),
    [preset, x, y, z],
  );

  return (
    <div className="module-panel">
      <h2>Numerical Gradient, Divergence &amp; Curl</h2>
      <div className="controls">
        <div className="control-group">
          <label>Scalar Field</label>
          <select value={preset} onChange={e => setPreset(e.target.value)}>
            {PRESETS.map(p => <option key={p} value={p}>{p}</option>)}
          </select>
        </div>
        <div className="control-group"><label>x</label><input type="number" value={x} onChange={e => setX(+e.target.value)} step={0.25} /></div>
        <div className="control-group"><label>y</label><input type="number" value={y} onChange={e => setY(+e.target.value)} step={0.25} /></div>
        <div className="control-group"><label>z</label><input type="number" value={z} onChange={e => setZ(+e.target.value)} step={0.25} /></div>
      </div>

      <div className="result-box">
        <h3 style={{ marginTop: 0 }}>∇f at ({x}, {y}, {z})</h3>
        <div className="result-grid">
          <div className="result-item"><span className="result-label">f(x,y,z)</span><span className="result-value">{grad.value.toFixed(6)}</span></div>
          <div className="result-item"><span className="result-label">∂f/∂x</span><span className="result-value">{grad.grad_x.toFixed(6)}</span></div>
          <div className="result-item"><span className="result-label">∂f/∂y</span><span className="result-value">{grad.grad_y.toFixed(6)}</span></div>
          <div className="result-item"><span className="result-label">∂f/∂z</span><span className="result-value">{grad.grad_z.toFixed(6)}</span></div>
          <div className="result-item"><span className="result-label">|∇f|</span><span className="result-value">{grad.grad_mag.toFixed(6)}</span></div>
        </div>
        <p style={{ fontSize: '0.85rem', color: '#666', marginTop: 12 }}>
          The gradient points in the direction of steepest ascent. Its magnitude gives the rate of change in that direction.
          Computed numerically using central differences (h = 10⁻⁶).
        </p>
      </div>
    </div>
  );
}

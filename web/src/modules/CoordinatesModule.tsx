import { useState, useMemo } from 'react';
import { wasm } from '../wasm';

export function CoordinatesModule() {
  const [mode, setMode] = useState<'c2s' | 's2c'>('c2s');
  const [x, setX] = useState(1);
  const [y, setY] = useState(1);
  const [z, setZ] = useState(1);
  const [r, setR] = useState(1.732);
  const [theta, setTheta] = useState(0.955);
  const [phi, setPhi] = useState(0.785);

  const spherical = useMemo(
    () => mode === 'c2s' ? wasm.cartesian_to_spherical(x, y, z) : null,
    [mode, x, y, z],
  );

  const cartesian = useMemo(
    () => mode === 's2c' ? wasm.spherical_to_cartesian(r, theta, phi) : null,
    [mode, r, theta, phi],
  );

  return (
    <div className="module-panel">
      <h2>Coordinate System Converter</h2>
      <div className="controls">
        <div className="control-group">
          <label>Direction</label>
          <select value={mode} onChange={e => setMode(e.target.value as 'c2s' | 's2c')}>
            <option value="c2s">Cartesian → Spherical</option>
            <option value="s2c">Spherical → Cartesian</option>
          </select>
        </div>
      </div>

      {mode === 'c2s' && (
        <>
          <div className="controls">
            <div className="control-group"><label>x</label><input type="number" value={x} onChange={e => setX(+e.target.value)} step={0.5} /></div>
            <div className="control-group"><label>y</label><input type="number" value={y} onChange={e => setY(+e.target.value)} step={0.5} /></div>
            <div className="control-group"><label>z</label><input type="number" value={z} onChange={e => setZ(+e.target.value)} step={0.5} /></div>
          </div>
          {spherical && (
            <div className="result-box">
              <div className="result-grid">
                <div className="result-item"><span className="result-label">r</span><span className="result-value">{spherical.r.toFixed(6)}</span></div>
                <div className="result-item"><span className="result-label">θ (rad)</span><span className="result-value">{spherical.theta.toFixed(6)}</span></div>
                <div className="result-item"><span className="result-label">θ (deg)</span><span className="result-value">{(spherical.theta * 180 / Math.PI).toFixed(3)}°</span></div>
                <div className="result-item"><span className="result-label">φ (rad)</span><span className="result-value">{spherical.phi.toFixed(6)}</span></div>
                <div className="result-item"><span className="result-label">φ (deg)</span><span className="result-value">{(spherical.phi * 180 / Math.PI).toFixed(3)}°</span></div>
              </div>
            </div>
          )}
        </>
      )}

      {mode === 's2c' && (
        <>
          <div className="controls">
            <div className="control-group"><label>r</label><input type="number" value={r} onChange={e => setR(+e.target.value)} step={0.1} min={0} /></div>
            <div className="control-group"><label>θ (rad)</label><input type="number" value={theta} onChange={e => setTheta(+e.target.value)} step={0.1} /></div>
            <div className="control-group"><label>φ (rad)</label><input type="number" value={phi} onChange={e => setPhi(+e.target.value)} step={0.1} /></div>
          </div>
          {cartesian && (
            <div className="result-box">
              <div className="result-grid">
                <div className="result-item"><span className="result-label">x</span><span className="result-value">{cartesian.x.toFixed(6)}</span></div>
                <div className="result-item"><span className="result-label">y</span><span className="result-value">{cartesian.y.toFixed(6)}</span></div>
                <div className="result-item"><span className="result-label">z</span><span className="result-value">{cartesian.z.toFixed(6)}</span></div>
              </div>
            </div>
          )}
        </>
      )}
    </div>
  );
}

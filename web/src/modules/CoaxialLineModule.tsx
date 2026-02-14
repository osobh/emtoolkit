import { useState, useMemo } from 'react';
import { wasm } from '../wasm';

export function CoaxialLineModule() {
  const [innerR, setInnerR] = useState(0.5);
  const [outerR, setOuterR] = useState(2.3);
  const [epsilonR, setEpsilonR] = useState(2.25);
  const [frequency, setFrequency] = useState(1e9);

  const data = useMemo(
    () => wasm.coaxial_line_params(innerR / 1000, outerR / 1000, epsilonR, frequency),
    [innerR, outerR, epsilonR, frequency],
  );

  return (
    <div className="module-panel">
      <h2>Coaxial Transmission Line Parameters</h2>
      <div className="controls">
        <div className="control-group"><label>Inner radius (mm)</label><input type="number" value={innerR} onChange={e => setInnerR(+e.target.value)} step={0.1} min={0.1} /></div>
        <div className="control-group"><label>Outer radius (mm)</label><input type="number" value={outerR} onChange={e => setOuterR(+e.target.value)} step={0.1} min={0.2} /></div>
        <div className="control-group"><label>εᵣ (dielectric)</label><input type="number" value={epsilonR} onChange={e => setEpsilonR(+e.target.value)} step={0.25} min={1} /></div>
        <div className="control-group"><label>Freq (GHz)</label><input type="number" value={frequency / 1e9} onChange={e => setFrequency(+e.target.value * 1e9)} step={0.1} min={0.001} /></div>
      </div>

      <div className="result-box">
        <h3 style={{ marginTop: 0 }}>Line Parameters</h3>
        <div className="result-grid">
          <div className="result-item"><span className="result-label">Z₀ (lossless)</span><span className="result-value">{data.z0_lossless.toFixed(2)} Ω</span></div>
          <div className="result-item"><span className="result-label">L per meter</span><span className="result-value">{(data.l_per_m * 1e9).toFixed(3)} nH/m</span></div>
          <div className="result-item"><span className="result-label">C per meter</span><span className="result-value">{(data.c_per_m * 1e12).toFixed(3)} pF/m</span></div>
          <div className="result-item"><span className="result-label">b/a ratio</span><span className="result-value">{(outerR / innerR).toFixed(3)}</span></div>
        </div>

        <div style={{ marginTop: 16, padding: 12, background: '#f0f4f8', borderRadius: 6 }}>
          <h4 style={{ margin: '0 0 8px 0', fontSize: '0.9rem' }}>Common Cables</h4>
          <table style={{ width: '100%', fontSize: '0.85rem', borderCollapse: 'collapse' }}>
            <thead><tr style={{ textAlign: 'left', borderBottom: '1px solid #ccc' }}>
              <th>Type</th><th>Z₀</th><th>Inner</th><th>Outer</th><th>εᵣ</th>
            </tr></thead>
            <tbody>
              <tr><td>RG-58</td><td>50 Ω</td><td>0.45mm</td><td>1.47mm</td><td>2.25</td></tr>
              <tr><td>RG-59</td><td>75 Ω</td><td>0.32mm</td><td>1.84mm</td><td>2.25</td></tr>
              <tr><td>RG-174</td><td>50 Ω</td><td>0.25mm</td><td>0.84mm</td><td>2.25</td></tr>
            </tbody>
          </table>
        </div>
      </div>
    </div>
  );
}

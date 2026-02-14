import { useState, useMemo } from 'react';
import { wasm } from '../wasm';

export function WireForceModule() {
  const [i1, setI1] = useState(10);
  const [i2, setI2] = useState(10);
  const [separation, setSeparation] = useState(0.1);
  const [length, setLength] = useState(1.0);

  const result = useMemo(
    () => wasm.parallel_wire_force(i1, i2, separation, length),
    [i1, i2, separation, length],
  );

  return (
    <div className="module-panel">
      <h2>Force Between Parallel Current-Carrying Wires</h2>
      <div className="controls">
        <div className="control-group"><label>I₁ (A)</label><input type="number" value={i1} onChange={e => setI1(+e.target.value)} step={1} /></div>
        <div className="control-group"><label>I₂ (A)</label><input type="number" value={i2} onChange={e => setI2(+e.target.value)} step={1} /></div>
        <div className="control-group"><label>Separation (m)</label><input type="number" value={separation} onChange={e => setSeparation(+e.target.value)} step={0.01} min={0.001} /></div>
        <div className="control-group"><label>Length (m)</label><input type="number" value={length} onChange={e => setLength(+e.target.value)} step={0.1} min={0.01} /></div>
      </div>
      <div className="result-box">
        <div className="result-grid">
          <div className="result-item"><span className="result-label">Force</span><span className="result-value">{result.force.toExponential(3)} N</span></div>
          <div className="result-item"><span className="result-label">Force/length</span><span className="result-value">{result.force_per_length.toExponential(3)} N/m</span></div>
          <div className="result-item"><span className="result-label">Nature</span><span className="result-value">{result.is_attractive ? 'Attractive (same direction)' : 'Repulsive (opposite direction)'}</span></div>
        </div>
        <p style={{ fontSize: '0.85rem', color: '#666', marginTop: 12 }}>
          Two parallel wires carrying currents in the {result.is_attractive ? 'same' : 'opposite'} direction
          experience a{result.is_attractive ? 'n attractive' : ' repulsive'} force.
          This is the basis of the SI definition of the ampere.
        </p>
      </div>
    </div>
  );
}

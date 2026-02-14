import { useState, useMemo } from 'react';
import { wasm } from '../wasm';

export function ConstantsModule() {
  const constants = useMemo(() => wasm.get_constants(), []);
  const [frequency, setFrequency] = useState(1e9);

  const lambda = useMemo(() => wasm.wavelength(frequency), [frequency]);
  const k = useMemo(() => wasm.wavenumber(frequency), [frequency]);

  const [linear, setLinear] = useState(100);
  const dbVal = useMemo(() => wasm.power_to_db(linear), [linear]);

  const [dbInput, setDbInput] = useState(20);
  const linVal = useMemo(() => wasm.db_to_power(dbInput), [dbInput]);

  return (
    <div className="module-panel">
      <h2>EM Constants &amp; Unit Conversions</h2>

      <div className="result-box">
        <h3 style={{ marginTop: 0 }}>Fundamental Constants</h3>
        <div className="result-grid">
          <div className="result-item"><span className="result-label">c₀ (speed of light)</span><span className="result-value">{constants.c0.toExponential(6)} m/s</span></div>
          <div className="result-item"><span className="result-label">μ₀ (permeability)</span><span className="result-value">{constants.mu0.toExponential(6)} H/m</span></div>
          <div className="result-item"><span className="result-label">ε₀ (permittivity)</span><span className="result-value">{constants.epsilon0.toExponential(6)} F/m</span></div>
          <div className="result-item"><span className="result-label">η₀ (impedance)</span><span className="result-value">{constants.eta0.toFixed(4)} Ω</span></div>
        </div>
      </div>

      <div className="result-box" style={{ marginTop: 16 }}>
        <h3 style={{ marginTop: 0 }}>Wavelength &amp; Wavenumber Calculator</h3>
        <div className="controls">
          <div className="control-group"><label>Frequency (GHz)</label><input type="number" value={frequency / 1e9} onChange={e => setFrequency(+e.target.value * 1e9)} step={0.1} min={0.001} /></div>
        </div>
        <div className="result-grid" style={{ marginTop: 8 }}>
          <div className="result-item"><span className="result-label">λ</span><span className="result-value">{lambda < 1 ? (lambda * 100).toFixed(3) + ' cm' : lambda.toFixed(4) + ' m'}</span></div>
          <div className="result-item"><span className="result-label">k (wavenumber)</span><span className="result-value">{k.toFixed(3)} rad/m</span></div>
        </div>
      </div>

      <div className="result-box" style={{ marginTop: 16 }}>
        <h3 style={{ marginTop: 0 }}>dB Conversions</h3>
        <div className="controls">
          <div className="control-group"><label>Linear → dB</label><input type="number" value={linear} onChange={e => setLinear(+e.target.value)} step={10} min={0.001} /></div>
          <div className="control-group"><label>= </label><span style={{ fontFamily: 'monospace', fontSize: '1.1rem' }}>{dbVal.toFixed(3)} dB</span></div>
        </div>
        <div className="controls" style={{ marginTop: 8 }}>
          <div className="control-group"><label>dB → Linear</label><input type="number" value={dbInput} onChange={e => setDbInput(+e.target.value)} step={1} /></div>
          <div className="control-group"><label>= </label><span style={{ fontFamily: 'monospace', fontSize: '1.1rem' }}>{linVal.toExponential(4)}</span></div>
        </div>
      </div>
    </div>
  );
}

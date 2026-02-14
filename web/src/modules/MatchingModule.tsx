import { useState, useMemo } from 'react';
import { wasm } from '../wasm';

export function MatchingModule() {
  const [matchType, setMatchType] = useState<'quarter' | 'stub'>('quarter');
  const [zLoad, setZLoad] = useState(100);
  const [zlIm, setZlIm] = useState(0);
  const [z0, setZ0] = useState(50);
  const [frequency, setFrequency] = useState(1e9);
  const [useShort, setUseShort] = useState(true);

  const qwResult = useMemo(
    () => matchType === 'quarter' ? wasm.quarter_wave_match(zLoad, z0, frequency) : null,
    [matchType, zLoad, z0, frequency],
  );

  const stubResult = useMemo(
    () => matchType === 'stub' ? wasm.single_stub_match(zLoad, zlIm, z0, useShort) : null,
    [matchType, zLoad, zlIm, z0, useShort],
  );

  return (
    <div className="module-panel">
      <h2>Impedance Matching</h2>
      <div className="controls">
        <div className="control-group">
          <label>Method</label>
          <select value={matchType} onChange={e => setMatchType(e.target.value as 'quarter' | 'stub')}>
            <option value="quarter">Quarter-Wave Transformer</option>
            <option value="stub">Single Stub</option>
          </select>
        </div>
        <div className="control-group"><label>Z_L Real (Ω)</label><input type="number" value={zLoad} onChange={e => setZLoad(+e.target.value)} step={5} min={1} /></div>
        {matchType === 'stub' && (
          <div className="control-group"><label>Z_L Imag (Ω)</label><input type="number" value={zlIm} onChange={e => setZlIm(+e.target.value)} step={5} /></div>
        )}
        <div className="control-group"><label>Z₀ (Ω)</label><input type="number" value={z0} onChange={e => setZ0(+e.target.value)} step={5} min={1} /></div>
        {matchType === 'quarter' && (
          <div className="control-group"><label>Freq (GHz)</label><input type="number" value={frequency / 1e9} onChange={e => setFrequency(+e.target.value * 1e9)} step={0.1} min={0.1} /></div>
        )}
        {matchType === 'stub' && (
          <div className="control-group">
            <label>Stub Type</label>
            <select value={useShort ? 'short' : 'open'} onChange={e => setUseShort(e.target.value === 'short')}>
              <option value="short">Short-circuited</option>
              <option value="open">Open-circuited</option>
            </select>
          </div>
        )}
      </div>

      <div className="result-box">
        {matchType === 'quarter' && qwResult && (
          <div className="result-grid">
            <div className="result-item"><span className="result-label">Transformer Z₀</span><span className="result-value">{qwResult.z_transformer.toFixed(2)} Ω</span></div>
            <div className="result-item"><span className="result-label">Section Length</span><span className="result-value">{(qwResult.length_m * 100).toFixed(2)} cm</span></div>
            <div className="result-item"><span className="result-label">λ/4</span><span className="result-value">{(qwResult.wavelength * 100).toFixed(2)} cm</span></div>
          </div>
        )}
        {matchType === 'stub' && stubResult && (
          <div className="result-grid">
            <div className="result-item"><span className="result-label">Stub distance (d/λ)</span><span className="result-value">{stubResult.d_over_lambda.toFixed(4)}</span></div>
            <div className="result-item"><span className="result-label">Stub length (l/λ)</span><span className="result-value">{stubResult.l_over_lambda.toFixed(4)}</span></div>
            <div className="result-item"><span className="result-label">Solution 2: d/λ</span><span className="result-value">{stubResult.d2_over_lambda?.toFixed(4) ?? '—'}</span></div>
            <div className="result-item"><span className="result-label">Solution 2: l/λ</span><span className="result-value">{stubResult.l2_over_lambda?.toFixed(4) ?? '—'}</span></div>
          </div>
        )}
      </div>
    </div>
  );
}

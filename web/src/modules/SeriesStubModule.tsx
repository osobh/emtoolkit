import { useState, useMemo } from 'react';
import * as wasm from '../lib/em_wasm';

interface SeriesStubSolution {
  stub_distance_wavelengths: number;
  stub_length_wavelengths: number;
}

export function SeriesStubModule() {
  const [zlRe, setZlRe] = useState(25.0);
  const [zlIm, setZlIm] = useState(50.0);
  const [z0, setZ0] = useState(50.0);
  const [useShort, setUseShort] = useState(true);

  const result: SeriesStubSolution[] | null = useMemo(() => {
    try {
      return wasm.series_stub_match(zlRe, zlIm, z0, useShort) as SeriesStubSolution[];
    } catch {
      return null;
    }
  }, [zlRe, zlIm, z0, useShort]);

  return (
    <div className="module">
      <h2>Series Stub Matching</h2>
      <p>
        Match a complex load using a single stub in <strong>series</strong> with the transmission line.
        Unlike shunt stub matching (which uses admittances), series stub matching works with impedances
        to find where the normalized resistance equals 1.
      </p>

      <div className="controls">
        <label>
          Z_L Real (Ω): <input type="range" min={5} max={200} step={5} value={zlRe}
            onChange={e => setZlRe(+e.target.value)} /> {zlRe.toFixed(0)}
        </label>
        <label>
          Z_L Imag (Ω): <input type="range" min={-100} max={100} step={5} value={zlIm}
            onChange={e => setZlIm(+e.target.value)} /> {zlIm >= 0 ? '+' : ''}{zlIm.toFixed(0)}j
        </label>
        <label>
          Z₀ (Ω): <input type="range" min={25} max={150} step={5} value={z0}
            onChange={e => setZ0(+e.target.value)} /> {z0.toFixed(0)}
        </label>
        <label>
          Stub Type:
          <select value={useShort ? 'short' : 'open'} onChange={e => setUseShort(e.target.value === 'short')}>
            <option value="short">Short Circuit</option>
            <option value="open">Open Circuit</option>
          </select>
        </label>
      </div>

      {result && result.length > 0 && (
        <div className="results-grid">
          <h3>Solution 1</h3>
          <div className="result-card">
            <span className="label">Distance to Stub</span>
            <span className="value">{result[0].stub_distance_wavelengths.toFixed(4)} λ</span>
          </div>
          <div className="result-card">
            <span className="label">Stub Length</span>
            <span className="value">{result[0].stub_length_wavelengths.toFixed(4)} λ</span>
          </div>
          
          <h3>Solution 2</h3>
          <div className="result-card">
            <span className="label">Distance to Stub</span>
            <span className="value">{result[1].stub_distance_wavelengths.toFixed(4)} λ</span>
          </div>
          <div className="result-card">
            <span className="label">Stub Length</span>
            <span className="value">{result[1].stub_length_wavelengths.toFixed(4)} λ</span>
          </div>
        </div>
      )}

      <div className="series-stub-diagram" style={{ textAlign: 'center', margin: '20px 0' }}>
        <svg viewBox="0 0 500 120" style={{ width: '100%', maxWidth: 600 }}>
          {/* Main line to stub point */}
          <rect x="20" y="50" width="180" height="10" fill="#2196F3" />
          
          {/* Series stub (breaks the line) */}
          <rect x="200" y="30" width="8" height="50" fill="#4CAF50" />
          <text x="204" y="95" textAnchor="middle" fontSize="10" fill="#666">Series Stub</text>
          
          {/* Line continues after stub */}
          <rect x="208" y="50" width="150" height="10" fill="#2196F3" />
          
          {/* Load */}
          <rect x="358" y="30" width="50" height="50" fill="#FFEBEE" stroke="#F44336" strokeWidth="2" rx="4" />
          <text x="383" y="60" textAnchor="middle" fontSize="12" fill="#333">Z_L</text>
          
          {/* Distance indicator */}
          <line x1="358" y1="20" x2="200" y2="20" stroke="#666" strokeDasharray="4" />
          <text x="279" y="15" textAnchor="middle" fontSize="10" fill="#666">d from load</text>
          
          {/* Source */}
          <circle cx="40" cy="55" r="15" fill="#E3F2FD" stroke="#2196F3" strokeWidth="2" />
          <text x="40" y="60" textAnchor="middle" fontSize="12" fill="#333">Z₀</text>
          
          {/* Series connection indicator */}
          <text x="204" y="25" textAnchor="middle" fontSize="9" fill="#4CAF50">IN SERIES</text>
        </svg>
      </div>

      <div className="theory">
        <h3>Theory</h3>
        <p><strong>Series stub matching</strong> places a stub in series with the transmission line, 
        as opposed to the more common shunt (parallel) stub configuration.</p>
        
        <p><strong>Design approach:</strong></p>
        <ul>
          <li>Find distance d where normalized resistance r(d) = 1</li>
          <li>At this point, Z_in = Z₀(1 + jx)</li>
          <li>Series stub provides reactance -jxZ₀ to cancel</li>
        </ul>
        
        <p><strong>Stub impedances:</strong></p>
        <ul>
          <li>Short-circuit stub: Z = jZ₀·tan(βl)</li>
          <li>Open-circuit stub: Z = -jZ₀·cot(βl)</li>
        </ul>
        
        <p><strong>When to use:</strong> Series stubs are preferred when the line can be broken 
        (e.g., slotline, series connection possible) or when shunt connection is impractical.</p>
      </div>
    </div>
  );
}

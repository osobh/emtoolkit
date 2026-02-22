import { useState, useMemo } from 'react';
import * as wasm from '../lib/em_wasm';

interface DoubleStubSolution {
  stub1_length_wavelengths: number;
  stub2_length_wavelengths: number;
}

interface DoubleStubResponse {
  success: boolean;
  solutions?: DoubleStubSolution[];
  error?: string;
}

export function DoubleStubModule() {
  const [zlRe, setZlRe] = useState(25.0);
  const [zlIm, setZlIm] = useState(25.0);
  const [z0, setZ0] = useState(50.0);
  const [stubSeparation, setStubSeparation] = useState(0.125);
  const [useShort, setUseShort] = useState(true);

  const result: DoubleStubResponse | null = useMemo(() => {
    try {
      return wasm.double_stub_match(zlRe, zlIm, z0, stubSeparation, useShort) as DoubleStubResponse;
    } catch {
      return null;
    }
  }, [zlRe, zlIm, z0, stubSeparation, useShort]);

  return (
    <div className="module">
      <h2>Double Stub Matching</h2>
      <p>
        Match a complex load using two stubs at fixed separation. The first stub is at the load.
        This technique is useful when the stub positions are constrained.
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
          Stub Separation (λ): <input type="range" min={0.0625} max={0.5} step={0.0625} value={stubSeparation}
            onChange={e => setStubSeparation(+e.target.value)} /> {stubSeparation.toFixed(4)}
        </label>
        <label>
          Stub Type:
          <select value={useShort ? 'short' : 'open'} onChange={e => setUseShort(e.target.value === 'short')}>
            <option value="short">Short Circuit</option>
            <option value="open">Open Circuit</option>
          </select>
        </label>
      </div>

      {result && result.success && result.solutions && (
        <div className="results-grid">
          <h3>Solution 1</h3>
          <div className="result-card">
            <span className="label">Stub 1 Length</span>
            <span className="value">{result.solutions[0].stub1_length_wavelengths.toFixed(4)} λ</span>
          </div>
          <div className="result-card">
            <span className="label">Stub 2 Length</span>
            <span className="value">{result.solutions[0].stub2_length_wavelengths.toFixed(4)} λ</span>
          </div>
          
          <h3>Solution 2</h3>
          <div className="result-card">
            <span className="label">Stub 1 Length</span>
            <span className="value">{result.solutions[1].stub1_length_wavelengths.toFixed(4)} λ</span>
          </div>
          <div className="result-card">
            <span className="label">Stub 2 Length</span>
            <span className="value">{result.solutions[1].stub2_length_wavelengths.toFixed(4)} λ</span>
          </div>
        </div>
      )}

      {result && !result.success && (
        <div className="error-message" style={{ color: '#F44336', padding: '1rem', background: '#FFEBEE', borderRadius: '4px', margin: '1rem 0' }}>
          <strong>Cannot Match:</strong> {result.error}
          <p style={{ marginTop: '0.5rem', fontSize: '0.9em' }}>
            Try adjusting the stub separation or load impedance. Certain load admittances fall in a "forbidden region" 
            where double-stub matching is not possible with the given separation.
          </p>
        </div>
      )}

      <div className="double-stub-diagram" style={{ textAlign: 'center', margin: '20px 0' }}>
        <svg viewBox="0 0 500 150" style={{ width: '100%', maxWidth: 600 }}>
          {/* Main line */}
          <rect x="20" y="60" width="400" height="10" fill="#2196F3" />
          
          {/* Load */}
          <rect x="380" y="40" width="40" height="50" fill="#FFEBEE" stroke="#F44336" strokeWidth="2" rx="4" />
          <text x="400" y="70" textAnchor="middle" fontSize="12" fill="#333">Z_L</text>
          
          {/* Stub 1 (at load) */}
          <rect x="340" y="70" width="8" height="50" fill="#4CAF50" />
          <text x="344" y="135" textAnchor="middle" fontSize="10" fill="#666">Stub 1</text>
          
          {/* Stub 2 */}
          <rect x="200" y="70" width="8" height="50" fill="#FF9800" />
          <text x="204" y="135" textAnchor="middle" fontSize="10" fill="#666">Stub 2</text>
          
          {/* Separation indicator */}
          <line x1="208" y1="50" x2="340" y2="50" stroke="#666" strokeDasharray="4" />
          <text x="274" y="45" textAnchor="middle" fontSize="10" fill="#666">d = {stubSeparation.toFixed(3)}λ</text>
          
          {/* Source */}
          <circle cx="40" cy="65" r="15" fill="#E3F2FD" stroke="#2196F3" strokeWidth="2" />
          <text x="40" y="70" textAnchor="middle" fontSize="12" fill="#333">Z₀</text>
        </svg>
      </div>

      <div className="theory">
        <h3>Theory</h3>
        <p><strong>Double-stub matching</strong> uses two shunt stubs at fixed separation to match any load 
        that doesn't fall in the "forbidden region."</p>
        <p><strong>Stub separation:</strong> Common values are λ/8 (0.125λ) or 3λ/8 (0.375λ).</p>
        <p><strong>Forbidden region:</strong> Loads with normalized conductance g_L &gt; 1/sin²(βd) cannot be matched.</p>
        <p><strong>Advantage:</strong> No line section to load needed — both stubs can be at fixed positions.</p>
      </div>
    </div>
  );
}

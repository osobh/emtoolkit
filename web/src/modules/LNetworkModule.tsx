import { useState, useMemo } from 'react';
import * as wasm from '../lib/em_wasm';

interface LNetworkSolution {
  topology: string;
  series_value: number;
  series_type: string;
  shunt_value: number;
  shunt_type: string;
  q_factor: number;
  bandwidth_fractional: number;
  description: string;
}

export function LNetworkModule() {
  const [zSource, setZSource] = useState(50.0);
  const [zlRe, setZlRe] = useState(25.0);
  const [zlIm, setZlIm] = useState(30.0);
  const [frequency, setFrequency] = useState(1e9);

  const result: LNetworkSolution[] | null = useMemo(() => {
    try {
      return wasm.l_network_match(zSource, zlRe, zlIm, frequency) as LNetworkSolution[];
    } catch {
      return null;
    }
  }, [zSource, zlRe, zlIm, frequency]);

  const formatValue = (value: number, type: string): string => {
    if (type.includes('Inductor')) {
      if (value < 1e-9) return `${(value * 1e12).toFixed(2)} pH`;
      if (value < 1e-6) return `${(value * 1e9).toFixed(2)} nH`;
      return `${(value * 1e6).toFixed(2)} µH`;
    } else if (type.includes('Capacitor')) {
      if (value < 1e-12) return `${(value * 1e15).toFixed(2)} fF`;
      if (value < 1e-9) return `${(value * 1e12).toFixed(2)} pF`;
      return `${(value * 1e9).toFixed(2)} nF`;
    }
    return `${value.toExponential(2)}`;
  };

  return (
    <div className="module">
      <h2>L-Network Matching</h2>
      <p>
        Design a two-element L-network to match a complex load to a real source impedance.
        L-networks use one series and one shunt reactive element (inductor or capacitor).
      </p>

      <div className="controls">
        <label>
          Z_Source (Ω): <input type="range" min={25} max={150} step={5} value={zSource}
            onChange={e => setZSource(+e.target.value)} /> {zSource.toFixed(0)}
        </label>
        <label>
          Z_L Real (Ω): <input type="range" min={5} max={200} step={5} value={zlRe}
            onChange={e => setZlRe(+e.target.value)} /> {zlRe.toFixed(0)}
        </label>
        <label>
          Z_L Imag (Ω): <input type="range" min={-100} max={100} step={5} value={zlIm}
            onChange={e => setZlIm(+e.target.value)} /> {zlIm >= 0 ? '+' : ''}{zlIm.toFixed(0)}j
        </label>
        <label>
          Frequency: <input type="range" min={1e6} max={10e9} step={1e8} value={frequency}
            onChange={e => setFrequency(+e.target.value)} /> {(frequency / 1e9).toFixed(2)} GHz
        </label>
      </div>

      {result && result.length > 0 && (
        <div className="l-network-solutions">
          <h3>Available Solutions</h3>
          {result.map((sol, idx) => (
            <div key={idx} className="solution-card" style={{
              background: idx % 2 === 0 ? '#E3F2FD' : '#FFF3E0',
              padding: '1rem',
              borderRadius: '8px',
              marginBottom: '1rem'
            }}>
              <h4 style={{ margin: '0 0 0.5rem 0' }}>
                {sol.topology}
                <span style={{ fontSize: '0.8em', color: '#666', marginLeft: '1rem' }}>
                  Q = {sol.q_factor.toFixed(2)}
                </span>
              </h4>
              <div className="results-grid" style={{ display: 'grid', gridTemplateColumns: 'repeat(2, 1fr)', gap: '0.5rem' }}>
                <div className="result-card">
                  <span className="label">{sol.series_type.split(' ')[0]}</span>
                  <span className="value">{formatValue(sol.series_value, sol.series_type)}</span>
                </div>
                <div className="result-card">
                  <span className="label">{sol.shunt_type.split(' ')[0]}</span>
                  <span className="value">{sol.shunt_value > 0 ? formatValue(sol.shunt_value, sol.shunt_type) : 'N/A'}</span>
                </div>
              </div>
              <p style={{ margin: '0.5rem 0 0 0', fontSize: '0.9em', color: '#666' }}>
                Fractional Bandwidth ≈ {(sol.bandwidth_fractional * 100).toFixed(1)}%
              </p>
            </div>
          ))}
        </div>
      )}

      {result && result.length === 0 && (
        <div className="info-message" style={{ padding: '1rem', background: '#FFF9C4', borderRadius: '4px' }}>
          No L-network solution found. The load may already be matched or require a different network topology.
        </div>
      )}

      <div className="l-network-diagram" style={{ textAlign: 'center', margin: '20px 0' }}>
        <svg viewBox="0 0 400 120" style={{ width: '100%', maxWidth: 500 }}>
          {/* Source */}
          <circle cx="40" cy="60" r="20" fill="#E3F2FD" stroke="#2196F3" strokeWidth="2" />
          <text x="40" y="65" textAnchor="middle" fontSize="12" fill="#333">Z_S</text>
          
          {/* Series element */}
          <rect x="80" y="50" width="60" height="20" fill="#4CAF50" stroke="#388E3C" strokeWidth="2" rx="4" />
          <text x="110" y="65" textAnchor="middle" fontSize="10" fill="white">Series</text>
          
          {/* Junction */}
          <line x1="140" y1="60" x2="200" y2="60" stroke="#333" strokeWidth="2" />
          
          {/* Shunt element */}
          <rect x="180" y="70" width="20" height="40" fill="#FF9800" stroke="#F57C00" strokeWidth="2" rx="4" />
          <text x="190" y="125" textAnchor="middle" fontSize="10" fill="#666">Shunt</text>
          
          {/* Ground for shunt */}
          <line x1="190" y1="110" x2="190" y2="115" stroke="#333" strokeWidth="2" />
          <line x1="180" y1="115" x2="200" y2="115" stroke="#333" strokeWidth="2" />
          
          {/* Line to load */}
          <line x1="200" y1="60" x2="280" y2="60" stroke="#333" strokeWidth="2" />
          
          {/* Load */}
          <rect x="280" y="40" width="60" height="40" fill="#FFEBEE" stroke="#F44336" strokeWidth="2" rx="4" />
          <text x="310" y="65" textAnchor="middle" fontSize="12" fill="#333">Z_L</text>
        </svg>
      </div>

      <div className="theory">
        <h3>Theory</h3>
        <p><strong>L-network</strong> is the simplest matching network, using exactly two reactive elements.</p>
        
        <p><strong>Q Factor:</strong> Q = √(R_large/R_small - 1)</p>
        <p><strong>Bandwidth:</strong> Fractional BW ≈ 1/Q</p>
        
        <p><strong>Topologies:</strong></p>
        <ul>
          <li><strong>R_L &lt; R_S:</strong> Series element on load side, shunt on source side</li>
          <li><strong>R_L &gt; R_S:</strong> Shunt element on load side, series on source side</li>
          <li>Low-pass (L series, C shunt) or High-pass (C series, L shunt) variants</li>
        </ul>
        
        <p><strong>Limitation:</strong> Fixed Q for given impedance ratio — cannot independently control bandwidth.</p>
      </div>
    </div>
  );
}

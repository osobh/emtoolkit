import { useState, useMemo } from 'react';
import Plot from 'react-plotly.js';
import * as wasm from '../lib/em_wasm';

interface MultisectionResponse {
  design_type: string;
  num_sections: number;
  section_impedances: number[];
  junction_gammas: number[];
  fractional_bandwidth?: number;
  frequency_response: [number, number][];
}

export function MultisectionModule() {
  const [z0, setZ0] = useState(50.0);
  const [zLoad, setZLoad] = useState(200.0);
  const [numSections, setNumSections] = useState(3);
  const [designType, setDesignType] = useState<'binomial' | 'chebyshev'>('binomial');
  const [maxGamma, setMaxGamma] = useState(0.05);

  const result: MultisectionResponse | null = useMemo(() => {
    try {
      if (designType === 'binomial') {
        return wasm.multisection_binomial(z0, zLoad, numSections) as MultisectionResponse;
      } else {
        return wasm.multisection_chebyshev(z0, zLoad, numSections, maxGamma) as MultisectionResponse;
      }
    } catch {
      return null;
    }
  }, [z0, zLoad, numSections, designType, maxGamma]);

  return (
    <div className="module">
      <h2>Multisection Transformer</h2>
      <p>
        Design a broadband impedance matching network using multiple quarter-wave transformer sections.
        Choose between Binomial (maximally flat) or Chebyshev (equal ripple) response.
      </p>

      <div className="controls">
        <label>
          Z₀ (Ω): <input type="range" min={25} max={150} step={5} value={z0}
            onChange={e => setZ0(+e.target.value)} /> {z0.toFixed(0)}
        </label>
        <label>
          Z_Load (Ω): <input type="range" min={50} max={500} step={10} value={zLoad}
            onChange={e => setZLoad(+e.target.value)} /> {zLoad.toFixed(0)}
        </label>
        <label>
          Number of Sections: <input type="range" min={1} max={6} step={1} value={numSections}
            onChange={e => setNumSections(+e.target.value)} /> {numSections}
        </label>
        <label>
          Design Type:
          <select value={designType} onChange={e => setDesignType(e.target.value as 'binomial' | 'chebyshev')}>
            <option value="binomial">Binomial (Maximally Flat)</option>
            <option value="chebyshev">Chebyshev (Equal Ripple)</option>
          </select>
        </label>
        {designType === 'chebyshev' && (
          <label>
            Max |Γ| in passband: <input type="range" min={0.01} max={0.2} step={0.01} value={maxGamma}
              onChange={e => setMaxGamma(+e.target.value)} /> {maxGamma.toFixed(3)}
          </label>
        )}
      </div>

      {result && (
        <>
          <div className="results-grid">
            <div className="result-card">
              <span className="label">Design Type</span>
              <span className="value">{result.design_type}</span>
            </div>
            <div className="result-card">
              <span className="label">Sections</span>
              <span className="value">{result.num_sections}</span>
            </div>
            {result.fractional_bandwidth && (
              <div className="result-card">
                <span className="label">Fractional BW</span>
                <span className="value">{(result.fractional_bandwidth * 100).toFixed(1)}%</span>
              </div>
            )}
          </div>

          <h3>Section Impedances</h3>
          <div className="section-impedances" style={{ display: 'flex', gap: '0.5rem', flexWrap: 'wrap', marginBottom: '1rem' }}>
            <div className="impedance-chip" style={{ background: '#E3F2FD', padding: '0.5rem 1rem', borderRadius: '4px' }}>
              Z₀ = {z0.toFixed(1)} Ω
            </div>
            {result.section_impedances.map((z, idx) => (
              <div key={idx} className="impedance-chip" style={{ background: '#FFF3E0', padding: '0.5rem 1rem', borderRadius: '4px' }}>
                Z_{idx + 1} = {z.toFixed(2)} Ω
              </div>
            ))}
            <div className="impedance-chip" style={{ background: '#FFEBEE', padding: '0.5rem 1rem', borderRadius: '4px' }}>
              Z_L = {zLoad.toFixed(1)} Ω
            </div>
          </div>

          <Plot
            data={[{
              x: result.frequency_response.map(([f, _]) => f),
              y: result.frequency_response.map(([_, g]) => g),
              type: 'scatter',
              mode: 'lines',
              name: '|Γ(f)|',
              line: { color: '#2196F3', width: 2 },
            }]}
            layout={{
              title: 'Frequency Response',
              xaxis: { title: 'f / f₀ (normalized frequency)' },
              yaxis: { title: '|Γ|', range: [0, Math.max(0.5, ...result.frequency_response.map(([_, g]) => g))] },
              margin: { t: 40, r: 20, b: 50, l: 60 },
              height: 350,
              shapes: designType === 'chebyshev' ? [{
                type: 'line',
                x0: 0.2,
                x1: 1.8,
                y0: maxGamma,
                y1: maxGamma,
                line: { color: '#F44336', dash: 'dash', width: 1 }
              }] : [],
            }}
            config={{ responsive: true }}
            style={{ width: '100%' }}
          />

          <div className="transformer-diagram" style={{ textAlign: 'center', margin: '20px 0' }}>
            <svg viewBox="0 0 600 80" style={{ width: '100%', maxWidth: 700 }}>
              {/* Source */}
              <rect x="10" y="25" width="60" height="30" fill="#E3F2FD" stroke="#2196F3" strokeWidth="2" rx="4" />
              <text x="40" y="45" textAnchor="middle" fontSize="10" fill="#333">{z0.toFixed(0)}Ω</text>
              
              {/* Sections */}
              {result.section_impedances.map((z, idx) => {
                const x = 90 + idx * 80;
                return (
                  <g key={idx}>
                    <rect x={x} y="25" width="60" height="30" fill="#FFF3E0" stroke="#FF9800" strokeWidth="2" rx="4" />
                    <text x={x + 30} y="45" textAnchor="middle" fontSize="9" fill="#333">{z.toFixed(1)}Ω</text>
                    <text x={x + 30} y="70" textAnchor="middle" fontSize="8" fill="#666">λ/4</text>
                  </g>
                );
              })}
              
              {/* Load */}
              <rect x={90 + result.num_sections * 80} y="25" width="60" height="30" fill="#FFEBEE" stroke="#F44336" strokeWidth="2" rx="4" />
              <text x={120 + result.num_sections * 80} y="45" textAnchor="middle" fontSize="10" fill="#333">{zLoad.toFixed(0)}Ω</text>
            </svg>
          </div>
        </>
      )}

      <div className="theory">
        <h3>Theory</h3>
        <p><strong>Multisection transformers</strong> provide broadband matching using cascaded λ/4 sections.</p>
        
        <p><strong>Binomial (Butterworth):</strong></p>
        <ul>
          <li>Maximally flat response at center frequency</li>
          <li>All derivatives of Γ vanish at f₀</li>
          <li>Γₙ = C(N,n) · Γ₀ / 2^N</li>
        </ul>
        
        <p><strong>Chebyshev:</strong></p>
        <ul>
          <li>Equal-ripple response in passband</li>
          <li>Wider bandwidth for same N sections</li>
          <li>Trade-off: ripple for bandwidth</li>
        </ul>
        
        <p><strong>Section impedances:</strong> Each section's Z₀ is determined to create the desired reflection coefficient distribution.</p>
      </div>
    </div>
  );
}

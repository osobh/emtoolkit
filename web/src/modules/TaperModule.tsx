import { useState, useMemo } from 'react';
import Plot from 'react-plotly.js';
import * as wasm from '../lib/em_wasm';

interface TaperResponse {
  taper_type: string;
  length_wavelengths: number;
  impedance_profile: [number, number][];
  frequency_response: [number, number][];
  max_ripple: number;
  cutoff_beta_l: number;
}

export function TaperModule() {
  const [z0, setZ0] = useState(50.0);
  const [zLoad, setZLoad] = useState(100.0);
  const [lengthWavelengths, setLengthWavelengths] = useState(1.0);
  const [taperType, setTaperType] = useState<'exponential' | 'triangular' | 'klopfenstein'>('exponential');

  const result: TaperResponse | null = useMemo(() => {
    try {
      return wasm.taper_design(z0, zLoad, lengthWavelengths, taperType) as TaperResponse;
    } catch {
      return null;
    }
  }, [z0, zLoad, lengthWavelengths, taperType]);

  return (
    <div className="module">
      <h2>Tapered Line Matching</h2>
      <p>
        Design a continuously tapered transmission line for broadband impedance matching.
        The impedance varies smoothly from Z₀ to Z_L over the taper length.
      </p>

      <div className="controls">
        <label>
          Z₀ (Ω): <input type="range" min={25} max={150} step={5} value={z0}
            onChange={e => setZ0(+e.target.value)} /> {z0.toFixed(0)}
        </label>
        <label>
          Z_Load (Ω): <input type="range" min={50} max={300} step={10} value={zLoad}
            onChange={e => setZLoad(+e.target.value)} /> {zLoad.toFixed(0)}
        </label>
        <label>
          Taper Length (λ): <input type="range" min={0.25} max={3.0} step={0.25} value={lengthWavelengths}
            onChange={e => setLengthWavelengths(+e.target.value)} /> {lengthWavelengths.toFixed(2)}
        </label>
        <label>
          Taper Type:
          <select value={taperType} onChange={e => setTaperType(e.target.value as typeof taperType)}>
            <option value="exponential">Exponential</option>
            <option value="triangular">Triangular</option>
            <option value="klopfenstein">Klopfenstein (Optimal)</option>
          </select>
        </label>
      </div>

      {result && (
        <>
          <div className="results-grid">
            <div className="result-card">
              <span className="label">Taper Type</span>
              <span className="value">{result.taper_type}</span>
            </div>
            <div className="result-card">
              <span className="label">Length</span>
              <span className="value">{result.length_wavelengths.toFixed(2)} λ</span>
            </div>
            <div className="result-card">
              <span className="label">Max Ripple |Γ|</span>
              <span className="value">{result.max_ripple.toFixed(4)}</span>
            </div>
            <div className="result-card">
              <span className="label">Cutoff βL</span>
              <span className="value">{result.cutoff_beta_l.toFixed(2)}</span>
            </div>
          </div>

          <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '1rem' }}>
            <Plot
              data={[{
                x: result.impedance_profile.map(([z, _]) => z),
                y: result.impedance_profile.map(([_, Z]) => Z),
                type: 'scatter',
                mode: 'lines',
                name: 'Z(z)',
                line: { color: '#4CAF50', width: 2 },
              }]}
              layout={{
                title: 'Impedance Profile',
                xaxis: { title: 'z / L (normalized position)' },
                yaxis: { title: 'Z(z) (Ω)' },
                margin: { t: 40, r: 20, b: 50, l: 60 },
                height: 300,
              }}
              config={{ responsive: true }}
              style={{ width: '100%' }}
            />

            <Plot
              data={[{
                x: result.frequency_response.map(([beta_l, _]) => beta_l / Math.PI),
                y: result.frequency_response.map(([_, g]) => g),
                type: 'scatter',
                mode: 'lines',
                name: '|Γ(f)|',
                line: { color: '#2196F3', width: 2 },
              }]}
              layout={{
                title: 'Frequency Response',
                xaxis: { title: 'βL / π (electrical length)' },
                yaxis: { title: '|Γ|', range: [0, 0.4] },
                margin: { t: 40, r: 20, b: 50, l: 60 },
                height: 300,
              }}
              config={{ responsive: true }}
              style={{ width: '100%' }}
            />
          </div>

          <div className="taper-diagram" style={{ textAlign: 'center', margin: '20px 0' }}>
            <svg viewBox="0 0 500 100" style={{ width: '100%', maxWidth: 600 }}>
              {/* Source end */}
              <rect x="20" y="40" width="40" height="20" fill="#E3F2FD" stroke="#2196F3" strokeWidth="2" rx="2" />
              <text x="40" y="75" textAnchor="middle" fontSize="10" fill="#666">Z₀={z0}Ω</text>
              
              {/* Tapered section */}
              <polygon 
                points="60,35 380,25 380,75 60,65" 
                fill="url(#taperGradient)" 
                stroke="#666" 
                strokeWidth="1"
              />
              <defs>
                <linearGradient id="taperGradient" x1="0%" y1="0%" x2="100%" y2="0%">
                  <stop offset="0%" style={{ stopColor: '#E3F2FD' }} />
                  <stop offset="100%" style={{ stopColor: '#FFEBEE' }} />
                </linearGradient>
              </defs>
              <text x="220" y="55" textAnchor="middle" fontSize="12" fill="#333">{result.taper_type}</text>
              <text x="220" y="90" textAnchor="middle" fontSize="10" fill="#666">L = {lengthWavelengths.toFixed(2)}λ</text>
              
              {/* Load end */}
              <rect x="380" y="30" width="50" height="40" fill="#FFEBEE" stroke="#F44336" strokeWidth="2" rx="2" />
              <text x="405" y="55" textAnchor="middle" fontSize="10" fill="#333">Z_L</text>
              <text x="405" y="85" textAnchor="middle" fontSize="10" fill="#666">{zLoad}Ω</text>
            </svg>
          </div>
        </>
      )}

      <div className="theory">
        <h3>Theory</h3>
        
        <p><strong>Exponential Taper:</strong></p>
        <ul>
          <li>Z(z) = Z₀ · e^(az) where a = ln(Z_L/Z₀)/L</li>
          <li>|Γ| ≈ Γ₀ · |sinc(βL)|</li>
          <li>First null at βL = π</li>
        </ul>
        
        <p><strong>Triangular Taper:</strong></p>
        <ul>
          <li>Piecewise linear ln(Z) profile</li>
          <li>|Γ| ≈ Γ₀ · sinc²(βL/2)</li>
          <li>Faster rolloff than exponential</li>
        </ul>
        
        <p><strong>Klopfenstein Taper:</strong></p>
        <ul>
          <li>Optimal design minimizing passband ripple for given length</li>
          <li>Equal-ripple response above cutoff</li>
          <li>Requires modified Bessel functions for exact design</li>
        </ul>
        
        <p><strong>Trade-offs:</strong> Longer tapers give lower reflection but require more space. 
        Klopfenstein is optimal but more complex to implement physically.</p>
      </div>
    </div>
  );
}

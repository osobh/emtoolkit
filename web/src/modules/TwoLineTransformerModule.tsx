import { useState, useMemo } from 'react';
import Plot from 'react-plotly.js';
import * as wasm from '../lib/em_wasm';

interface TwoLineResponse {
  z1: number;
  l1_wavelengths: number;
  z2: number;
  l2_wavelengths: number;
  z_in_re: number;
  z_in_im: number;
  gamma_re: number;
  gamma_im: number;
  gamma_mag: number;
  vswr: number;
  frequency_response: [number, number][];
}

export function TwoLineTransformerModule() {
  const [z0, setZ0] = useState(50.0);
  const [zlRe, setZlRe] = useState(200.0);
  const [zlIm, setZlIm] = useState(0.0);
  const [z1, setZ1] = useState(84.09);
  const [l1, setL1] = useState(0.25);
  const [z2, setZ2] = useState(59.46);
  const [l2, setL2] = useState(0.25);
  const [frequency, setFrequency] = useState(1e9);

  // Auto-calculate geometric stepping for two sections
  const geometricZ = useMemo(() => {
    const zLmag = Math.sqrt(zlRe * zlRe + zlIm * zlIm);
    const ratio = Math.pow(zLmag / z0, 1/3);
    return {
      z1: z0 * ratio * ratio,
      z2: z0 * ratio,
    };
  }, [z0, zlRe, zlIm]);

  const result: TwoLineResponse | null = useMemo(() => {
    try {
      return wasm.two_line_transformer_analysis(z0, zlRe, zlIm, z1, l1, z2, l2, frequency) as TwoLineResponse;
    } catch {
      return null;
    }
  }, [z0, zlRe, zlIm, z1, l1, z2, l2, frequency]);

  return (
    <div className="module">
      <h2>Two-Line Section Transformer</h2>
      <p>
        Analyze a two-section transformer for broadband matching or complex load matching.
        Two cascaded transmission line sections provide more design flexibility than a single section.
      </p>

      <div className="controls">
        <label>
          Z₀ (Source) (Ω): <input type="range" min={25} max={150} step={5} value={z0}
            onChange={e => setZ0(+e.target.value)} /> {z0.toFixed(0)}
        </label>
        <label>
          Z_L Real (Ω): <input type="range" min={25} max={400} step={5} value={zlRe}
            onChange={e => setZlRe(+e.target.value)} /> {zlRe.toFixed(0)}
        </label>
        <label>
          Z_L Imag (Ω): <input type="range" min={-150} max={150} step={5} value={zlIm}
            onChange={e => setZlIm(+e.target.value)} /> {zlIm >= 0 ? '+' : ''}{zlIm.toFixed(0)}j
        </label>
        
        <div style={{ borderTop: '1px solid #ddd', paddingTop: '1rem', marginTop: '1rem' }}>
          <h4 style={{ margin: '0 0 0.5rem 0' }}>Section 1 (Near Load)</h4>
          <label>
            Z₁ (Ω): <input type="range" min={20} max={250} step={1} value={z1}
              onChange={e => setZ1(+e.target.value)} /> {z1.toFixed(1)}
          </label>
          <label>
            ℓ₁ (λ): <input type="range" min={0.05} max={0.5} step={0.01} value={l1}
              onChange={e => setL1(+e.target.value)} /> {l1.toFixed(3)}
          </label>
        </div>
        
        <div style={{ borderTop: '1px solid #ddd', paddingTop: '1rem', marginTop: '1rem' }}>
          <h4 style={{ margin: '0 0 0.5rem 0' }}>Section 2 (Near Source)</h4>
          <label>
            Z₂ (Ω): <input type="range" min={20} max={200} step={1} value={z2}
              onChange={e => setZ2(+e.target.value)} /> {z2.toFixed(1)}
          </label>
          <label>
            ℓ₂ (λ): <input type="range" min={0.05} max={0.5} step={0.01} value={l2}
              onChange={e => setL2(+e.target.value)} /> {l2.toFixed(3)}
          </label>
        </div>
        
        <button 
          onClick={() => {
            setZ1(geometricZ.z1);
            setZ2(geometricZ.z2);
            setL1(0.25);
            setL2(0.25);
          }}
          style={{ marginTop: '1rem', padding: '0.5rem 1rem' }}
        >
          Set Geometric Stepping (λ/4 sections)
        </button>
      </div>

      {result && (
        <>
          <div className="results-grid">
            <div className="result-card">
              <span className="label">Z_in</span>
              <span className="value">
                {result.z_in_re.toFixed(2)} {result.z_in_im >= 0 ? '+' : ''}{result.z_in_im.toFixed(2)}j Ω
              </span>
            </div>
            <div className="result-card">
              <span className="label">|Γ|</span>
              <span className="value">{result.gamma_mag.toFixed(4)}</span>
            </div>
            <div className="result-card">
              <span className="label">VSWR</span>
              <span className="value">{result.vswr < 100 ? result.vswr.toFixed(3) : '∞'}</span>
            </div>
            <div className="result-card">
              <span className="label">Return Loss</span>
              <span className="value">{result.gamma_mag > 0 ? (-20 * Math.log10(result.gamma_mag)).toFixed(2) : '∞'} dB</span>
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
              yaxis: { title: '|Γ|', range: [0, 1] },
              margin: { t: 40, r: 20, b: 50, l: 60 },
              height: 350,
            }}
            config={{ responsive: true }}
            style={{ width: '100%' }}
          />

          <div className="transformer-diagram" style={{ textAlign: 'center', margin: '20px 0' }}>
            <svg viewBox="0 0 600 100" style={{ width: '100%', maxWidth: 700 }}>
              {/* Source */}
              <rect x="20" y="35" width="80" height="30" fill="#E3F2FD" stroke="#2196F3" strokeWidth="2" rx="4" />
              <text x="60" y="55" textAnchor="middle" fontSize="10" fill="#333">Z₀ = {z0}Ω</text>
              
              {/* Section 2 (near source) */}
              <rect x="110" y="35" width="140" height="30" fill="#E8F5E9" stroke="#4CAF50" strokeWidth="2" rx="4" />
              <text x="180" y="50" textAnchor="middle" fontSize="10" fill="#333">Z₂ = {z2.toFixed(1)}Ω</text>
              <text x="180" y="62" textAnchor="middle" fontSize="9" fill="#666">ℓ₂ = {l2.toFixed(3)}λ</text>
              
              {/* Section 1 (near load) */}
              <rect x="260" y="35" width="140" height="30" fill="#FFF3E0" stroke="#FF9800" strokeWidth="2" rx="4" />
              <text x="330" y="50" textAnchor="middle" fontSize="10" fill="#333">Z₁ = {z1.toFixed(1)}Ω</text>
              <text x="330" y="62" textAnchor="middle" fontSize="9" fill="#666">ℓ₁ = {l1.toFixed(3)}λ</text>
              
              {/* Load */}
              <rect x="410" y="35" width="100" height="30" fill="#FFEBEE" stroke="#F44336" strokeWidth="2" rx="4" />
              <text x="460" y="55" textAnchor="middle" fontSize="9" fill="#333">
                Z_L = {zlRe}{zlIm >= 0 ? '+' : ''}{zlIm}j
              </text>
              
              {/* Labels */}
              <text x="60" y="85" textAnchor="middle" fontSize="9" fill="#666">Source</text>
              <text x="180" y="85" textAnchor="middle" fontSize="9" fill="#666">Section 2</text>
              <text x="330" y="85" textAnchor="middle" fontSize="9" fill="#666">Section 1</text>
              <text x="460" y="85" textAnchor="middle" fontSize="9" fill="#666">Load</text>
            </svg>
          </div>

          <div className="match-quality" style={{ 
            padding: '1rem', 
            background: result.gamma_mag < 0.1 ? '#E8F5E9' : result.gamma_mag < 0.3 ? '#FFF3E0' : '#FFEBEE',
            borderRadius: '8px',
            marginTop: '1rem'
          }}>
            <strong>Match Quality: </strong>
            {result.gamma_mag < 0.05 ? 'Excellent (|Γ| < 0.05)' :
             result.gamma_mag < 0.1 ? 'Good (|Γ| < 0.1)' :
             result.gamma_mag < 0.3 ? 'Moderate (|Γ| < 0.3)' :
             'Poor (|Γ| ≥ 0.3)'}
          </div>
        </>
      )}

      <div className="theory">
        <h3>Theory</h3>
        <p><strong>Two-section transformer</strong> cascades two transmission line sections for:</p>
        <ul>
          <li>Broader bandwidth than single quarter-wave</li>
          <li>Matching complex loads</li>
          <li>Optimizing for specific frequency response</li>
        </ul>
        
        <p><strong>Geometric stepping:</strong> For broadband matching of real loads with N sections,
        use impedances that step geometrically: Z_n = Z₀ × (Z_L/Z₀)^(n/(N+1))</p>
        
        <p><strong>Complex loads:</strong> First section can transform complex load toward real axis,
        second section completes the match to Z₀.</p>
        
        <p><strong>Design approach:</strong></p>
        <ol>
          <li>Start with geometric stepping as initial guess</li>
          <li>Adjust section lengths for complex loads</li>
          <li>Optimize for desired bandwidth or center frequency</li>
        </ol>
      </div>
    </div>
  );
}

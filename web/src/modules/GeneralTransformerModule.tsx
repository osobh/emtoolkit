import { useState, useMemo } from 'react';
import Plot from 'react-plotly.js';
import * as wasm from '../lib/em_wasm';

interface TransformerResponse {
  z_transformer: number;
  length_wavelengths: number;
  z_in_re: number;
  z_in_im: number;
  gamma_re: number;
  gamma_im: number;
  gamma_mag: number;
  vswr: number;
  frequency_response: [number, number][];
}

export function GeneralTransformerModule() {
  const [z0, setZ0] = useState(50.0);
  const [zlRe, setZlRe] = useState(100.0);
  const [zlIm, setZlIm] = useState(0.0);
  const [zTransformer, setZTransformer] = useState(70.71);
  const [lengthWavelengths, setLengthWavelengths] = useState(0.25);
  const [frequency, setFrequency] = useState(1e9);

  // Auto-calculate optimal Z_transformer for quarter-wave
  const optimalZt = useMemo(() => {
    const zMag = Math.sqrt(zlRe * zlRe + zlIm * zlIm);
    return Math.sqrt(z0 * zMag);
  }, [z0, zlRe, zlIm]);

  const result: TransformerResponse | null = useMemo(() => {
    try {
      return wasm.general_transformer_analysis(z0, zlRe, zlIm, zTransformer, lengthWavelengths, frequency) as TransformerResponse;
    } catch {
      return null;
    }
  }, [z0, zlRe, zlIm, zTransformer, lengthWavelengths, frequency]);

  return (
    <div className="module">
      <h2>General Impedance Transformer</h2>
      <p>
        Analyze a transmission line transformer section of arbitrary length.
        Unlike the quarter-wave transformer, this allows any length and complex loads.
      </p>

      <div className="controls">
        <label>
          Z₀ (Source Line) (Ω): <input type="range" min={25} max={150} step={5} value={z0}
            onChange={e => setZ0(+e.target.value)} /> {z0.toFixed(0)}
        </label>
        <label>
          Z_L Real (Ω): <input type="range" min={10} max={300} step={5} value={zlRe}
            onChange={e => setZlRe(+e.target.value)} /> {zlRe.toFixed(0)}
        </label>
        <label>
          Z_L Imag (Ω): <input type="range" min={-150} max={150} step={5} value={zlIm}
            onChange={e => setZlIm(+e.target.value)} /> {zlIm >= 0 ? '+' : ''}{zlIm.toFixed(0)}j
        </label>
        <label>
          Z_Transformer (Ω): <input type="range" min={20} max={200} step={1} value={zTransformer}
            onChange={e => setZTransformer(+e.target.value)} /> {zTransformer.toFixed(1)}
          <button 
            onClick={() => setZTransformer(optimalZt)}
            style={{ marginLeft: '1rem', padding: '0.25rem 0.5rem', fontSize: '0.8em' }}
          >
            Set Optimal ({optimalZt.toFixed(1)} Ω)
          </button>
        </label>
        <label>
          Length (λ): <input type="range" min={0.05} max={0.5} step={0.01} value={lengthWavelengths}
            onChange={e => setLengthWavelengths(+e.target.value)} /> {lengthWavelengths.toFixed(3)}
          <button 
            onClick={() => setLengthWavelengths(0.25)}
            style={{ marginLeft: '1rem', padding: '0.25rem 0.5rem', fontSize: '0.8em' }}
          >
            Set λ/4
          </button>
        </label>
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
              shapes: [{
                type: 'line',
                x0: 1,
                x1: 1,
                y0: 0,
                y1: 1,
                line: { color: '#F44336', dash: 'dash', width: 1 }
              }],
            }}
            config={{ responsive: true }}
            style={{ width: '100%' }}
          />

          <div className="transformer-diagram" style={{ textAlign: 'center', margin: '20px 0' }}>
            <svg viewBox="0 0 500 100" style={{ width: '100%', maxWidth: 600 }}>
              {/* Source line */}
              <rect x="20" y="35" width="120" height="30" fill="#E3F2FD" stroke="#2196F3" strokeWidth="2" rx="4" />
              <text x="80" y="55" textAnchor="middle" fontSize="11" fill="#333">Z₀ = {z0}Ω</text>
              
              {/* Transformer section */}
              <rect x="140" y="35" width="160" height="30" fill="#FFF3E0" stroke="#FF9800" strokeWidth="2" rx="4" />
              <text x="220" y="50" textAnchor="middle" fontSize="10" fill="#333">Z_T = {zTransformer.toFixed(1)}Ω</text>
              <text x="220" y="62" textAnchor="middle" fontSize="9" fill="#666">ℓ = {lengthWavelengths.toFixed(3)}λ</text>
              
              {/* Load */}
              <rect x="300" y="35" width="120" height="30" fill="#FFEBEE" stroke="#F44336" strokeWidth="2" rx="4" />
              <text x="360" y="55" textAnchor="middle" fontSize="10" fill="#333">
                Z_L = {zlRe}{zlIm >= 0 ? '+' : ''}{zlIm}j Ω
              </text>
              
              {/* Arrows */}
              <text x="80" y="85" textAnchor="middle" fontSize="10" fill="#666">Source</text>
              <text x="220" y="85" textAnchor="middle" fontSize="10" fill="#666">Transformer</text>
              <text x="360" y="85" textAnchor="middle" fontSize="10" fill="#666">Load</text>
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
        <p><strong>Input impedance</strong> of a lossless line section:</p>
        <p style={{ fontFamily: 'monospace', background: '#f5f5f5', padding: '0.5rem', borderRadius: '4px' }}>
          Z_in = Z_T × (Z_L + jZ_T·tan(βℓ)) / (Z_T + jZ_L·tan(βℓ))
        </p>
        
        <p><strong>Quarter-wave (λ/4):</strong> When ℓ = λ/4, tan(βℓ) → ∞, giving Z_in = Z_T²/Z_L</p>
        <p><strong>Half-wave (λ/2):</strong> When ℓ = λ/2, tan(βℓ) = 0, giving Z_in = Z_L (repeats)</p>
        
        <p><strong>Optimal Z_T for real loads:</strong> Z_T = √(Z₀ × Z_L)</p>
        
        <p><strong>Note:</strong> For complex loads, the optimal transformer impedance and length 
        may differ from the simple quarter-wave formula.</p>
      </div>
    </div>
  );
}

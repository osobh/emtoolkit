import { useState, useMemo } from 'react';
import * as wasm from '../lib/em_wasm';

interface TransformerResult {
  turns_ratio: number;
  v_secondary: number;
  i_secondary: number;
  power_primary: number;
  power_secondary: number;
  impedance_ratio: number;
  step_type: string;
}

export function TransformerModule() {
  const [nPrimary, setNPrimary] = useState(100);
  const [nSecondary, setNSecondary] = useState(200);
  const [vPrimary, setVPrimary] = useState(120.0);
  const [iPrimary, setIPrimary] = useState(2.0);

  const result: TransformerResult | null = useMemo(() => {
    try {
      return wasm.transformer(nPrimary, nSecondary, vPrimary, iPrimary) as TransformerResult;
    } catch { return null; }
  }, [nPrimary, nSecondary, vPrimary, iPrimary]);

  return (
    <div className="module">
      <h2>Ideal Transformer</h2>
      <p>Compute voltage, current, and impedance transformation for an ideal transformer.</p>

      <div className="controls">
        <label>
          Primary turns (N₁): <input type="range" min={1} max={500} step={1} value={nPrimary}
            onChange={e => setNPrimary(+e.target.value)} /> {nPrimary}
        </label>
        <label>
          Secondary turns (N₂): <input type="range" min={1} max={500} step={1} value={nSecondary}
            onChange={e => setNSecondary(+e.target.value)} /> {nSecondary}
        </label>
        <label>
          V₁ (V): <input type="range" min={1} max={480} step={1} value={vPrimary}
            onChange={e => setVPrimary(+e.target.value)} /> {vPrimary.toFixed(0)}
        </label>
        <label>
          I₁ (A): <input type="range" min={0.1} max={50} step={0.1} value={iPrimary}
            onChange={e => setIPrimary(+e.target.value)} /> {iPrimary.toFixed(1)}
        </label>
      </div>

      {result && (
        <>
          <div className="results-grid">
            <div className="result-card">
              <span className="label">Turns Ratio (N₂/N₁)</span>
              <span className="value">{result.turns_ratio.toFixed(3)}</span>
            </div>
            <div className="result-card">
              <span className="label">V₂</span>
              <span className="value">{result.v_secondary.toFixed(1)} V</span>
            </div>
            <div className="result-card">
              <span className="label">I₂</span>
              <span className="value">{result.i_secondary.toFixed(3)} A</span>
            </div>
            <div className="result-card">
              <span className="label">Type</span>
              <span className="value">{result.step_type}</span>
            </div>
            <div className="result-card">
              <span className="label">Power</span>
              <span className="value">{result.power_primary.toFixed(1)} W</span>
            </div>
            <div className="result-card">
              <span className="label">Z ratio (Z₂/Z₁)</span>
              <span className="value">{result.impedance_ratio.toFixed(3)}</span>
            </div>
          </div>

          <div className="transformer-diagram">
            <svg viewBox="0 0 400 200" style={{ width: '100%', maxWidth: 500, display: 'block', margin: '20px auto' }}>
              {/* Primary coil */}
              <text x="60" y="20" textAnchor="middle" fontSize="14" fill="#333">Primary</text>
              {Array.from({ length: 6 }).map((_, i) => (
                <path key={`p${i}`} d={`M 50 ${35 + i * 20} C 30 ${35 + i * 20}, 30 ${55 + i * 20}, 50 ${55 + i * 20}`}
                  fill="none" stroke="#2196F3" strokeWidth="2" />
              ))}
              <line x1="50" y1="35" x2="50" y2="25" stroke="#2196F3" strokeWidth="2" />
              <line x1="50" y1="155" x2="50" y2="165" stroke="#2196F3" strokeWidth="2" />
              <text x="20" y="100" textAnchor="middle" fontSize="12" fill="#666">N₁={nPrimary}</text>

              {/* Core */}
              <line x1="70" y1="25" x2="70" y2="175" stroke="#333" strokeWidth="3" />
              <line x1="130" y1="25" x2="130" y2="175" stroke="#333" strokeWidth="3" />

              {/* Secondary coil */}
              <text x="160" y="20" textAnchor="middle" fontSize="14" fill="#333">Secondary</text>
              {Array.from({ length: 6 }).map((_, i) => (
                <path key={`s${i}`} d={`M 150 ${35 + i * 20} C 170 ${35 + i * 20}, 170 ${55 + i * 20}, 150 ${55 + i * 20}`}
                  fill="none" stroke="#F44336" strokeWidth="2" />
              ))}
              <line x1="150" y1="35" x2="150" y2="25" stroke="#F44336" strokeWidth="2" />
              <line x1="150" y1="155" x2="150" y2="165" stroke="#F44336" strokeWidth="2" />
              <text x="185" y="100" textAnchor="middle" fontSize="12" fill="#666">N₂={nSecondary}</text>

              {/* Labels */}
              <text x="290" y="50" fontSize="12" fill="#333">V₁ = {vPrimary.toFixed(0)} V</text>
              <text x="290" y="70" fontSize="12" fill="#333">I₁ = {iPrimary.toFixed(1)} A</text>
              <text x="290" y="100" fontSize="12" fill="#333">V₂ = {result.v_secondary.toFixed(1)} V</text>
              <text x="290" y="120" fontSize="12" fill="#333">I₂ = {result.i_secondary.toFixed(2)} A</text>
              <text x="290" y="150" fontSize="12" fill="#333">P = {result.power_primary.toFixed(1)} W</text>
            </svg>
          </div>
        </>
      )}

      <div className="theory">
        <h3>Theory</h3>
        <p><strong>Voltage:</strong> V₂/V₁ = N₂/N₁</p>
        <p><strong>Current:</strong> I₂/I₁ = N₁/N₂ (power conservation)</p>
        <p><strong>Impedance:</strong> Z₂/Z₁ = (N₂/N₁)²</p>
        <p>Step-up: N₂ &gt; N₁ (higher voltage, lower current). Step-down: N₂ &lt; N₁.</p>
      </div>
    </div>
  );
}

import { useState, useMemo } from 'react';
import Plot from 'react-plotly.js';
import * as wasm from '../lib/em_wasm';

const MATERIALS: Record<string, { er: number; label: string }> = {
  air: { er: 1.0, label: 'Air (n=1.0)' },
  water: { er: 1.77, label: 'Water (n=1.33)' },
  glass: { er: 2.25, label: 'Glass (n=1.5)' },
  diamond: { er: 5.84, label: 'Diamond (n=2.42)' },
  silicon: { er: 11.7, label: 'Silicon (n=3.42)' },
  custom: { er: 4.0, label: 'Custom' },
};

export function SnellModule() {
  const [mat1, setMat1] = useState('air');
  const [mat2, setMat2] = useState('glass');
  const [customEr1, setCustomEr1] = useState(1.0);
  const [customEr2, setCustomEr2] = useState(4.0);
  const [thetaI, setThetaI] = useState(30);

  const er1 = mat1 === 'custom' ? customEr1 : MATERIALS[mat1].er;
  const er2 = mat2 === 'custom' ? customEr2 : MATERIALS[mat2].er;
  const n1 = Math.sqrt(er1);
  const n2 = Math.sqrt(er2);

  const result = useMemo(() => {
    try {
      return wasm.fresnel_oblique(er1, er2, thetaI) as {
        theta_t_deg: number | null;
        is_tir: boolean;
        critical_angle_deg: number | null;
        brewster_angle_deg: number;
        gamma_perp: number;
        gamma_par: number;
      };
    } catch { return null; }
  }, [er1, er2, thetaI]);

  const sweepData = useMemo(() => {
    try {
      return wasm.fresnel_vs_angle(er1, er2, 200) as {
        angles_deg: number[];
        gamma_perp: number[];
        gamma_par: number[];
      };
    } catch { return null; }
  }, [er1, er2]);

  return (
    <div className="module">
      <h2>Snell's Law & Refraction</h2>
      <p>Explore refraction, total internal reflection, and Brewster's angle at dielectric interfaces.</p>

      <div className="controls">
        <label>
          Medium 1:
          <select value={mat1} onChange={e => setMat1(e.target.value)}>
            {Object.entries(MATERIALS).map(([k, v]) => <option key={k} value={k}>{v.label}</option>)}
          </select>
        </label>
        {mat1 === 'custom' && (
          <label>εᵣ₁: <input type="range" min={1} max={20} step={0.1} value={customEr1}
            onChange={e => setCustomEr1(+e.target.value)} /> {customEr1.toFixed(1)}</label>
        )}
        <label>
          Medium 2:
          <select value={mat2} onChange={e => setMat2(e.target.value)}>
            {Object.entries(MATERIALS).map(([k, v]) => <option key={k} value={k}>{v.label}</option>)}
          </select>
        </label>
        {mat2 === 'custom' && (
          <label>εᵣ₂: <input type="range" min={1} max={20} step={0.1} value={customEr2}
            onChange={e => setCustomEr2(+e.target.value)} /> {customEr2.toFixed(1)}</label>
        )}
        <label>θ_i (°): <input type="range" min={0} max={89} step={1} value={thetaI}
          onChange={e => setThetaI(+e.target.value)} /> {thetaI}°</label>
      </div>

      {result && (
        <div className="results-grid">
          <div className="result-card">
            <span className="label">n₁</span>
            <span className="value">{n1.toFixed(3)}</span>
          </div>
          <div className="result-card">
            <span className="label">n₂</span>
            <span className="value">{n2.toFixed(3)}</span>
          </div>
          <div className="result-card">
            <span className="label">θ_t (refracted)</span>
            <span className="value">{result.is_tir ? 'TIR' : result.theta_t_deg !== null ? result.theta_t_deg.toFixed(2) + '°' : '—'}</span>
          </div>
          <div className="result-card">
            <span className="label">Brewster angle</span>
            <span className="value">{result.brewster_angle_deg.toFixed(2)}°</span>
          </div>
          <div className="result-card">
            <span className="label">Critical angle</span>
            <span className="value">{result.critical_angle_deg !== null ? result.critical_angle_deg.toFixed(2) + '°' : 'N/A (n₁ < n₂)'}</span>
          </div>
          <div className="result-card">
            <span className="label">Status</span>
            <span className="value" style={{ color: result.is_tir ? '#F44336' : '#4CAF50' }}>
              {result.is_tir ? '⚠ Total Internal Reflection' : '✓ Transmitting'}
            </span>
          </div>
          <div className="result-card">
            <span className="label">Γ_⊥ (TE)</span>
            <span className="value">{result.gamma_perp.toFixed(4)}</span>
          </div>
          <div className="result-card">
            <span className="label">Γ_∥ (TM)</span>
            <span className="value">{result.gamma_par.toFixed(4)}</span>
          </div>
        </div>
      )}

      <div style={{ textAlign: 'center', margin: '20px 0' }}>
        <svg viewBox="0 0 400 300" style={{ width: '100%', maxWidth: 500 }}>
          {/* Interface */}
          <line x1="0" y1="150" x2="400" y2="150" stroke="#999" strokeWidth="2" />
          <rect x="0" y="150" width="400" height="150" fill="rgba(33,150,243,0.1)" />
          <text x="380" y="140" fontSize="12" fill="#666" textAnchor="end">n₁ = {n1.toFixed(2)}</text>
          <text x="380" y="170" fontSize="12" fill="#666" textAnchor="end">n₂ = {n2.toFixed(2)}</text>
          {/* Normal */}
          <line x1="200" y1="20" x2="200" y2="280" stroke="#ccc" strokeWidth="1" strokeDasharray="4" />
          {/* Incident ray */}
          {(() => {
            const rad = thetaI * Math.PI / 180;
            const x1 = 200 - 130 * Math.sin(rad);
            const y1 = 150 - 130 * Math.cos(rad);
            return <line x1={x1} y1={y1} x2="200" y2="150" stroke="#F44336" strokeWidth="2" markerEnd="url(#arrow)" />;
          })()}
          {/* Reflected ray */}
          {(() => {
            const rad = thetaI * Math.PI / 180;
            const x2 = 200 + 130 * Math.sin(rad);
            const y2 = 150 - 130 * Math.cos(rad);
            return <line x1="200" y1="150" x2={x2} y2={y2} stroke="#FF9800" strokeWidth="2" />;
          })()}
          {/* Transmitted ray */}
          {result && !result.is_tir && result.theta_t_deg !== null && (() => {
            const rad = result.theta_t_deg * Math.PI / 180;
            const x2 = 200 + 130 * Math.sin(rad);
            const y2 = 150 + 130 * Math.cos(rad);
            return <line x1="200" y1="150" x2={x2} y2={y2} stroke="#2196F3" strokeWidth="2" />;
          })()}
          {/* Labels */}
          <text x="120" y="80" fontSize="13" fill="#F44336">θ_i = {thetaI}°</text>
          <text x="260" y="80" fontSize="13" fill="#FF9800">θ_r = {thetaI}°</text>
          {result && !result.is_tir && result.theta_t_deg !== null &&
            <text x="260" y="230" fontSize="13" fill="#2196F3">θ_t = {result.theta_t_deg.toFixed(1)}°</text>
          }
          {result?.is_tir && <text x="150" y="230" fontSize="14" fill="#F44336" fontWeight="bold">Total Internal Reflection</text>}
          <defs>
            <marker id="arrow" viewBox="0 0 10 10" refX="5" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
              <path d="M 0 0 L 10 5 L 0 10 z" fill="#F44336" />
            </marker>
          </defs>
        </svg>
      </div>

      {sweepData && (
        <Plot
          data={[
            { x: sweepData.angles_deg, y: sweepData.gamma_perp.map(Math.abs), type: 'scatter', mode: 'lines',
              name: '|Γ_⊥| (TE)', line: { color: '#2196F3', width: 2 } },
            { x: sweepData.angles_deg, y: sweepData.gamma_par.map(Math.abs), type: 'scatter', mode: 'lines',
              name: '|Γ_∥| (TM)', line: { color: '#F44336', width: 2 } },
          ]}
          layout={{
            title: 'Reflection Coefficient vs Angle',
            xaxis: { title: 'θ_i (degrees)', range: [0, 90] },
            yaxis: { title: '|Γ|', range: [0, 1.05] },
            margin: { t: 40, r: 20, b: 50, l: 60 }, height: 350,
            legend: { x: 0.02, y: 0.98 },
          }}
          config={{ responsive: true }} style={{ width: '100%' }}
        />
      )}

      <div className="theory">
        <h3>Theory</h3>
        <p><strong>Snell's law:</strong> n₁ sin θ_i = n₂ sin θ_t</p>
        <p><strong>Brewster angle:</strong> θ_B = arctan(n₂/n₁) — zero TM reflection</p>
        <p><strong>Critical angle:</strong> θ_c = arcsin(n₂/n₁) — exists only when n₁ &gt; n₂</p>
        <p><strong>TIR:</strong> When θ_i &gt; θ_c, all power is reflected (evanescent wave in medium 2)</p>
      </div>
    </div>
  );
}

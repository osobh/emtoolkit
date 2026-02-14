import { useState } from 'react';

interface Material {
  name: string;
  type: string;
  mu_r: number;
  chi_m: number;
  curie?: number;
  uses: string;
}

const MATERIALS: Material[] = [
  { name: 'Vacuum', type: 'N/A', mu_r: 1.0, chi_m: 0, uses: 'Reference' },
  { name: 'Copper', type: 'Diamagnetic', mu_r: 0.999990, chi_m: -9.63e-6, uses: 'Wiring, waveguides' },
  { name: 'Silver', type: 'Diamagnetic', mu_r: 0.999974, chi_m: -2.6e-5, uses: 'Conductors, mirrors' },
  { name: 'Water', type: 'Diamagnetic', mu_r: 0.999991, chi_m: -9.04e-6, uses: 'Biological systems' },
  { name: 'Aluminum', type: 'Paramagnetic', mu_r: 1.000021, chi_m: 2.07e-5, uses: 'Lightweight structures' },
  { name: 'Platinum', type: 'Paramagnetic', mu_r: 1.000265, chi_m: 2.65e-4, uses: 'Sensors, catalysts' },
  { name: 'Iron', type: 'Ferromagnetic', mu_r: 5000, chi_m: 4999, curie: 770, uses: 'Transformers, motors' },
  { name: 'Nickel', type: 'Ferromagnetic', mu_r: 600, chi_m: 599, curie: 358, uses: 'Shielding, alloys' },
  { name: 'Cobalt', type: 'Ferromagnetic', mu_r: 250, chi_m: 249, curie: 1115, uses: 'Permanent magnets' },
  { name: 'Mu-metal', type: 'Ferromagnetic', mu_r: 100000, chi_m: 99999, curie: 400, uses: 'Magnetic shielding' },
  { name: 'Ferrite (MnZn)', type: 'Ferrimagnetic', mu_r: 2000, chi_m: 1999, curie: 300, uses: 'RF transformers, inductors' },
  { name: 'Ferrite (NiZn)', type: 'Ferrimagnetic', mu_r: 100, chi_m: 99, curie: 500, uses: 'High-freq RF, EMI filters' },
];

export function MagMaterialsModule() {
  const [selected, setSelected] = useState(6); // Iron by default
  const [hField, setHField] = useState(100);

  const mat = MATERIALS[selected];
  const mu0 = 4e-7 * Math.PI;
  const bField = mu0 * mat.mu_r * hField;
  const mField = mat.chi_m * hField;

  return (
    <div className="module">
      <h2>Magnetic Materials</h2>
      <p>Explore diamagnetic, paramagnetic, ferromagnetic, and ferrimagnetic materials.</p>

      <div className="controls">
        <label>Material:
          <select value={selected} onChange={e => setSelected(+e.target.value)}>
            {MATERIALS.map((m, i) => (
              <option key={i} value={i}>{m.name} ({m.type})</option>
            ))}
          </select>
        </label>
        <label>H (A/m): <input type="range" min={1} max={10000} step={1} value={hField}
          onChange={e => setHField(+e.target.value)} /> {hField}</label>
      </div>

      <div className="results-grid">
        <div className="result-card">
          <span className="label">Material</span>
          <span className="value">{mat.name}</span>
        </div>
        <div className="result-card">
          <span className="label">Type</span>
          <span className="value" style={{ color:
            mat.type === 'Ferromagnetic' ? '#F44336' :
            mat.type === 'Ferrimagnetic' ? '#FF9800' :
            mat.type === 'Paramagnetic' ? '#2196F3' :
            mat.type === 'Diamagnetic' ? '#4CAF50' : '#666'
          }}>{mat.type}</span>
        </div>
        <div className="result-card">
          <span className="label">μᵣ</span>
          <span className="value">{mat.mu_r >= 100 ? mat.mu_r.toLocaleString() : mat.mu_r.toFixed(6)}</span>
        </div>
        <div className="result-card">
          <span className="label">χ_m</span>
          <span className="value">{Math.abs(mat.chi_m) < 0.01 ? mat.chi_m.toExponential(3) : mat.chi_m.toFixed(0)}</span>
        </div>
        <div className="result-card">
          <span className="label">B = μ₀μᵣH</span>
          <span className="value">{bField < 0.01 ? (bField * 1e3).toFixed(3) + ' mT'
            : bField.toFixed(4) + ' T'}</span>
        </div>
        <div className="result-card">
          <span className="label">M = χ_m H</span>
          <span className="value">{Math.abs(mField) < 1 ? mField.toExponential(3) : mField.toFixed(0)} A/m</span>
        </div>
        {mat.curie && (
          <div className="result-card">
            <span className="label">Curie temperature</span>
            <span className="value">{mat.curie} °C</span>
          </div>
        )}
        <div className="result-card">
          <span className="label">Applications</span>
          <span className="value">{mat.uses}</span>
        </div>
      </div>

      <div style={{ marginTop: 20 }}>
        <h3>Material Classification</h3>
        <table style={{ width: '100%', borderCollapse: 'collapse', fontSize: 13 }}>
          <thead>
            <tr style={{ borderBottom: '2px solid #ddd' }}>
              <th style={{ padding: 6, textAlign: 'left' }}>Type</th>
              <th style={{ padding: 6 }}>χ_m</th>
              <th style={{ padding: 6 }}>μᵣ</th>
              <th style={{ padding: 6, textAlign: 'left' }}>Behavior</th>
            </tr>
          </thead>
          <tbody>
            <tr style={{ background: '#E8F5E9' }}>
              <td style={{ padding: 6 }}>Diamagnetic</td>
              <td style={{ padding: 6, textAlign: 'center' }}>−10⁻⁵</td>
              <td style={{ padding: 6, textAlign: 'center' }}>≈ 1 (slightly &lt;)</td>
              <td style={{ padding: 6 }}>Weakly opposes applied field</td>
            </tr>
            <tr style={{ background: '#E3F2FD' }}>
              <td style={{ padding: 6 }}>Paramagnetic</td>
              <td style={{ padding: 6, textAlign: 'center' }}>+10⁻⁵ to 10⁻³</td>
              <td style={{ padding: 6, textAlign: 'center' }}>≈ 1 (slightly &gt;)</td>
              <td style={{ padding: 6 }}>Weakly aligns with field</td>
            </tr>
            <tr style={{ background: '#FFEBEE' }}>
              <td style={{ padding: 6 }}>Ferromagnetic</td>
              <td style={{ padding: 6, textAlign: 'center' }}>10² to 10⁵</td>
              <td style={{ padding: 6, textAlign: 'center' }}>100 − 100,000</td>
              <td style={{ padding: 6 }}>Strong alignment, hysteresis, domains</td>
            </tr>
            <tr style={{ background: '#FFF3E0' }}>
              <td style={{ padding: 6 }}>Ferrimagnetic</td>
              <td style={{ padding: 6, textAlign: 'center' }}>10 to 10⁴</td>
              <td style={{ padding: 6, textAlign: 'center' }}>10 − 10,000</td>
              <td style={{ padding: 6 }}>Like ferro but lower μ; useful at RF</td>
            </tr>
          </tbody>
        </table>
      </div>

      <div className="theory">
        <h3>Key Relations</h3>
        <p><strong>B</strong> = μ₀(H + M) = μ₀μᵣH</p>
        <p><strong>M</strong> = χ_m H (linear approximation)</p>
        <p><strong>μᵣ</strong> = 1 + χ_m</p>
        <p>Above the Curie temperature, ferromagnets become paramagnetic.</p>
      </div>
    </div>
  );
}

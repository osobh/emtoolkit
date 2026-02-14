import { useState, useMemo } from 'react';

export function BoundaryModule() {
  const [er1, setEr1] = useState(1.0);
  const [er2, setEr2] = useState(4.0);
  const [mur1, setMur1] = useState(1.0);
  const [mur2, setMur2] = useState(1.0);
  const [dn1, setDn1] = useState(1.0);  // D_n in medium 1
  const [et1, setEt1] = useState(10.0); // E_t in medium 1
  const [bn1, setBn1] = useState(0.5);  // B_n in medium 1
  const [ht1, setHt1] = useState(100.0); // H_t in medium 1
  const [rhoS, setRhoS] = useState(0.0); // surface charge
  const [jsT, setJsT] = useState(0.0);   // surface current

  const eps0 = 8.854187817e-12;
  const mu0 = 4e-7 * Math.PI;

  const results = useMemo(() => {
    // D_n continuous (with surface charge): D1n - D2n = ρ_s
    const dn2 = dn1 - rhoS;
    // E_n: D = εE → E = D/(ε₀εᵣ)
    const en1 = dn1 / (eps0 * er1);
    const en2 = dn2 / (eps0 * er2);

    // E_t continuous
    const et2 = et1;
    // D_t: D = εE
    const dt1 = eps0 * er1 * et1;
    const dt2 = eps0 * er2 * et2;

    // B_n continuous
    const bn2 = bn1;
    // H_n: H = B/(μ₀μᵣ)
    const hn1 = bn1 / (mu0 * mur1);
    const hn2 = bn2 / (mu0 * mur2);

    // H_t: H1t - H2t = Js (with surface current)
    const ht2 = ht1 - jsT;
    // B_t: B = μH
    const bt1 = mu0 * mur1 * ht1;
    const bt2 = mu0 * mur2 * ht2;

    // Refraction angles
    const e_angle1 = Math.atan2(et1, en1) * 180 / Math.PI;
    const e_angle2 = Math.atan2(et2, en2) * 180 / Math.PI;

    return {
      dn1, dn2, en1, en2, et1, et2, dt1, dt2,
      bn1, bn2, hn1, hn2, ht1, ht2, bt1, bt2,
      e_angle1, e_angle2,
    };
  }, [er1, er2, mur1, mur2, dn1, et1, bn1, ht1, rhoS, jsT]);

  const fmt = (v: number) => Math.abs(v) < 0.01 ? v.toExponential(3) : v.toFixed(4);

  return (
    <div className="module">
      <h2>Boundary Conditions</h2>
      <p>Explore how E, D, B, H fields behave at the interface between two media.</p>

      <div className="controls" style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: 16 }}>
        <div>
          <h4>Medium 1</h4>
          <label>εᵣ₁: <input type="range" min={1} max={100} step={0.5} value={er1}
            onChange={e => setEr1(+e.target.value)} /> {er1}</label>
          <label>μᵣ₁: <input type="range" min={1} max={1000} step={1} value={mur1}
            onChange={e => setMur1(+e.target.value)} /> {mur1}</label>
        </div>
        <div>
          <h4>Medium 2</h4>
          <label>εᵣ₂: <input type="range" min={1} max={100} step={0.5} value={er2}
            onChange={e => setEr2(+e.target.value)} /> {er2}</label>
          <label>μᵣ₂: <input type="range" min={1} max={1000} step={1} value={mur2}
            onChange={e => setMur2(+e.target.value)} /> {mur2}</label>
        </div>
      </div>

      <div className="controls">
        <h4>Fields in Medium 1</h4>
        <label>D_n (C/m²): <input type="number" value={dn1} step={0.1}
          onChange={e => setDn1(+e.target.value)} style={{ width: 80 }} /></label>
        <label>E_t (V/m): <input type="number" value={et1} step={1}
          onChange={e => setEt1(+e.target.value)} style={{ width: 80 }} /></label>
        <label>B_n (T): <input type="number" value={bn1} step={0.1}
          onChange={e => setBn1(+e.target.value)} style={{ width: 80 }} /></label>
        <label>H_t (A/m): <input type="number" value={ht1} step={10}
          onChange={e => setHt1(+e.target.value)} style={{ width: 80 }} /></label>
        <label>ρ_s (C/m²): <input type="number" value={rhoS} step={0.01}
          onChange={e => setRhoS(+e.target.value)} style={{ width: 80 }} /></label>
        <label>J_s (A/m): <input type="number" value={jsT} step={1}
          onChange={e => setJsT(+e.target.value)} style={{ width: 80 }} /></label>
      </div>

      <div style={{ overflowX: 'auto', marginTop: 16 }}>
        <table style={{ width: '100%', borderCollapse: 'collapse', fontSize: 14 }}>
          <thead>
            <tr style={{ borderBottom: '2px solid #2196F3' }}>
              <th style={{ padding: 8, textAlign: 'left' }}>Boundary Condition</th>
              <th style={{ padding: 8 }}>Medium 1</th>
              <th style={{ padding: 8 }}>Medium 2</th>
              <th style={{ padding: 8, textAlign: 'left' }}>Rule</th>
            </tr>
          </thead>
          <tbody>
            <tr style={{ background: '#f9f9f9' }}>
              <td style={{ padding: 8, fontWeight: 'bold' }}>D_n</td>
              <td style={{ padding: 8, textAlign: 'center' }}>{fmt(results.dn1)}</td>
              <td style={{ padding: 8, textAlign: 'center' }}>{fmt(results.dn2)}</td>
              <td style={{ padding: 8 }}>D₁ₙ − D₂ₙ = ρₛ</td>
            </tr>
            <tr>
              <td style={{ padding: 8, fontWeight: 'bold' }}>E_n</td>
              <td style={{ padding: 8, textAlign: 'center' }}>{fmt(results.en1)}</td>
              <td style={{ padding: 8, textAlign: 'center' }}>{fmt(results.en2)}</td>
              <td style={{ padding: 8 }}>E_n = D_n / (ε₀εᵣ)</td>
            </tr>
            <tr style={{ background: '#f9f9f9' }}>
              <td style={{ padding: 8, fontWeight: 'bold' }}>E_t</td>
              <td style={{ padding: 8, textAlign: 'center' }}>{fmt(results.et1)}</td>
              <td style={{ padding: 8, textAlign: 'center' }}>{fmt(results.et2)}</td>
              <td style={{ padding: 8 }}>E₁ₜ = E₂ₜ (always)</td>
            </tr>
            <tr>
              <td style={{ padding: 8, fontWeight: 'bold' }}>D_t</td>
              <td style={{ padding: 8, textAlign: 'center' }}>{fmt(results.dt1)}</td>
              <td style={{ padding: 8, textAlign: 'center' }}>{fmt(results.dt2)}</td>
              <td style={{ padding: 8 }}>D_t = ε₀εᵣ E_t</td>
            </tr>
            <tr style={{ background: '#E3F2FD' }}>
              <td style={{ padding: 8, fontWeight: 'bold' }}>B_n</td>
              <td style={{ padding: 8, textAlign: 'center' }}>{fmt(results.bn1)}</td>
              <td style={{ padding: 8, textAlign: 'center' }}>{fmt(results.bn2)}</td>
              <td style={{ padding: 8 }}>B₁ₙ = B₂ₙ (always)</td>
            </tr>
            <tr>
              <td style={{ padding: 8, fontWeight: 'bold' }}>H_n</td>
              <td style={{ padding: 8, textAlign: 'center' }}>{fmt(results.hn1)}</td>
              <td style={{ padding: 8, textAlign: 'center' }}>{fmt(results.hn2)}</td>
              <td style={{ padding: 8 }}>H_n = B_n / (μ₀μᵣ)</td>
            </tr>
            <tr style={{ background: '#E3F2FD' }}>
              <td style={{ padding: 8, fontWeight: 'bold' }}>H_t</td>
              <td style={{ padding: 8, textAlign: 'center' }}>{fmt(results.ht1)}</td>
              <td style={{ padding: 8, textAlign: 'center' }}>{fmt(results.ht2)}</td>
              <td style={{ padding: 8 }}>H₁ₜ − H₂ₜ = Jₛ</td>
            </tr>
            <tr>
              <td style={{ padding: 8, fontWeight: 'bold' }}>B_t</td>
              <td style={{ padding: 8, textAlign: 'center' }}>{fmt(results.bt1)}</td>
              <td style={{ padding: 8, textAlign: 'center' }}>{fmt(results.bt2)}</td>
              <td style={{ padding: 8 }}>B_t = μ₀μᵣ H_t</td>
            </tr>
          </tbody>
        </table>
      </div>

      <div className="results-grid" style={{ marginTop: 16 }}>
        <div className="result-card">
          <span className="label">E-field angle (Med 1)</span>
          <span className="value">{results.e_angle1.toFixed(1)}° from normal</span>
        </div>
        <div className="result-card">
          <span className="label">E-field angle (Med 2)</span>
          <span className="value">{results.e_angle2.toFixed(1)}° from normal</span>
        </div>
      </div>

      <div className="theory">
        <h3>Summary</h3>
        <p><strong>Tangential E:</strong> Always continuous (E₁ₜ = E₂ₜ)</p>
        <p><strong>Normal D:</strong> Discontinuous by ρₛ (D₁ₙ − D₂ₙ = ρₛ)</p>
        <p><strong>Normal B:</strong> Always continuous (B₁ₙ = B₂ₙ)</p>
        <p><strong>Tangential H:</strong> Discontinuous by Jₛ (H₁ₜ − H₂ₜ = Jₛ)</p>
        <p><strong>Refraction:</strong> tan θ₁/tan θ₂ = ε₁/ε₂</p>
      </div>
    </div>
  );
}

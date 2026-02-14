import { useState, useMemo } from 'react';

export function LorentzForceModule() {
  const [q, setQ] = useState(1.6e-19);
  const [vx, setVx] = useState(1e6);
  const [vy, setVy] = useState(0);
  const [vz, setVz] = useState(0);
  const [ex, setEx] = useState(0);
  const [ey, setEy] = useState(100);
  const [ez, setEz] = useState(0);
  const [bx, setBx] = useState(0);
  const [by, setBy] = useState(0);
  const [bz, setBz] = useState(0.1);
  const [mass, setMass] = useState(9.109e-31);

  const presets: Record<string, { q: number; m: number; label: string }> = {
    electron: { q: -1.6e-19, m: 9.109e-31, label: 'Electron' },
    proton: { q: 1.6e-19, m: 1.673e-27, label: 'Proton' },
    alpha: { q: 3.2e-19, m: 6.646e-27, label: 'Alpha particle' },
  };

  const results = useMemo(() => {
    // F = q(E + v × B)
    const vCrossB_x = vy * bz - vz * by;
    const vCrossB_y = vz * bx - vx * bz;
    const vCrossB_z = vx * by - vy * bx;

    const fe_x = q * ex;
    const fe_y = q * ey;
    const fe_z = q * ez;

    const fb_x = q * vCrossB_x;
    const fb_y = q * vCrossB_y;
    const fb_z = q * vCrossB_z;

    const f_x = fe_x + fb_x;
    const f_y = fe_y + fb_y;
    const f_z = fe_z + fb_z;

    const fMag = Math.sqrt(f_x * f_x + f_y * f_y + f_z * f_z);
    const feMag = Math.sqrt(fe_x * fe_x + fe_y * fe_y + fe_z * fe_z);
    const fbMag = Math.sqrt(fb_x * fb_x + fb_y * fb_y + fb_z * fb_z);

    const accel = fMag / mass;
    const vMag = Math.sqrt(vx * vx + vy * vy + vz * vz);
    const bMag = Math.sqrt(bx * bx + by * by + bz * bz);

    // Cyclotron radius and frequency (B-field only)
    const vPerp = Math.sqrt(
      (vy * bz - vz * by) ** 2 + (vz * bx - vx * bz) ** 2 + (vx * by - vy * bx) ** 2
    ) / bMag;
    const cyclotronRadius = bMag > 0 ? mass * vPerp / (Math.abs(q) * bMag) : Infinity;
    const cyclotronFreq = bMag > 0 ? Math.abs(q) * bMag / (2 * Math.PI * mass) : 0;

    return {
      fe: { x: fe_x, y: fe_y, z: fe_z, mag: feMag },
      fb: { x: fb_x, y: fb_y, z: fb_z, mag: fbMag },
      f: { x: f_x, y: f_y, z: f_z, mag: fMag },
      accel, vMag, bMag, cyclotronRadius, cyclotronFreq,
    };
  }, [q, vx, vy, vz, ex, ey, ez, bx, by, bz, mass]);

  const fmt = (v: number) => {
    if (Math.abs(v) === 0) return '0';
    if (Math.abs(v) < 1e-3 || Math.abs(v) > 1e6) return v.toExponential(3);
    return v.toFixed(4);
  };

  return (
    <div className="module">
      <h2>Lorentz Force</h2>
      <p>F = q(E + v × B) — the complete electromagnetic force on a moving charged particle.</p>

      <div className="controls">
        <label>Particle:
          <select onChange={e => {
            const p = presets[e.target.value];
            if (p) { setQ(p.q); setMass(p.m); }
          }}>
            {Object.entries(presets).map(([k, v]) => <option key={k} value={k}>{v.label}</option>)}
          </select>
        </label>

        <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr 1fr', gap: 12 }}>
          <div>
            <h4>Velocity (m/s)</h4>
            <label>vx: <input type="number" value={vx} onChange={e => setVx(+e.target.value)} style={{ width: 90 }} /></label>
            <label>vy: <input type="number" value={vy} onChange={e => setVy(+e.target.value)} style={{ width: 90 }} /></label>
            <label>vz: <input type="number" value={vz} onChange={e => setVz(+e.target.value)} style={{ width: 90 }} /></label>
          </div>
          <div>
            <h4>E-field (V/m)</h4>
            <label>Ex: <input type="number" value={ex} onChange={e => setEx(+e.target.value)} style={{ width: 90 }} /></label>
            <label>Ey: <input type="number" value={ey} onChange={e => setEy(+e.target.value)} style={{ width: 90 }} /></label>
            <label>Ez: <input type="number" value={ez} onChange={e => setEz(+e.target.value)} style={{ width: 90 }} /></label>
          </div>
          <div>
            <h4>B-field (T)</h4>
            <label>Bx: <input type="number" value={bx} step={0.01} onChange={e => setBx(+e.target.value)} style={{ width: 90 }} /></label>
            <label>By: <input type="number" value={by} step={0.01} onChange={e => setBy(+e.target.value)} style={{ width: 90 }} /></label>
            <label>Bz: <input type="number" value={bz} step={0.01} onChange={e => setBz(+e.target.value)} style={{ width: 90 }} /></label>
          </div>
        </div>
      </div>

      <div className="results-grid">
        <div className="result-card">
          <span className="label">F_electric</span>
          <span className="value">({fmt(results.fe.x)}, {fmt(results.fe.y)}, {fmt(results.fe.z)}) N</span>
        </div>
        <div className="result-card">
          <span className="label">|F_E|</span>
          <span className="value">{results.fe.mag.toExponential(3)} N</span>
        </div>
        <div className="result-card">
          <span className="label">F_magnetic</span>
          <span className="value">({fmt(results.fb.x)}, {fmt(results.fb.y)}, {fmt(results.fb.z)}) N</span>
        </div>
        <div className="result-card">
          <span className="label">|F_B|</span>
          <span className="value">{results.fb.mag.toExponential(3)} N</span>
        </div>
        <div className="result-card">
          <span className="label">F_total</span>
          <span className="value">({fmt(results.f.x)}, {fmt(results.f.y)}, {fmt(results.f.z)}) N</span>
        </div>
        <div className="result-card">
          <span className="label">|F_total|</span>
          <span className="value">{results.f.mag.toExponential(3)} N</span>
        </div>
        <div className="result-card">
          <span className="label">Acceleration</span>
          <span className="value">{results.accel.toExponential(3)} m/s²</span>
        </div>
        <div className="result-card">
          <span className="label">Cyclotron radius</span>
          <span className="value">{isFinite(results.cyclotronRadius)
            ? results.cyclotronRadius < 0.01
              ? (results.cyclotronRadius * 1e3).toFixed(3) + ' mm'
              : results.cyclotronRadius.toFixed(4) + ' m'
            : '∞'}</span>
        </div>
        <div className="result-card">
          <span className="label">Cyclotron frequency</span>
          <span className="value">{results.cyclotronFreq >= 1e9
            ? (results.cyclotronFreq / 1e9).toFixed(3) + ' GHz'
            : results.cyclotronFreq >= 1e6
            ? (results.cyclotronFreq / 1e6).toFixed(3) + ' MHz'
            : results.cyclotronFreq.toExponential(3) + ' Hz'}</span>
        </div>
      </div>

      <div className="theory">
        <h3>Theory</h3>
        <p><strong>Lorentz force:</strong> F = qE + qv × B</p>
        <p><strong>Electric force:</strong> F_E = qE (parallel to E, does work)</p>
        <p><strong>Magnetic force:</strong> F_B = qv × B (perpendicular to v, does no work)</p>
        <p><strong>Cyclotron:</strong> r = mv⊥/(|q|B), f_c = |q|B/(2πm)</p>
      </div>
    </div>
  );
}

import { useState, useMemo } from 'react';
import * as wasm from '../lib/em_wasm';

interface Vec3Result { x: number; y: number; z: number; }

export function VectorOpsModule() {
  const [ax, setAx] = useState(1); const [ay, setAy] = useState(2); const [az, setAz] = useState(3);
  const [bx, setBx] = useState(4); const [by, setBy] = useState(5); const [bz, setBz] = useState(6);

  const results = useMemo(() => {
    try {
      const add = wasm.vector_add(ax, ay, az, bx, by, bz) as Vec3Result;
      const cross = wasm.vector_cross(ax, ay, az, bx, by, bz) as Vec3Result;
      const proj = wasm.vector_project(ax, ay, az, bx, by, bz) as Vec3Result;

      const dot = ax * bx + ay * by + az * bz;
      const magA = Math.sqrt(ax * ax + ay * ay + az * az);
      const magB = Math.sqrt(bx * bx + by * by + bz * bz);
      const magCross = Math.sqrt(cross.x ** 2 + cross.y ** 2 + cross.z ** 2);
      const angle = Math.acos(Math.min(1, Math.max(-1, dot / (magA * magB))));

      const sub = { x: ax - bx, y: ay - by, z: az - bz };
      const unitA = magA > 0 ? { x: ax / magA, y: ay / magA, z: az / magA } : { x: 0, y: 0, z: 0 };

      return { add, cross, proj, dot, magA, magB, magCross, angle, sub, unitA };
    } catch { return null; }
  }, [ax, ay, az, bx, by, bz]);

  const fmt = (v: { x: number; y: number; z: number }) =>
    `(${v.x.toFixed(3)}, ${v.y.toFixed(3)}, ${v.z.toFixed(3)})`;

  return (
    <div className="module">
      <h2>Vector Operations</h2>
      <p>Complete vector algebra: addition, subtraction, dot product, cross product, projection, angle, unit vectors.</p>

      <div className="controls" style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '16px' }}>
        <div>
          <h4 style={{ margin: '0 0 8px' }}>Vector A</h4>
          <label>x: <input type="number" value={ax} onChange={e => setAx(+e.target.value)} style={{ width: 80 }} /></label>
          <label>y: <input type="number" value={ay} onChange={e => setAy(+e.target.value)} style={{ width: 80 }} /></label>
          <label>z: <input type="number" value={az} onChange={e => setAz(+e.target.value)} style={{ width: 80 }} /></label>
        </div>
        <div>
          <h4 style={{ margin: '0 0 8px' }}>Vector B</h4>
          <label>x: <input type="number" value={bx} onChange={e => setBx(+e.target.value)} style={{ width: 80 }} /></label>
          <label>y: <input type="number" value={by} onChange={e => setBy(+e.target.value)} style={{ width: 80 }} /></label>
          <label>z: <input type="number" value={bz} onChange={e => setBz(+e.target.value)} style={{ width: 80 }} /></label>
        </div>
      </div>

      {results && (
        <div className="results-table" style={{ marginTop: 20 }}>
          <table style={{ width: '100%', borderCollapse: 'collapse' }}>
            <thead>
              <tr style={{ borderBottom: '2px solid #ddd' }}>
                <th style={{ textAlign: 'left', padding: 8 }}>Operation</th>
                <th style={{ textAlign: 'left', padding: 8 }}>Result</th>
              </tr>
            </thead>
            <tbody>
              <tr><td style={{ padding: 8, fontWeight: 'bold' }}>|A|</td><td style={{ padding: 8 }}>{results.magA.toFixed(4)}</td></tr>
              <tr style={{ background: '#f9f9f9' }}><td style={{ padding: 8, fontWeight: 'bold' }}>|B|</td><td style={{ padding: 8 }}>{results.magB.toFixed(4)}</td></tr>
              <tr><td style={{ padding: 8, fontWeight: 'bold' }}> Â (unit)</td><td style={{ padding: 8 }}>{fmt(results.unitA)}</td></tr>
              <tr style={{ background: '#f9f9f9' }}><td style={{ padding: 8, fontWeight: 'bold' }}>A + B</td><td style={{ padding: 8 }}>{fmt(results.add)}</td></tr>
              <tr><td style={{ padding: 8, fontWeight: 'bold' }}>A − B</td><td style={{ padding: 8 }}>{fmt(results.sub)}</td></tr>
              <tr style={{ background: '#f9f9f9' }}><td style={{ padding: 8, fontWeight: 'bold' }}>A · B (dot)</td><td style={{ padding: 8 }}>{results.dot.toFixed(4)}</td></tr>
              <tr><td style={{ padding: 8, fontWeight: 'bold' }}>A × B (cross)</td><td style={{ padding: 8 }}>{fmt(results.cross)}</td></tr>
              <tr style={{ background: '#f9f9f9' }}><td style={{ padding: 8, fontWeight: 'bold' }}>|A × B|</td><td style={{ padding: 8 }}>{results.magCross.toFixed(4)}</td></tr>
              <tr><td style={{ padding: 8, fontWeight: 'bold' }}>proj_B(A)</td><td style={{ padding: 8 }}>{fmt(results.proj)}</td></tr>
              <tr style={{ background: '#f9f9f9' }}><td style={{ padding: 8, fontWeight: 'bold' }}>Angle (A,B)</td><td style={{ padding: 8 }}>{(results.angle * 180 / Math.PI).toFixed(2)}°</td></tr>
            </tbody>
          </table>
        </div>
      )}

      <div className="theory">
        <h3>Identities</h3>
        <p><strong>Dot product:</strong> A·B = |A||B|cos θ = AxBx + AyBy + AzBz</p>
        <p><strong>Cross product:</strong> |A×B| = |A||B|sin θ, direction by right-hand rule</p>
        <p><strong>Projection:</strong> proj_B(A) = (A·B/|B|²)B</p>
        <p><strong>Triple product:</strong> A·(B×C) = volume of parallelepiped</p>
      </div>
    </div>
  );
}

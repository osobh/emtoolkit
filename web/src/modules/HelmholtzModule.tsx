import { useState, useMemo } from 'react';
import Plot from 'react-plotly.js';
import * as wasm from '../lib/em_wasm';

export function HelmholtzModule() {
  const [radius, setRadius] = useState(0.1);
  const [current, setCurrent] = useState(1.0);
  const [turns, setTurns] = useState(10);
  const [separation, setSeparation] = useState(1.0); // ratio to radius

  const data = useMemo(() => {
    try {
      const sep = separation * radius;
      const zMax = sep * 2;
      const result = wasm.helmholtz_coil(radius, current, turns, -zMax, zMax, 300) as {
        z: number[]; bz: number[];
      };
      // Also compute single loop for comparison
      const single = wasm.current_loop_on_axis(radius, current * turns, -zMax, zMax, 300) as {
        z: number[]; bz: number[];
      };
      return { helmholtz: result, single };
    } catch { return null; }
  }, [radius, current, turns, separation]);

  const uniformity = useMemo(() => {
    if (!data) return null;
    const { bz } = data.helmholtz;
    const center = Math.floor(bz.length / 2);
    const bCenter = bz[center];
    const span = Math.floor(bz.length * 0.1); // ±10% of range
    let maxDev = 0;
    for (let i = center - span; i <= center + span; i++) {
      if (i >= 0 && i < bz.length) {
        const dev = Math.abs((bz[i] - bCenter) / bCenter);
        if (dev > maxDev) maxDev = dev;
      }
    }
    return { bCenter, maxDev };
  }, [data]);

  return (
    <div className="module">
      <h2>Helmholtz Coil</h2>
      <p>Two co-axial loops separated by distance d. At d = R (Helmholtz condition), the field is highly uniform at the center.</p>

      <div className="controls">
        <label>
          Radius (m): <input type="range" min={0.01} max={0.5} step={0.01} value={radius}
            onChange={e => setRadius(+e.target.value)} /> {radius.toFixed(2)}
        </label>
        <label>
          Current (A): <input type="range" min={0.1} max={20} step={0.1} value={current}
            onChange={e => setCurrent(+e.target.value)} /> {current.toFixed(1)}
        </label>
        <label>
          Turns per coil: <input type="range" min={1} max={100} step={1} value={turns}
            onChange={e => setTurns(+e.target.value)} /> {turns}
        </label>
        <label>
          Separation (×R): <input type="range" min={0.2} max={3.0} step={0.05} value={separation}
            onChange={e => setSeparation(+e.target.value)} /> {separation.toFixed(2)}
          {Math.abs(separation - 1.0) < 0.06 && <span className="badge">✓ Helmholtz</span>}
        </label>
      </div>

      {uniformity && (
        <div className="results-grid">
          <div className="result-card">
            <span className="label">B at center</span>
            <span className="value">{(uniformity.bCenter * 1e6).toFixed(1)} μT</span>
          </div>
          <div className="result-card">
            <span className="label">Uniformity (±10% span)</span>
            <span className="value">{(uniformity.maxDev * 100).toFixed(2)}% deviation</span>
          </div>
          <div className="result-card">
            <span className="label">Separation</span>
            <span className="value">{(separation * radius * 100).toFixed(1)} cm</span>
          </div>
        </div>
      )}

      {data && (
        <Plot
          data={[
            {
              x: data.helmholtz.z.map(z => z * 100),
              y: data.helmholtz.bz.map(b => b * 1e6),
              type: 'scatter',
              mode: 'lines',
              name: 'Helmholtz pair',
              line: { color: '#2196F3', width: 2 },
            },
            {
              x: data.single.z.map(z => z * 100),
              y: data.single.bz.map(b => b * 1e6),
              type: 'scatter',
              mode: 'lines',
              name: 'Single loop',
              line: { color: '#FF9800', width: 2, dash: 'dash' },
            },
          ]}
          layout={{
            title: 'Axial B-Field: Helmholtz vs Single Loop',
            xaxis: { title: 'z (cm)' },
            yaxis: { title: 'Bz (μT)' },
            margin: { t: 40, r: 20, b: 50, l: 60 },
            height: 400,
            legend: { x: 0.02, y: 0.98 },
          }}
          config={{ responsive: true }}
          style={{ width: '100%' }}
        />
      )}

      <div className="theory">
        <h3>Helmholtz Condition</h3>
        <p>When two identical coils are separated by a distance equal to their radius (d = R),
          the second derivative of B vanishes at the midpoint, creating a region of nearly uniform field.</p>
        <p><strong>B at center:</strong> B = (4/5)^(3/2) μ₀ N I / R ≈ 0.7155 μ₀ N I / R</p>
        <p>Adjust the separation slider to see how the field profile changes. The Helmholtz sweet spot at d/R = 1.0 gives the flattest central region.</p>
      </div>
    </div>
  );
}

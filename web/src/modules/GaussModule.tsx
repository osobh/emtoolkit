import { useState, useMemo } from 'react';
import Plot from 'react-plotly.js';
import * as wasm from '../lib/em_wasm';

type Geometry = 'sphere' | 'line' | 'surface';

export function GaussModule() {
  const [geometry, setGeometry] = useState<Geometry>('sphere');
  const [charge, setCharge] = useState(1e-6);
  const [radius, setRadius] = useState(0.1);
  const [epsilonR, setEpsilonR] = useState(1.0);
  const [rhoL, setRhoL] = useState(1e-9);

  const sphereData = useMemo(() => {
    if (geometry !== 'sphere') return null;
    try {
      return wasm.gauss_sphere_profile(charge, radius, epsilonR, radius * 3, 300) as {
        r: number[]; e_field: number[]; potential: number[];
      };
    } catch { return null; }
  }, [geometry, charge, radius, epsilonR]);

  const lineData = useMemo(() => {
    if (geometry !== 'line') return null;
    try {
      return wasm.gauss_line_charge(rhoL, epsilonR, 0.001, 0.5, 300) as {
        rho: number[]; e_field: number[];
      };
    } catch { return null; }
  }, [geometry, rhoL, epsilonR]);

  const surfaceE = useMemo(() => {
    if (geometry !== 'surface') return null;
    const eps0 = 8.854187817e-12;
    const rhoS = charge; // reuse charge slider as surface charge density
    return rhoS / (2.0 * eps0 * epsilonR);
  }, [geometry, charge, epsilonR]);

  return (
    <div className="module">
      <h2>Gauss's Law Explorer</h2>
      <p>Visualize E-field from symmetric charge distributions using Gauss's law.</p>

      <div className="controls">
        <label>
          Geometry:
          <select value={geometry} onChange={e => setGeometry(e.target.value as Geometry)}>
            <option value="sphere">Uniformly Charged Sphere</option>
            <option value="line">Infinite Line Charge</option>
            <option value="surface">Infinite Surface Charge</option>
          </select>
        </label>

        <label>
          εᵣ: <input type="range" min={1} max={20} step={0.5} value={epsilonR}
            onChange={e => setEpsilonR(+e.target.value)} /> {epsilonR.toFixed(1)}
        </label>

        {geometry === 'sphere' && (
          <>
            <label>Total charge Q (μC): <input type="range" min={0.1} max={100} step={0.1}
              value={charge * 1e6} onChange={e => setCharge(+e.target.value * 1e-6)} />
              {(charge * 1e6).toFixed(1)}</label>
            <label>Sphere radius R (cm): <input type="range" min={1} max={50} step={1}
              value={radius * 100} onChange={e => setRadius(+e.target.value / 100)} />
              {(radius * 100).toFixed(0)}</label>
          </>
        )}

        {geometry === 'line' && (
          <label>ρ_L (nC/m): <input type="range" min={0.1} max={100} step={0.1}
            value={rhoL * 1e9} onChange={e => setRhoL(+e.target.value * 1e-9)} />
            {(rhoL * 1e9).toFixed(1)}</label>
        )}

        {geometry === 'surface' && (
          <label>ρ_s (μC/m²): <input type="range" min={0.01} max={10} step={0.01}
            value={charge * 1e6} onChange={e => setCharge(+e.target.value * 1e-6)} />
            {(charge * 1e6).toFixed(2)}</label>
        )}
      </div>

      {geometry === 'surface' && surfaceE !== null && (
        <div className="results-grid">
          <div className="result-card">
            <span className="label">E (each side)</span>
            <span className="value">{surfaceE.toExponential(3)} V/m</span>
          </div>
          <div className="result-card">
            <span className="label">Total E between plates</span>
            <span className="value">{(surfaceE * 2).toExponential(3)} V/m</span>
          </div>
        </div>
      )}

      {sphereData && (
        <>
          <Plot
            data={[{
              x: sphereData.r.map(r => r * 100),
              y: sphereData.e_field,
              type: 'scatter', mode: 'lines', name: 'E(r)',
              line: { color: '#F44336', width: 2 },
            }]}
            layout={{
              title: 'E-Field: Charged Sphere (Q = ' + (charge * 1e6).toFixed(1) + ' μC, R = ' + (radius * 100).toFixed(0) + ' cm)',
              xaxis: { title: 'r (cm)' },
              yaxis: { title: 'E (V/m)' },
              margin: { t: 40, r: 20, b: 50, l: 60 }, height: 350,
              shapes: [{ type: 'line', x0: radius * 100, x1: radius * 100, y0: 0,
                y1: Math.max(...sphereData.e_field) * 1.1,
                line: { color: '#4CAF50', dash: 'dash', width: 1 } }],
              annotations: [{ x: radius * 100, y: Math.max(...sphereData.e_field) * 1.05,
                text: 'R', showarrow: false, font: { color: '#4CAF50' } }],
            }}
            config={{ responsive: true }} style={{ width: '100%' }}
          />
          <Plot
            data={[{
              x: sphereData.r.map(r => r * 100),
              y: sphereData.potential,
              type: 'scatter', mode: 'lines', name: 'V(r)',
              line: { color: '#2196F3', width: 2 },
            }]}
            layout={{
              title: 'Potential V(r)',
              xaxis: { title: 'r (cm)' },
              yaxis: { title: 'V (Volts)' },
              margin: { t: 40, r: 20, b: 50, l: 60 }, height: 300,
            }}
            config={{ responsive: true }} style={{ width: '100%' }}
          />
        </>
      )}

      {lineData && (
        <Plot
          data={[{
            x: lineData.rho.map(r => r * 100),
            y: lineData.e_field,
            type: 'scatter', mode: 'lines', name: 'E(ρ)',
            line: { color: '#F44336', width: 2 },
          }]}
          layout={{
            title: 'E-Field: Line Charge (ρ_L = ' + (rhoL * 1e9).toFixed(1) + ' nC/m)',
            xaxis: { title: 'ρ (cm)' },
            yaxis: { title: 'E (V/m)' },
            margin: { t: 40, r: 20, b: 50, l: 60 }, height: 380,
          }}
          config={{ responsive: true }} style={{ width: '100%' }}
        />
      )}

      <div className="theory">
        <h3>Gauss's Law: ∮ D·dS = Q_enc</h3>
        <p><strong>Sphere:</strong> E = Qr/(4πε₀R³) for r&lt;R, E = Q/(4πε₀r²) for r≥R</p>
        <p><strong>Line charge:</strong> E = ρ_L/(2πε₀ρ)</p>
        <p><strong>Surface charge:</strong> E = ρ_s/(2ε₀) on each side</p>
      </div>
    </div>
  );
}

import { useState, useMemo } from 'react';
import * as wasm from '../lib/em_wasm';

type Geometry = 'parallel_plate' | 'coaxial' | 'spherical' | 'isolated_sphere';

interface CapResult {
  capacitance: number;
  energy: number;
  charge: number;
  voltage: number;
  geometry: string;
}

function formatCapacitance(c: number): string {
  if (c >= 1e-3) return (c * 1e3).toFixed(3) + ' mF';
  if (c >= 1e-6) return (c * 1e6).toFixed(3) + ' μF';
  if (c >= 1e-9) return (c * 1e9).toFixed(3) + ' nF';
  return (c * 1e12).toFixed(3) + ' pF';
}

function formatEnergy(w: number): string {
  if (w >= 1) return w.toFixed(3) + ' J';
  if (w >= 1e-3) return (w * 1e3).toFixed(3) + ' mJ';
  if (w >= 1e-6) return (w * 1e6).toFixed(3) + ' μJ';
  return (w * 1e9).toFixed(3) + ' nJ';
}

export function CapacitanceModule() {
  const [geometry, setGeometry] = useState<Geometry>('parallel_plate');
  const [area, setArea] = useState(0.01);
  const [separation, setSeparation] = useState(0.001);
  const [innerR, setInnerR] = useState(0.005);
  const [outerR, setOuterR] = useState(0.02);
  const [length, setLength] = useState(1.0);
  const [radius, setRadius] = useState(0.1);
  const [epsilonR, setEpsilonR] = useState(1.0);
  const [voltage, setVoltage] = useState(100);

  const result: CapResult | null = useMemo(() => {
    try {
      const params: Record<string, unknown> = { epsilon_r: epsilonR, voltage };
      switch (geometry) {
        case 'parallel_plate':
          params.area = area;
          params.separation = separation;
          break;
        case 'coaxial':
          params.inner_radius = innerR;
          params.outer_radius = outerR;
          params.length = length;
          break;
        case 'spherical':
          params.inner_radius = innerR;
          params.outer_radius = outerR;
          break;
        case 'isolated_sphere':
          params.radius = radius;
          break;
      }
      return wasm.capacitance_calc(geometry, JSON.stringify(params)) as CapResult;
    } catch { return null; }
  }, [geometry, area, separation, innerR, outerR, length, radius, epsilonR, voltage]);

  return (
    <div className="module">
      <h2>Capacitance Calculator</h2>
      <p>Compute capacitance, stored energy, and charge for common capacitor geometries.</p>

      <div className="controls">
        <label>
          Geometry:
          <select value={geometry} onChange={e => setGeometry(e.target.value as Geometry)}>
            <option value="parallel_plate">Parallel Plate</option>
            <option value="coaxial">Coaxial (cylindrical)</option>
            <option value="spherical">Spherical</option>
            <option value="isolated_sphere">Isolated Sphere</option>
          </select>
        </label>

        {geometry === 'parallel_plate' && (
          <>
            <label>Area (m²): <input type="range" min={0.001} max={1} step={0.001} value={area}
              onChange={e => setArea(+e.target.value)} /> {area.toFixed(3)}</label>
            <label>Separation (mm): <input type="range" min={0.1} max={50} step={0.1}
              value={separation * 1000} onChange={e => setSeparation(+e.target.value / 1000)} />
              {(separation * 1000).toFixed(1)}</label>
          </>
        )}

        {(geometry === 'coaxial' || geometry === 'spherical') && (
          <>
            <label>Inner radius (mm): <input type="range" min={1} max={50} step={0.5}
              value={innerR * 1000} onChange={e => setInnerR(+e.target.value / 1000)} />
              {(innerR * 1000).toFixed(1)}</label>
            <label>Outer radius (mm): <input type="range" min={5} max={100} step={0.5}
              value={outerR * 1000} onChange={e => setOuterR(+e.target.value / 1000)} />
              {(outerR * 1000).toFixed(1)}</label>
          </>
        )}

        {geometry === 'coaxial' && (
          <label>Length (m): <input type="range" min={0.01} max={10} step={0.01} value={length}
            onChange={e => setLength(+e.target.value)} /> {length.toFixed(2)}</label>
        )}

        {geometry === 'isolated_sphere' && (
          <label>Radius (m): <input type="range" min={0.01} max={1} step={0.01} value={radius}
            onChange={e => setRadius(+e.target.value)} /> {radius.toFixed(2)}</label>
        )}

        <label>Dielectric εᵣ: <input type="range" min={1} max={100} step={0.5} value={epsilonR}
          onChange={e => setEpsilonR(+e.target.value)} /> {epsilonR.toFixed(1)}</label>

        <label>Voltage (V): <input type="range" min={1} max={10000} step={1} value={voltage}
          onChange={e => setVoltage(+e.target.value)} /> {voltage}</label>
      </div>

      {result && (
        <div className="results-grid">
          <div className="result-card">
            <span className="label">Capacitance</span>
            <span className="value">{formatCapacitance(result.capacitance)}</span>
          </div>
          <div className="result-card">
            <span className="label">Stored Energy</span>
            <span className="value">{formatEnergy(result.energy)}</span>
          </div>
          <div className="result-card">
            <span className="label">Charge</span>
            <span className="value">{result.charge < 1e-6
              ? (result.charge * 1e9).toFixed(3) + ' nC'
              : result.charge < 1e-3
              ? (result.charge * 1e6).toFixed(3) + ' μC'
              : (result.charge * 1e3).toFixed(3) + ' mC'}</span>
          </div>
          <div className="result-card">
            <span className="label">E-field (V/m)</span>
            <span className="value">{geometry === 'parallel_plate'
              ? (voltage / separation).toExponential(3)
              : '—'}</span>
          </div>
        </div>
      )}

      <div className="theory">
        <h3>Formulas</h3>
        <p><strong>Parallel plate:</strong> C = ε₀εᵣA/d</p>
        <p><strong>Coaxial:</strong> C = 2πε₀εᵣℓ / ln(b/a)</p>
        <p><strong>Spherical:</strong> C = 4πε₀εᵣab / (b−a)</p>
        <p><strong>Isolated sphere:</strong> C = 4πε₀a</p>
        <p><strong>Energy:</strong> W = ½CV²</p>
        <p>Common dielectrics: Air (1.0), Teflon (2.1), Paper (3.5), Glass (5-10), Water (80), BaTiO₃ (1200+)</p>
      </div>
    </div>
  );
}

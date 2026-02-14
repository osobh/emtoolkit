import { useState, useMemo } from 'react';
import * as wasm from '../lib/em_wasm';

type Geometry = 'solenoid' | 'toroid' | 'coaxial' | 'parallel_wires';

interface IndResult {
  inductance: number;
  energy: number;
  current: number;
  time_constant: number;
}

function formatL(l: number): string {
  if (l >= 1) return l.toFixed(3) + ' H';
  if (l >= 1e-3) return (l * 1e3).toFixed(3) + ' mH';
  if (l >= 1e-6) return (l * 1e6).toFixed(3) + ' μH';
  return (l * 1e9).toFixed(3) + ' nH';
}

export function InductanceModule() {
  const [geometry, setGeometry] = useState<Geometry>('solenoid');
  const [turns, setTurns] = useState(100);
  const [length, setLength] = useState(0.1);
  const [radius, setRadius] = useState(0.02);
  const [innerR, setInnerR] = useState(0.05);
  const [outerR, setOuterR] = useState(0.08);
  const [height, setHeight] = useState(0.02);
  const [wireR, setWireR] = useState(0.001);
  const [sep, setSep] = useState(0.1);
  const [muR, setMuR] = useState(1.0);
  const [current, setCurrent] = useState(1.0);
  const [resistance, setResistance] = useState(100.0);

  const result: IndResult | null = useMemo(() => {
    try {
      const params: Record<string, unknown> = { mu_r: muR, current, resistance };
      switch (geometry) {
        case 'solenoid': Object.assign(params, { turns, length, radius }); break;
        case 'toroid': Object.assign(params, { turns, inner_radius: innerR, outer_radius: outerR, height }); break;
        case 'coaxial': Object.assign(params, { inner_radius: innerR, outer_radius: outerR, length }); break;
        case 'parallel_wires': Object.assign(params, { wire_radius: wireR, separation: sep, length }); break;
      }
      return wasm.inductance_calc(geometry, JSON.stringify(params)) as IndResult;
    } catch { return null; }
  }, [geometry, turns, length, radius, innerR, outerR, height, wireR, sep, muR, current, resistance]);

  return (
    <div className="module">
      <h2>Inductance Calculator</h2>
      <p>Compute inductance, stored energy, and RL time constant for common geometries.</p>

      <div className="controls">
        <label>
          Geometry:
          <select value={geometry} onChange={e => setGeometry(e.target.value as Geometry)}>
            <option value="solenoid">Solenoid</option>
            <option value="toroid">Toroid</option>
            <option value="coaxial">Coaxial Cable</option>
            <option value="parallel_wires">Parallel Wires</option>
          </select>
        </label>

        {(geometry === 'solenoid' || geometry === 'toroid') && (
          <label>Turns: <input type="range" min={10} max={1000} step={10} value={turns}
            onChange={e => setTurns(+e.target.value)} /> {turns}</label>
        )}

        {geometry === 'solenoid' && (
          <>
            <label>Length (m): <input type="range" min={0.01} max={1} step={0.01} value={length}
              onChange={e => setLength(+e.target.value)} /> {length.toFixed(2)}</label>
            <label>Radius (mm): <input type="range" min={1} max={100} step={1}
              value={radius * 1000} onChange={e => setRadius(+e.target.value / 1000)} />
              {(radius * 1000).toFixed(0)}</label>
          </>
        )}

        {geometry === 'toroid' && (
          <>
            <label>Inner R (mm): <input type="range" min={10} max={100} step={1}
              value={innerR * 1000} onChange={e => setInnerR(+e.target.value / 1000)} />
              {(innerR * 1000).toFixed(0)}</label>
            <label>Outer R (mm): <input type="range" min={20} max={200} step={1}
              value={outerR * 1000} onChange={e => setOuterR(+e.target.value / 1000)} />
              {(outerR * 1000).toFixed(0)}</label>
            <label>Height (mm): <input type="range" min={5} max={50} step={1}
              value={height * 1000} onChange={e => setHeight(+e.target.value / 1000)} />
              {(height * 1000).toFixed(0)}</label>
          </>
        )}

        {geometry === 'coaxial' && (
          <>
            <label>Inner R (mm): <input type="range" min={0.5} max={10} step={0.1}
              value={innerR * 1000} onChange={e => setInnerR(+e.target.value / 1000)} />
              {(innerR * 1000).toFixed(1)}</label>
            <label>Outer R (mm): <input type="range" min={2} max={30} step={0.5}
              value={outerR * 1000} onChange={e => setOuterR(+e.target.value / 1000)} />
              {(outerR * 1000).toFixed(1)}</label>
            <label>Length (m): <input type="range" min={0.1} max={100} step={0.1} value={length}
              onChange={e => setLength(+e.target.value)} /> {length.toFixed(1)}</label>
          </>
        )}

        {geometry === 'parallel_wires' && (
          <>
            <label>Wire radius (mm): <input type="range" min={0.1} max={5} step={0.1}
              value={wireR * 1000} onChange={e => setWireR(+e.target.value / 1000)} />
              {(wireR * 1000).toFixed(1)}</label>
            <label>Separation (cm): <input type="range" min={1} max={50} step={1}
              value={sep * 100} onChange={e => setSep(+e.target.value / 100)} />
              {(sep * 100).toFixed(0)}</label>
            <label>Length (m): <input type="range" min={0.1} max={100} step={0.1} value={length}
              onChange={e => setLength(+e.target.value)} /> {length.toFixed(1)}</label>
          </>
        )}

        <label>Core μᵣ: <input type="range" min={1} max={5000} step={1} value={muR}
          onChange={e => setMuR(+e.target.value)} /> {muR}</label>
        <label>Current (A): <input type="range" min={0.01} max={20} step={0.01} value={current}
          onChange={e => setCurrent(+e.target.value)} /> {current.toFixed(2)}</label>
        <label>Load R (Ω): <input type="range" min={1} max={10000} step={1} value={resistance}
          onChange={e => setResistance(+e.target.value)} /> {resistance}</label>
      </div>

      {result && (
        <div className="results-grid">
          <div className="result-card">
            <span className="label">Inductance</span>
            <span className="value">{formatL(result.inductance)}</span>
          </div>
          <div className="result-card">
            <span className="label">Stored Energy</span>
            <span className="value">{result.energy < 1e-6
              ? (result.energy * 1e9).toFixed(3) + ' nJ'
              : result.energy < 1e-3
              ? (result.energy * 1e6).toFixed(3) + ' μJ'
              : (result.energy * 1e3).toFixed(3) + ' mJ'}</span>
          </div>
          <div className="result-card">
            <span className="label">RL Time Constant</span>
            <span className="value">{result.time_constant < 1e-6
              ? (result.time_constant * 1e9).toFixed(1) + ' ns'
              : result.time_constant < 1e-3
              ? (result.time_constant * 1e6).toFixed(1) + ' μs'
              : (result.time_constant * 1e3).toFixed(2) + ' ms'}</span>
          </div>
        </div>
      )}

      <div className="theory">
        <h3>Formulas</h3>
        <p><strong>Solenoid:</strong> L = μ₀μᵣN²πr²/ℓ</p>
        <p><strong>Toroid:</strong> L = μ₀μᵣN²h·ln(b/a)/(2π)</p>
        <p><strong>Coaxial:</strong> L/ℓ = μ₀ln(b/a)/(2π)</p>
        <p><strong>Parallel wires:</strong> L/ℓ = μ₀ln(d/a)/π</p>
        <p><strong>Energy:</strong> W = ½LI², <strong>τ = L/R</strong></p>
      </div>
    </div>
  );
}

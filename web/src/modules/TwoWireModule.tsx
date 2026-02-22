import { useState, useMemo } from 'react';
import Plot from 'react-plotly.js';
import { wasm } from '../wasm';

interface TwoWireResult {
  z0: number;
  phase_velocity: number;
  l_per_m: number;
  c_per_m: number;
}

export function TwoWireModule() {
  const [wireRadius, setWireRadius] = useState(1.0); // mm
  const [separation, setSeparation] = useState(10.0); // mm
  const [epsilonR, setEpsilonR] = useState(1.0);

  const result: TwoWireResult | null = useMemo(() => {
    try {
      // Ensure separation > 2*radius for valid geometry
      if (separation <= 2 * wireRadius) return null;
      return wasm.two_wire_params(
        wireRadius * 1e-3,
        separation * 1e-3,
        epsilonR
      ) as TwoWireResult;
    } catch {
      return null;
    }
  }, [wireRadius, separation, epsilonR]);

  // Generate Z₀ vs separation curve
  const z0VsSeparation = useMemo(() => {
    const separations: number[] = [];
    const z0s: number[] = [];
    const minSep = wireRadius * 2.5;
    for (let d = minSep; d <= 50.0; d += 0.5) {
      try {
        const r = wasm.two_wire_params(
          wireRadius * 1e-3,
          d * 1e-3,
          epsilonR
        ) as TwoWireResult;
        separations.push(d);
        z0s.push(r.z0);
      } catch {
        // skip invalid points
      }
    }
    return { separations, z0s };
  }, [wireRadius, epsilonR]);

  const ratio = separation / (2 * wireRadius);
  const isValidGeometry = separation > 2 * wireRadius;

  return (
    <div className="module">
      <h2>Two-Wire Transmission Line</h2>
      <p>
        The classic parallel wire transmission line. Used for balanced transmission,
        twisted pairs, and as a reference geometry for impedance calculations.
      </p>

      <div className="controls">
        <label>
          Wire Radius a (mm):
          <input
            type="range"
            min={0.1}
            max={5.0}
            step={0.1}
            value={wireRadius}
            onChange={(e) => setWireRadius(+e.target.value)}
          />
          {wireRadius.toFixed(1)}
        </label>
        <label>
          Separation d (mm):
          <input
            type="range"
            min={wireRadius * 2.1}
            max={100.0}
            step={0.5}
            value={separation}
            onChange={(e) => setSeparation(+e.target.value)}
          />
          {separation.toFixed(1)}
        </label>
        <label>
          ε_r (surrounding medium):
          <input
            type="range"
            min={1.0}
            max={10.0}
            step={0.1}
            value={epsilonR}
            onChange={(e) => setEpsilonR(+e.target.value)}
          />
          {epsilonR.toFixed(1)}
        </label>
      </div>

      {!isValidGeometry && (
        <div style={{ color: '#F44336', padding: '10px', margin: '10px 0' }}>
          ⚠️ Invalid geometry: Separation must be greater than wire diameter (2a).
        </div>
      )}

      {result && (
        <div className="results-grid">
          <div className="result-card">
            <span className="label">Z₀</span>
            <span className="value">{result.z0.toFixed(2)} Ω</span>
          </div>
          <div className="result-card">
            <span className="label">Phase Velocity</span>
            <span className="value">{(result.phase_velocity / 1e8).toFixed(3)} × 10⁸ m/s</span>
          </div>
          <div className="result-card">
            <span className="label">d/(2a) ratio</span>
            <span className="value">{ratio.toFixed(2)}</span>
          </div>
          <div className="result-card">
            <span className="label">Velocity Factor</span>
            <span className="value">{(result.phase_velocity / 3e8).toFixed(3)}</span>
          </div>
          <div className="result-card">
            <span className="label">L (per m)</span>
            <span className="value">{(result.l_per_m * 1e6).toFixed(3)} µH/m</span>
          </div>
          <div className="result-card">
            <span className="label">C (per m)</span>
            <span className="value">{(result.c_per_m * 1e12).toFixed(2)} pF/m</span>
          </div>
        </div>
      )}

      <Plot
        data={[
          {
            x: z0VsSeparation.separations,
            y: z0VsSeparation.z0s,
            type: 'scatter',
            mode: 'lines',
            name: 'Z₀ vs Separation',
            line: { color: '#4CAF50', width: 2 },
          },
          result
            ? {
                x: [separation],
                y: [result.z0],
                type: 'scatter',
                mode: 'markers',
                name: 'Current',
                marker: { size: 12, color: '#F44336' },
              }
            : null,
        ].filter(Boolean)}
        layout={{
          title: 'Characteristic Impedance vs Wire Separation',
          xaxis: { title: 'Separation d (mm)' },
          yaxis: { title: 'Z₀ (Ω)' },
          margin: { t: 40, r: 20, b: 50, l: 60 },
          height: 350,
          showlegend: true,
          legend: { x: 0.7, y: 0.95 },
        }}
        config={{ responsive: true }}
        style={{ width: '100%' }}
      />

      {/* Cross-section diagram */}
      <div style={{ textAlign: 'center', margin: '20px 0' }}>
        <svg viewBox="0 0 300 120" style={{ width: '100%', maxWidth: 400 }}>
          {/* Left wire */}
          <circle cx="80" cy="60" r={15 + wireRadius * 2} fill="#B8860B" stroke="#8B6914" strokeWidth="2" />
          <circle cx="80" cy="60" r="3" fill="#333" />
          {/* Right wire */}
          <circle cx="220" cy="60" r={15 + wireRadius * 2} fill="#B8860B" stroke="#8B6914" strokeWidth="2" />
          <circle cx="220" cy="60" r="3" fill="#333" />
          {/* Separation dimension */}
          <line x1="80" y1="100" x2="220" y2="100" stroke="#333" strokeWidth="1" />
          <line x1="80" y1="95" x2="80" y2="105" stroke="#333" strokeWidth="1" />
          <line x1="220" y1="95" x2="220" y2="105" stroke="#333" strokeWidth="1" />
          <text x="150" y="115" textAnchor="middle" fontSize="12" fill="#333">d</text>
          {/* Radius dimension */}
          <line x1="80" y1="60" x2={80 + 15 + wireRadius * 2} y2="60" stroke="#333" strokeWidth="1" strokeDasharray="3,2" />
          <text x={80 + (15 + wireRadius * 2) / 2} y="55" textAnchor="middle" fontSize="10" fill="#333">a</text>
          {/* Labels */}
          <text x="80" y="20" textAnchor="middle" fontSize="10" fill="#333">Wire 1</text>
          <text x="220" y="20" textAnchor="middle" fontSize="10" fill="#333">Wire 2</text>
        </svg>
      </div>

      <div className="theory">
        <h3>Theory</h3>
        <p><strong>Characteristic Impedance:</strong></p>
        <p>Z₀ = (120/√ε_r) · acosh(d/(2a)) ≈ (120/√ε_r) · ln(d/a) for d ≫ a</p>
        <p><strong>Per-unit-length parameters:</strong></p>
        <p>L = (μ/π) · acosh(d/(2a)) H/m</p>
        <p>C = π·ε / acosh(d/(2a)) F/m</p>
        <p><strong>Applications:</strong> TV antenna lead-in (300Ω), balanced audio cables, telephone lines, ladder lines.</p>
        <p><strong>Note:</strong> The formulas assume the wires are thin (a ≪ d) and in a uniform dielectric medium.</p>
      </div>
    </div>
  );
}

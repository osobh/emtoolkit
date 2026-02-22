import { useState, useMemo } from 'react';
import Plot from 'react-plotly.js';
import { wasm } from '../wasm';

interface StriplineResult {
  z0: number;
  epsilon_eff: number;
  phase_velocity: number;
  l_per_m: number;
  c_per_m: number;
}

export function StriplineModule() {
  const [width, setWidth] = useState(1.0); // mm
  const [groundSpacing, setGroundSpacing] = useState(2.0); // mm
  const [epsilonR, setEpsilonR] = useState(4.4);

  const result: StriplineResult | null = useMemo(() => {
    try {
      return wasm.stripline_params(
        width * 1e-3,
        groundSpacing * 1e-3,
        epsilonR
      ) as StriplineResult;
    } catch {
      return null;
    }
  }, [width, groundSpacing, epsilonR]);

  // Generate Z₀ vs width curve
  const z0VsWidth = useMemo(() => {
    const widths: number[] = [];
    const z0s: number[] = [];
    for (let w = 0.2; w <= 5.0; w += 0.1) {
      try {
        const r = wasm.stripline_params(
          w * 1e-3,
          groundSpacing * 1e-3,
          epsilonR
        ) as StriplineResult;
        widths.push(w);
        z0s.push(r.z0);
      } catch {
        // skip invalid points
      }
    }
    return { widths, z0s };
  }, [groundSpacing, epsilonR]);

  const ratio = width / groundSpacing;

  return (
    <div className="module">
      <h2>Stripline Calculator</h2>
      <p>
        Stripline is a conductor strip sandwiched between two parallel ground planes,
        providing TEM propagation with no dispersion.
      </p>

      <div className="controls">
        <label>
          Strip Width (mm):
          <input
            type="range"
            min={0.1}
            max={5.0}
            step={0.05}
            value={width}
            onChange={(e) => setWidth(+e.target.value)}
          />
          {width.toFixed(2)}
        </label>
        <label>
          Ground Spacing b (mm):
          <input
            type="range"
            min={0.5}
            max={10.0}
            step={0.1}
            value={groundSpacing}
            onChange={(e) => setGroundSpacing(+e.target.value)}
          />
          {groundSpacing.toFixed(1)}
        </label>
        <label>
          ε_r (substrate):
          <input
            type="range"
            min={1.0}
            max={12.0}
            step={0.1}
            value={epsilonR}
            onChange={(e) => setEpsilonR(+e.target.value)}
          />
          {epsilonR.toFixed(1)}
        </label>
      </div>

      {result && (
        <div className="results-grid">
          <div className="result-card">
            <span className="label">Z₀</span>
            <span className="value">{result.z0.toFixed(2)} Ω</span>
          </div>
          <div className="result-card">
            <span className="label">ε_eff</span>
            <span className="value">{result.epsilon_eff.toFixed(2)}</span>
          </div>
          <div className="result-card">
            <span className="label">Phase Velocity</span>
            <span className="value">{(result.phase_velocity / 1e8).toFixed(3)} × 10⁸ m/s</span>
          </div>
          <div className="result-card">
            <span className="label">w/b ratio</span>
            <span className="value">{ratio.toFixed(3)}</span>
          </div>
          <div className="result-card">
            <span className="label">L (per m)</span>
            <span className="value">{(result.l_per_m * 1e9).toFixed(2)} nH/m</span>
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
            x: z0VsWidth.widths,
            y: z0VsWidth.z0s,
            type: 'scatter',
            mode: 'lines',
            name: 'Z₀ vs Width',
            line: { color: '#2196F3', width: 2 },
          },
          result
            ? {
                x: [width],
                y: [result.z0],
                type: 'scatter',
                mode: 'markers',
                name: 'Current',
                marker: { size: 12, color: '#F44336' },
              }
            : null,
        ].filter(Boolean)}
        layout={{
          title: 'Characteristic Impedance vs Strip Width',
          xaxis: { title: 'Strip Width (mm)' },
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
        <svg viewBox="0 0 300 150" style={{ width: '100%', maxWidth: 400 }}>
          {/* Ground planes */}
          <rect x="30" y="20" width="240" height="8" fill="#555" />
          <rect x="30" y="122" width="240" height="8" fill="#555" />
          {/* Dielectric */}
          <rect x="30" y="28" width="240" height="94" fill="#FFECB3" stroke="#FF9800" strokeWidth="1" />
          {/* Strip conductor */}
          <rect
            x={150 - (width / groundSpacing) * 40}
            y="71"
            width={(width / groundSpacing) * 80}
            height="8"
            fill="#B8860B"
            stroke="#8B6914"
            strokeWidth="1"
          />
          {/* Labels */}
          <text x="150" y="12" textAnchor="middle" fontSize="10" fill="#333">Ground Plane</text>
          <text x="150" y="145" textAnchor="middle" fontSize="10" fill="#333">Ground Plane</text>
          <text x="150" y="65" textAnchor="middle" fontSize="10" fill="#333">w</text>
          {/* Dimension arrow for b */}
          <line x1="280" y1="28" x2="280" y2="122" stroke="#333" strokeWidth="1" />
          <line x1="275" y1="28" x2="285" y2="28" stroke="#333" strokeWidth="1" />
          <line x1="275" y1="122" x2="285" y2="122" stroke="#333" strokeWidth="1" />
          <text x="290" y="78" fontSize="10" fill="#333">b</text>
          {/* Epsilon label */}
          <text x="60" y="78" fontSize="12" fill="#333">ε_r</text>
        </svg>
      </div>

      <div className="theory">
        <h3>Theory</h3>
        <p><strong>Narrow strip (w/b &lt; 0.35):</strong></p>
        <p>Z₀ = (60/√ε_r) · ln(4b/(π·w·0.85))</p>
        <p><strong>Wide strip (w/b ≥ 0.35):</strong></p>
        <p>Z₀ = (η₀/√ε_r) / (w/b + (2/π)·ln(2))</p>
        <p><strong>Key advantages:</strong> TEM mode, no dispersion, well-shielded, ε_eff = ε_r exactly.</p>
        <p><strong>Typical applications:</strong> Microwave circuits, filters, couplers, high-isolation designs.</p>
      </div>
    </div>
  );
}

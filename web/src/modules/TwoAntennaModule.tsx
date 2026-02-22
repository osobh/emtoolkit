import { useState, useMemo } from 'react';
import Plot from 'react-plotly.js';
import * as wasm from '../lib/em_wasm';

interface TwoElementResult {
  angles_deg: number[];
  pattern_db: number[];
  element_pattern: number[];
  array_factor: number[];
  beamwidth_deg: number;
  directivity: number;
  main_beam_deg: number;
}

export function TwoAntennaModule() {
  const [elementLength, setElementLength] = useState(0.5); // wavelengths
  const [spacing, setSpacing] = useState(0.5); // wavelengths
  const [phaseShift, setPhaseShift] = useState(0); // degrees

  const result: TwoElementResult | null = useMemo(() => {
    try {
      return wasm.two_element_array(elementLength, spacing, phaseShift, 361) as TwoElementResult;
    } catch {
      return null;
    }
  }, [elementLength, spacing, phaseShift]);

  // Convert to polar plot format (theta in degrees, r is pattern)
  const polarData = useMemo(() => {
    if (!result) return null;

    // Normalize patterns for polar plot
    const maxElement = Math.max(...result.element_pattern.filter(v => isFinite(v)));
    const maxAF = Math.max(...result.array_factor.filter(v => isFinite(v)));

    return {
      angles: result.angles_deg,
      combined: result.pattern_db.map(v => Math.max(v + 60, 0)), // Shift to positive for polar
      element: result.element_pattern.map(v => (isFinite(v) ? (v / maxElement) * 60 : 0)),
      arrayFactor: result.array_factor.map(v => (isFinite(v) ? (v / maxAF) * 60 : 0)),
    };
  }, [result]);

  // Quick presets
  const presets = [
    { name: 'Broadside', spacing: 0.5, phase: 0 },
    { name: 'Endfire (0°)', spacing: 0.5, phase: -180 },
    { name: 'Endfire (180°)', spacing: 0.5, phase: 180 },
    { name: 'Scanned 45°', spacing: 0.5, phase: -127.3 },
    { name: 'Wide spacing', spacing: 1.0, phase: 0 },
  ];

  return (
    <div className="module">
      <h2>Two-Antenna Array</h2>
      <p>
        Analyze a two-element antenna array. The combined pattern is the product of the
        element pattern (dipole) and the array factor.
      </p>

      <div className="controls">
        <label>
          Element length (λ):
          <input
            type="range"
            min={0.1}
            max={1.5}
            step={0.05}
            value={elementLength}
            onChange={e => setElementLength(+e.target.value)}
          />
          {elementLength.toFixed(2)} λ
        </label>

        <label>
          Element spacing (λ):
          <input
            type="range"
            min={0.1}
            max={2.0}
            step={0.05}
            value={spacing}
            onChange={e => setSpacing(+e.target.value)}
          />
          {spacing.toFixed(2)} λ
        </label>

        <label>
          Phase shift β (°):
          <input
            type="range"
            min={-180}
            max={180}
            step={1}
            value={phaseShift}
            onChange={e => setPhaseShift(+e.target.value)}
          />
          {phaseShift}°
        </label>

        <div style={{ marginTop: 10 }}>
          <strong>Presets:</strong>{' '}
          {presets.map((p, i) => (
            <button
              key={i}
              onClick={() => {
                setSpacing(p.spacing);
                setPhaseShift(p.phase);
              }}
              style={{
                margin: '2px 4px',
                padding: '4px 8px',
                cursor: 'pointer',
              }}
            >
              {p.name}
            </button>
          ))}
        </div>
      </div>

      {result && (
        <>
          <div className="results-grid">
            <div className="result-card">
              <span className="label">Main beam</span>
              <span className="value">{result.main_beam_deg.toFixed(1)}°</span>
            </div>
            <div className="result-card">
              <span className="label">Beamwidth (3dB)</span>
              <span className="value">{result.beamwidth_deg.toFixed(1)}°</span>
            </div>
            <div className="result-card">
              <span className="label">Directivity (approx)</span>
              <span className="value">{result.directivity.toFixed(2)}</span>
            </div>
          </div>

          {/* Polar pattern plot */}
          {polarData && (
            <Plot
              data={[
                {
                  type: 'scatterpolar',
                  mode: 'lines',
                  r: polarData.combined,
                  theta: polarData.angles,
                  name: 'Combined (dB)',
                  line: { color: '#2196F3', width: 2 },
                },
                {
                  type: 'scatterpolar',
                  mode: 'lines',
                  r: polarData.element,
                  theta: polarData.angles,
                  name: 'Element',
                  line: { color: '#4CAF50', width: 1, dash: 'dash' },
                },
                {
                  type: 'scatterpolar',
                  mode: 'lines',
                  r: polarData.arrayFactor,
                  theta: polarData.angles,
                  name: 'Array Factor',
                  line: { color: '#FF9800', width: 1, dash: 'dot' },
                },
              ]}
              layout={{
                title: 'Radiation Pattern (E-plane)',
                polar: {
                  radialaxis: {
                    visible: true,
                    range: [0, 60],
                    ticksuffix: '',
                  },
                  angularaxis: {
                    direction: 'clockwise',
                    rotation: 90,
                  },
                },
                margin: { t: 50, r: 50, b: 50, l: 50 },
                height: 500,
                showlegend: true,
                legend: { x: 1.05, y: 0.5 },
              }}
              config={{ responsive: true }}
              style={{ width: '100%' }}
            />
          )}

          {/* Rectangular pattern plot */}
          <Plot
            data={[
              {
                x: result.angles_deg,
                y: result.pattern_db,
                type: 'scatter',
                mode: 'lines',
                name: 'Combined pattern',
                line: { color: '#2196F3', width: 2 },
              },
            ]}
            layout={{
              title: 'Pattern vs Angle (dB)',
              xaxis: { title: 'θ (degrees)', range: [0, 180] },
              yaxis: { title: 'Relative power (dB)', range: [-40, 5] },
              margin: { t: 40, r: 20, b: 50, l: 60 },
              height: 350,
            }}
            config={{ responsive: true }}
            style={{ width: '100%' }}
          />
        </>
      )}

      <div className="theory">
        <h3>Theory</h3>
        <p>
          <strong>Array factor (2 elements):</strong> AF(θ) = 2cos(ψ/2) where ψ = kd·cos(θ) + β
        </p>
        <p>
          <strong>Element pattern (thin dipole):</strong> E(θ) = [cos(kL/2·cosθ) - cos(kL/2)] /
          sin(θ)
        </p>
        <p>
          <strong>Combined pattern:</strong> E_total(θ) = E_element(θ) × AF(θ)
        </p>
        <p>
          <strong>Broadside:</strong> β = 0 → main beam at θ = 90°
        </p>
        <p>
          <strong>Endfire:</strong> β = ±kd → main beam at θ = 0° or 180°
        </p>
        <p>
          <strong>Steering:</strong> β = -kd·cos(θ₀) → main beam at θ = θ₀
        </p>
      </div>
    </div>
  );
}

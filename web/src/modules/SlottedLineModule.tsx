import { useState, useMemo } from 'react';
import Plot from 'react-plotly.js';
import { wasm } from '../wasm';

interface SlottedLineResult {
  vswr: number;
  gamma_magnitude: number;
  gamma_angle_deg: number;
  gamma_re: number;
  gamma_im: number;
  zl_re: number;
  zl_im: number;
  zl_norm_re: number;
  zl_norm_im: number;
}

interface StandingWavePattern {
  positions: number[];
  voltages: number[];
}

export function SlottedLineModule() {
  const [vMax, setVMax] = useState(2.0);
  const [vMin, setVMin] = useState(1.0);
  const [dMin, setDMin] = useState(0.075); // meters
  const [wavelength, setWavelength] = useState(0.3); // meters (1 GHz in free space)
  const [z0, setZ0] = useState(50.0);

  const result: SlottedLineResult | null = useMemo(() => {
    try {
      if (vMin <= 0 || vMax < vMin) return null;
      return wasm.slotted_line_measurement(vMax, vMin, dMin, wavelength, z0) as SlottedLineResult;
    } catch {
      return null;
    }
  }, [vMax, vMin, dMin, wavelength, z0]);

  const standingWave: StandingWavePattern | null = useMemo(() => {
    if (!result) return null;
    try {
      return wasm.slotted_line_standing_wave(
        result.gamma_magnitude,
        result.gamma_angle_deg * Math.PI / 180,
        2.0, // show 2 wavelengths
        500
      ) as StandingWavePattern;
    } catch {
      return null;
    }
  }, [result]);

  const frequency = 3e8 / wavelength / 1e9; // GHz

  return (
    <div className="module">
      <h2>Slotted Line Measurement</h2>
      <p>
        Determine load impedance from voltage standing wave measurements.
        The slotted line is a classic technique for measuring impedance before network analyzers.
      </p>

      <div className="controls">
        <label>
          V_max (normalized):
          <input
            type="range"
            min={1.0}
            max={10.0}
            step={0.1}
            value={vMax}
            onChange={(e) => setVMax(+e.target.value)}
          />
          {vMax.toFixed(1)}
        </label>
        <label>
          V_min (normalized):
          <input
            type="range"
            min={0.1}
            max={vMax - 0.1}
            step={0.1}
            value={vMin}
            onChange={(e) => setVMin(+e.target.value)}
          />
          {vMin.toFixed(1)}
        </label>
        <label>
          d_min (distance to first min, cm):
          <input
            type="range"
            min={0}
            max={wavelength * 50}
            step={0.1}
            value={dMin * 100}
            onChange={(e) => setDMin(+e.target.value / 100)}
          />
          {(dMin * 100).toFixed(1)}
        </label>
        <label>
          Wavelength λ (cm):
          <input
            type="range"
            min={1}
            max={100}
            step={1}
            value={wavelength * 100}
            onChange={(e) => setWavelength(+e.target.value / 100)}
          />
          {(wavelength * 100).toFixed(0)} ({frequency.toFixed(2)} GHz)
        </label>
        <label>
          Z₀ (Ω):
          <input
            type="range"
            min={25}
            max={300}
            step={5}
            value={z0}
            onChange={(e) => setZ0(+e.target.value)}
          />
          {z0.toFixed(0)}
        </label>
      </div>

      {result && (
        <div className="results-grid">
          <div className="result-card">
            <span className="label">VSWR</span>
            <span className="value">{result.vswr.toFixed(3)}</span>
          </div>
          <div className="result-card">
            <span className="label">|Γ|</span>
            <span className="value">{result.gamma_magnitude.toFixed(4)}</span>
          </div>
          <div className="result-card">
            <span className="label">∠Γ</span>
            <span className="value">{result.gamma_angle_deg.toFixed(1)}°</span>
          </div>
          <div className="result-card">
            <span className="label">Z_L</span>
            <span className="value">
              {result.zl_re.toFixed(1)} {result.zl_im >= 0 ? '+' : ''}{result.zl_im.toFixed(1)}j Ω
            </span>
          </div>
          <div className="result-card">
            <span className="label">z_L (normalized)</span>
            <span className="value">
              {result.zl_norm_re.toFixed(3)} {result.zl_norm_im >= 0 ? '+' : ''}{result.zl_norm_im.toFixed(3)}j
            </span>
          </div>
          <div className="result-card">
            <span className="label">Return Loss</span>
            <span className="value">
              {(-20 * Math.log10(result.gamma_magnitude)).toFixed(2)} dB
            </span>
          </div>
        </div>
      )}

      {standingWave && (
        <Plot
          data={[
            {
              x: standingWave.positions.map((p) => p * wavelength * 100),
              y: standingWave.voltages,
              type: 'scatter',
              mode: 'lines',
              name: '|V(z)|',
              line: { color: '#2196F3', width: 2 },
            },
            // Mark Vmax
            {
              x: standingWave.positions
                .filter((_, i) => {
                  const v = standingWave.voltages[i];
                  const prev = standingWave.voltages[i - 1] || 0;
                  const next = standingWave.voltages[i + 1] || 0;
                  return v > prev && v > next;
                })
                .map((p) => p * wavelength * 100),
              y: standingWave.voltages.filter((v, i) => {
                const prev = standingWave.voltages[i - 1] || 0;
                const next = standingWave.voltages[i + 1] || 0;
                return v > prev && v > next;
              }),
              type: 'scatter',
              mode: 'markers',
              name: 'V_max',
              marker: { size: 10, color: '#4CAF50', symbol: 'triangle-up' },
            },
            // Mark Vmin
            {
              x: standingWave.positions
                .filter((_, i) => {
                  const v = standingWave.voltages[i];
                  const prev = standingWave.voltages[i - 1] || Infinity;
                  const next = standingWave.voltages[i + 1] || Infinity;
                  return v < prev && v < next && i > 0 && i < standingWave.voltages.length - 1;
                })
                .map((p) => p * wavelength * 100),
              y: standingWave.voltages.filter((v, i) => {
                const prev = standingWave.voltages[i - 1] || Infinity;
                const next = standingWave.voltages[i + 1] || Infinity;
                return v < prev && v < next && i > 0 && i < standingWave.voltages.length - 1;
              }),
              type: 'scatter',
              mode: 'markers',
              name: 'V_min',
              marker: { size: 10, color: '#F44336', symbol: 'triangle-down' },
            },
            // Mark d_min position
            {
              x: [dMin * 100],
              y: [1 - result!.gamma_magnitude],
              type: 'scatter',
              mode: 'markers',
              name: 'd_min measurement',
              marker: { size: 14, color: '#FF9800', symbol: 'x' },
            },
          ]}
          layout={{
            title: 'Standing Wave Pattern',
            xaxis: { title: 'Distance from Load (cm)' },
            yaxis: { title: '|V(z)| (normalized)', range: [0, 2] },
            margin: { t: 40, r: 20, b: 50, l: 60 },
            height: 350,
            showlegend: true,
            legend: { x: 0.65, y: 0.95 },
            shapes: [
              // Add vertical line at d_min
              {
                type: 'line',
                x0: dMin * 100,
                x1: dMin * 100,
                y0: 0,
                y1: 2,
                line: { color: '#FF9800', width: 1, dash: 'dot' },
              },
            ],
          }}
          config={{ responsive: true }}
          style={{ width: '100%' }}
        />
      )}

      <div className="theory">
        <h3>Theory</h3>
        <p><strong>From measurements:</strong></p>
        <p>VSWR = V_max / V_min</p>
        <p>|Γ| = (VSWR - 1) / (VSWR + 1)</p>
        <p>∠Γ = π - 2βd_min, where β = 2π/λ</p>
        <p><strong>Load impedance:</strong></p>
        <p>Z_L = Z₀ · (1 + Γ) / (1 - Γ)</p>
        <p><strong>Physical interpretation:</strong></p>
        <p>
          At a voltage minimum, incident and reflected waves are 180° out of phase.
          The position of the first minimum from the load determines the phase of Γ.
        </p>
        <p><strong>Note:</strong> d_min = 0 indicates a short-circuit-like load (Γ phase ≈ 180°).</p>
      </div>
    </div>
  );
}

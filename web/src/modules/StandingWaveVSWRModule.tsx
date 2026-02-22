import { useState, useMemo } from 'react';
import Plot from 'react-plotly.js';
import * as wasm from '../lib/em_wasm';

interface StandingWaveData {
  positions: number[];
  voltage_mag: number[];
  current_mag: number[];
  vswr: number;
  gamma_mag: number;
  gamma_angle_deg: number;
  v_max: number;
  v_min: number;
  d_to_first_min: number;
  d_to_first_max: number;
}

type InputMode = 'vswr' | 'load';

export function StandingWaveVSWRModule() {
  const [inputMode, setInputMode] = useState<InputMode>('vswr');

  // Common parameters
  const [z0, setZ0] = useState(50);
  const [frequency, setFrequency] = useState(1e9);
  const [length, setLength] = useState(0.6);

  // VSWR mode parameters
  const [vswr, setVswr] = useState(2.0);
  const [gammaAngle, setGammaAngle] = useState(0);

  // Load mode parameters
  const [zlRe, setZlRe] = useState(100);
  const [zlIm, setZlIm] = useState(0);

  const data: StandingWaveData | null = useMemo(() => {
    try {
      if (inputMode === 'vswr') {
        return wasm.standing_wave_from_vswr(z0, vswr, gammaAngle, frequency, length, 500) as StandingWaveData;
      } else {
        return wasm.standing_wave_pattern(z0, zlRe, zlIm, frequency, length, 500) as StandingWaveData;
      }
    } catch {
      return null;
    }
  }, [inputMode, z0, vswr, gammaAngle, zlRe, zlIm, frequency, length]);

  // Calculate wavelength for reference
  const wavelength = 3e8 / frequency;

  return (
    <div className="module">
      <h2>Standing Wave Pattern (VSWR Mode)</h2>
      <p>
        Visualize voltage and current standing wave patterns. Choose to specify either
        VSWR + reflection coefficient angle, or load impedance.
      </p>

      <div className="controls">
        <label>
          Input mode:
          <select value={inputMode} onChange={e => setInputMode(e.target.value as InputMode)}>
            <option value="vswr">VSWR + Γ angle</option>
            <option value="load">Load impedance Z_L</option>
          </select>
        </label>

        <label>
          Z₀ (Ω):
          <input
            type="number"
            value={z0}
            onChange={e => setZ0(+e.target.value)}
            step={5}
            min={1}
          />
        </label>

        {inputMode === 'vswr' ? (
          <>
            <label>
              VSWR:
              <input
                type="range"
                min={1}
                max={10}
                step={0.1}
                value={vswr}
                onChange={e => setVswr(+e.target.value)}
              />
              {vswr.toFixed(2)}
            </label>

            <label>
              Γ angle (°):
              <input
                type="range"
                min={-180}
                max={180}
                step={1}
                value={gammaAngle}
                onChange={e => setGammaAngle(+e.target.value)}
              />
              {gammaAngle}°
            </label>
          </>
        ) : (
          <>
            <label>
              Z_L Real (Ω):
              <input
                type="number"
                value={zlRe}
                onChange={e => setZlRe(+e.target.value)}
                step={5}
              />
            </label>

            <label>
              Z_L Imag (Ω):
              <input
                type="number"
                value={zlIm}
                onChange={e => setZlIm(+e.target.value)}
                step={5}
              />
            </label>
          </>
        )}

        <label>
          Frequency:
          <input
            type="range"
            min={8}
            max={11}
            step={0.1}
            value={Math.log10(frequency)}
            onChange={e => setFrequency(Math.pow(10, +e.target.value))}
          />
          {frequency >= 1e9
            ? (frequency / 1e9).toFixed(2) + ' GHz'
            : (frequency / 1e6).toFixed(2) + ' MHz'}
        </label>

        <label>
          Line length:
          <input
            type="range"
            min={0.1}
            max={2}
            step={0.01}
            value={length}
            onChange={e => setLength(+e.target.value)}
          />
          {length.toFixed(2)} m ({(length / wavelength).toFixed(2)} λ)
        </label>
      </div>

      {data && (
        <>
          <div className="results-grid">
            <div className="result-card">
              <span className="label">VSWR</span>
              <span className="value">{data.vswr.toFixed(3)}</span>
            </div>
            <div className="result-card">
              <span className="label">|Γ|</span>
              <span className="value">{data.gamma_mag.toFixed(4)}</span>
            </div>
            <div className="result-card">
              <span className="label">∠Γ</span>
              <span className="value">{data.gamma_angle_deg.toFixed(1)}°</span>
            </div>
            <div className="result-card">
              <span className="label">V_max</span>
              <span className="value">{data.v_max.toFixed(3)}</span>
            </div>
            <div className="result-card">
              <span className="label">V_min</span>
              <span className="value">{data.v_min.toFixed(3)}</span>
            </div>
            <div className="result-card">
              <span className="label">Return loss</span>
              <span className="value">
                {data.gamma_mag > 0 ? (-20 * Math.log10(data.gamma_mag)).toFixed(1) : '∞'} dB
              </span>
            </div>
          </div>

          <div className="results-grid">
            <div className="result-card">
              <span className="label">d to V_min</span>
              <span className="value">
                {(data.d_to_first_min * 1e3).toFixed(1)} mm (
                {(data.d_to_first_min / wavelength).toFixed(3)} λ)
              </span>
            </div>
            <div className="result-card">
              <span className="label">d to V_max</span>
              <span className="value">
                {(data.d_to_first_max * 1e3).toFixed(1)} mm (
                {(data.d_to_first_max / wavelength).toFixed(3)} λ)
              </span>
            </div>
          </div>

          <Plot
            data={[
              {
                x: data.positions.map(d => d * 1e3),
                y: data.voltage_mag,
                type: 'scatter',
                mode: 'lines',
                line: { color: '#2196F3', width: 2 },
                name: '|V(d)|',
              },
              {
                x: data.positions.map(d => d * 1e3),
                y: data.current_mag,
                type: 'scatter',
                mode: 'lines',
                line: { color: '#4CAF50', width: 2 },
                name: '|I(d)|',
                yaxis: 'y2',
              },
            ]}
            layout={{
              title: 'Standing Wave Pattern',
              xaxis: { title: 'Distance from load (mm)' },
              yaxis: {
                title: '|V| (normalized)',
                titlefont: { color: '#2196F3' },
                tickfont: { color: '#2196F3' },
              },
              yaxis2: {
                title: '|I| (normalized)',
                titlefont: { color: '#4CAF50' },
                tickfont: { color: '#4CAF50' },
                overlaying: 'y',
                side: 'right',
              },
              margin: { t: 40, r: 60, b: 50, l: 60 },
              height: 400,
              legend: { x: 0.02, y: 0.98 },
            }}
            config={{ responsive: true }}
            style={{ width: '100%' }}
          />

          {/* Show where Vmax and Vmin occur */}
          <div style={{ marginTop: 20 }}>
            <h3>Key Positions</h3>
            <ul>
              <li>
                <strong>Voltage minimum</strong> at d = {(data.d_to_first_min * 1e3).toFixed(2)} mm
                from load
              </li>
              <li>
                <strong>Voltage maximum</strong> at d = {(data.d_to_first_max * 1e3).toFixed(2)} mm
                from load
              </li>
              <li>
                <strong>Wavelength:</strong> λ = {(wavelength * 1e3).toFixed(2)} mm
              </li>
              <li>
                <strong>Pattern period:</strong> λ/2 = {((wavelength / 2) * 1e3).toFixed(2)} mm
              </li>
            </ul>
          </div>
        </>
      )}

      <div className="theory">
        <h3>Theory</h3>
        <p>
          <strong>VSWR:</strong> S = (1 + |Γ|) / (1 - |Γ|)
        </p>
        <p>
          <strong>|Γ| from VSWR:</strong> |Γ| = (S - 1) / (S + 1)
        </p>
        <p>
          <strong>Voltage pattern:</strong> |V(d)| = |V⁺| · |1 + Γ_L · e^(-j2βd)|
        </p>
        <p>
          <strong>Current pattern:</strong> |I(d)| = |V⁺|/Z₀ · |1 - Γ_L · e^(-j2βd)|
        </p>
        <p>
          <strong>V_max/V_min:</strong> (1 + |Γ|) / (1 - |Γ|) = VSWR
        </p>
        <p>
          <strong>Position of V_min:</strong> d_min = (∠Γ - π) / (-2β)
        </p>
        <p>
          <strong>Position of V_max:</strong> d_max = ∠Γ / (2β)
        </p>
      </div>
    </div>
  );
}

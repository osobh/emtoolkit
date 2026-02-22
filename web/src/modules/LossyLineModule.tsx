import { useState, useMemo } from 'react';
import Plot from 'react-plotly.js';
import { wasm } from '../wasm';

interface LossyLineResult {
  alpha: number;
  beta: number;
  gamma_re: number;
  gamma_im: number;
  z0_re: number;
  z0_im: number;
  zin_re: number;
  zin_im: number;
  gamma_in_mag: number;
  gamma_in_phase_deg: number;
  power_loss_ratio: number;
  power_loss_db: number;
  phase_velocity: number;
  wavelength: number;
}

interface LossyLineProfile {
  positions: number[];
  voltages: number[];
  currents: number[];
}

export function LossyLineModule() {
  // Per-unit-length parameters
  const [r, setR] = useState(0.5); // Ω/m
  const [l, setL] = useState(250); // nH/m
  const [g, setG] = useState(0.01); // mS/m
  const [c, setC] = useState(100); // pF/m
  
  // Operating conditions
  const [frequency, setFrequency] = useState(1.0); // GHz
  const [length, setLength] = useState(1.0); // m
  const [zlRe, setZlRe] = useState(50.0); // Ω
  const [zlIm, setZlIm] = useState(0.0); // Ω

  const result: LossyLineResult | null = useMemo(() => {
    try {
      return wasm.lossy_line_analysis(
        r,
        l * 1e-9,
        g * 1e-3,
        c * 1e-12,
        frequency * 1e9,
        length,
        zlRe,
        zlIm
      ) as LossyLineResult;
    } catch {
      return null;
    }
  }, [r, l, g, c, frequency, length, zlRe, zlIm]);

  const profile: LossyLineProfile | null = useMemo(() => {
    if (!result) return null;
    try {
      return wasm.lossy_line_profile(
        result.z0_re,
        result.z0_im,
        result.gamma_re,
        result.gamma_im,
        zlRe,
        zlIm,
        length,
        200
      ) as LossyLineProfile;
    } catch {
      return null;
    }
  }, [result, zlRe, zlIm, length]);

  // Calculate loss per meter in dB
  const lossPerMeter = result ? 8.686 * result.alpha : 0; // α in Np/m → dB/m

  return (
    <div className="module">
      <h2>General Lossy Transmission Line</h2>
      <p>
        Analyze a transmission line with distributed R, L, G, C parameters.
        Computes propagation constant, characteristic impedance, and input impedance.
      </p>

      <div className="controls">
        <h4>Per-Unit-Length Parameters</h4>
        <label>
          R (Ω/m):
          <input
            type="range"
            min={0}
            max={10}
            step={0.1}
            value={r}
            onChange={(e) => setR(+e.target.value)}
          />
          {r.toFixed(1)}
        </label>
        <label>
          L (nH/m):
          <input
            type="range"
            min={50}
            max={1000}
            step={10}
            value={l}
            onChange={(e) => setL(+e.target.value)}
          />
          {l.toFixed(0)}
        </label>
        <label>
          G (mS/m):
          <input
            type="range"
            min={0}
            max={1}
            step={0.01}
            value={g}
            onChange={(e) => setG(+e.target.value)}
          />
          {g.toFixed(2)}
        </label>
        <label>
          C (pF/m):
          <input
            type="range"
            min={10}
            max={500}
            step={5}
            value={c}
            onChange={(e) => setC(+e.target.value)}
          />
          {c.toFixed(0)}
        </label>

        <h4>Operating Conditions</h4>
        <label>
          Frequency (GHz):
          <input
            type="range"
            min={0.1}
            max={10}
            step={0.1}
            value={frequency}
            onChange={(e) => setFrequency(+e.target.value)}
          />
          {frequency.toFixed(1)}
        </label>
        <label>
          Line Length (m):
          <input
            type="range"
            min={0.1}
            max={10}
            step={0.1}
            value={length}
            onChange={(e) => setLength(+e.target.value)}
          />
          {length.toFixed(1)}
        </label>

        <h4>Load Impedance</h4>
        <label>
          Z_L Real (Ω):
          <input
            type="range"
            min={0}
            max={200}
            step={5}
            value={zlRe}
            onChange={(e) => setZlRe(+e.target.value)}
          />
          {zlRe.toFixed(0)}
        </label>
        <label>
          Z_L Imag (Ω):
          <input
            type="range"
            min={-100}
            max={100}
            step={5}
            value={zlIm}
            onChange={(e) => setZlIm(+e.target.value)}
          />
          {zlIm.toFixed(0)}
        </label>
      </div>

      {result && (
        <>
          <div className="results-grid">
            <div className="result-card">
              <span className="label">α (attenuation)</span>
              <span className="value">{result.alpha.toFixed(4)} Np/m</span>
            </div>
            <div className="result-card">
              <span className="label">Loss per meter</span>
              <span className="value">{lossPerMeter.toFixed(3)} dB/m</span>
            </div>
            <div className="result-card">
              <span className="label">β (phase)</span>
              <span className="value">{result.beta.toFixed(3)} rad/m</span>
            </div>
            <div className="result-card">
              <span className="label">λ (wavelength)</span>
              <span className="value">{(result.wavelength * 100).toFixed(2)} cm</span>
            </div>
            <div className="result-card">
              <span className="label">Z₀</span>
              <span className="value">
                {result.z0_re.toFixed(2)} {result.z0_im >= 0 ? '+' : ''}{result.z0_im.toFixed(2)}j Ω
              </span>
            </div>
            <div className="result-card">
              <span className="label">Phase Velocity</span>
              <span className="value">{(result.phase_velocity / 1e8).toFixed(3)} × 10⁸ m/s</span>
            </div>
            <div className="result-card">
              <span className="label">Z_in</span>
              <span className="value">
                {result.zin_re.toFixed(2)} {result.zin_im >= 0 ? '+' : ''}{result.zin_im.toFixed(2)}j Ω
              </span>
            </div>
            <div className="result-card">
              <span className="label">|Γ_in|</span>
              <span className="value">{result.gamma_in_mag.toFixed(4)}</span>
            </div>
            <div className="result-card">
              <span className="label">Total Power Loss</span>
              <span className="value">{result.power_loss_db.toFixed(2)} dB</span>
            </div>
            <div className="result-card">
              <span className="label">Power Loss Ratio</span>
              <span className="value">{(result.power_loss_ratio * 100).toFixed(2)}%</span>
            </div>
          </div>
        </>
      )}

      {profile && (
        <Plot
          data={[
            {
              x: profile.positions.map((p) => p * 100),
              y: profile.voltages,
              type: 'scatter',
              mode: 'lines',
              name: '|V(z)|',
              line: { color: '#2196F3', width: 2 },
            },
            {
              x: profile.positions.map((p) => p * 100),
              y: profile.currents,
              type: 'scatter',
              mode: 'lines',
              name: '|I(z)|',
              line: { color: '#FF9800', width: 2 },
            },
          ]}
          layout={{
            title: 'Voltage and Current Profile Along Line',
            xaxis: { title: 'Distance from Load (cm)' },
            yaxis: { title: 'Normalized Magnitude' },
            margin: { t: 40, r: 20, b: 50, l: 60 },
            height: 350,
            showlegend: true,
            legend: { x: 0.7, y: 0.95 },
          }}
          config={{ responsive: true }}
          style={{ width: '100%' }}
        />
      )}

      <div className="theory">
        <h3>Theory</h3>
        <p><strong>Propagation constant:</strong></p>
        <p>γ = √((R + jωL)(G + jωC)) = α + jβ</p>
        <p><strong>Characteristic impedance:</strong></p>
        <p>Z₀ = √((R + jωL)/(G + jωC))</p>
        <p><strong>Input impedance:</strong></p>
        <p>Z_in = Z₀ · (Z_L + Z₀·tanh(γℓ)) / (Z₀ + Z_L·tanh(γℓ))</p>
        <p><strong>Loss mechanisms:</strong></p>
        <ul>
          <li><strong>R:</strong> Conductor loss (skin effect, DC resistance)</li>
          <li><strong>G:</strong> Dielectric loss (leakage, lossy dielectric)</li>
        </ul>
        <p><strong>Special cases:</strong></p>
        <ul>
          <li>Lossless line: R = G = 0 → α = 0, Z₀ is real</li>
          <li>Low-loss line: R ≪ ωL, G ≪ ωC → Z₀ ≈ √(L/C), α ≈ R/(2Z₀) + GZ₀/2</li>
          <li>Distortionless line: R/L = G/C → α = √(RG), Z₀ = √(L/C) (real)</li>
        </ul>
      </div>
    </div>
  );
}

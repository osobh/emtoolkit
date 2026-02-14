import { useState, useMemo } from 'react';
import Plot from 'react-plotly.js';

export function WaveEquationModule() {
  const [epsilonR, setEpsilonR] = useState(1.0);
  const [muR, setMuR] = useState(1.0);
  const [frequency, setFrequency] = useState(1e9);
  const [e0, setE0] = useState(1.0);
  const [time, setTime] = useState(0);

  const c0 = 2.99792458e8;
  const v = c0 / Math.sqrt(epsilonR * muR);
  const lambda = v / frequency;
  const k = 2 * Math.PI / lambda;
  const omega = 2 * Math.PI * frequency;
  const eta = 377.0 * Math.sqrt(muR / epsilonR);

  const waveData = useMemo(() => {
    const n = 500;
    const zMax = lambda * 3;
    const zs: number[] = [];
    const eField: number[] = [];
    const hField: number[] = [];
    const poynting: number[] = [];
    for (let i = 0; i < n; i++) {
      const z = zMax * i / (n - 1);
      zs.push(z);
      const eVal = e0 * Math.cos(omega * time * 1e-12 - k * z);
      const hVal = (e0 / eta) * Math.cos(omega * time * 1e-12 - k * z);
      eField.push(eVal);
      hField.push(hVal);
      poynting.push(eVal * hVal);
    }
    return { zs, eField, hField, poynting };
  }, [e0, omega, k, eta, lambda, time]);

  return (
    <div className="module">
      <h2>Wave Equation Visualizer</h2>
      <p>Visualize a uniform plane wave solution to Maxwell's equations in a simple medium.</p>

      <div className="controls">
        <label>εᵣ: <input type="range" min={1} max={20} step={0.1} value={epsilonR}
          onChange={e => setEpsilonR(+e.target.value)} /> {epsilonR.toFixed(1)}</label>
        <label>μᵣ: <input type="range" min={1} max={20} step={0.1} value={muR}
          onChange={e => setMuR(+e.target.value)} /> {muR.toFixed(1)}</label>
        <label>Frequency: <input type="range" min={6} max={11} step={0.1}
          value={Math.log10(frequency)} onChange={e => setFrequency(10 ** +e.target.value)} />
          {frequency >= 1e9 ? (frequency / 1e9).toFixed(1) + ' GHz' : (frequency / 1e6).toFixed(1) + ' MHz'}</label>
        <label>E₀ (V/m): <input type="range" min={0.1} max={10} step={0.1} value={e0}
          onChange={e => setE0(+e.target.value)} /> {e0.toFixed(1)}</label>
        <label>Time (ps): <input type="range" min={0} max={1000} step={1} value={time}
          onChange={e => setTime(+e.target.value)} /> {time}</label>
      </div>

      <div className="results-grid">
        <div className="result-card">
          <span className="label">Phase velocity</span>
          <span className="value">{(v / c0).toFixed(4)} c = {(v / 1e8).toFixed(3)} × 10⁸ m/s</span>
        </div>
        <div className="result-card">
          <span className="label">Wavelength</span>
          <span className="value">{lambda < 0.01
            ? (lambda * 1e3).toFixed(2) + ' mm'
            : lambda < 1
            ? (lambda * 100).toFixed(2) + ' cm'
            : lambda.toFixed(3) + ' m'}</span>
        </div>
        <div className="result-card">
          <span className="label">Intrinsic impedance η</span>
          <span className="value">{eta.toFixed(1)} Ω</span>
        </div>
        <div className="result-card">
          <span className="label">k (wavenumber)</span>
          <span className="value">{k.toFixed(2)} rad/m</span>
        </div>
        <div className="result-card">
          <span className="label">H₀</span>
          <span className="value">{(e0 / eta * 1e3).toFixed(3)} mA/m</span>
        </div>
        <div className="result-card">
          <span className="label">⟨S⟩ avg power</span>
          <span className="value">{(e0 * e0 / (2 * eta) * 1e3).toFixed(3)} mW/m²</span>
        </div>
      </div>

      <Plot
        data={[
          { x: waveData.zs.map(z => z / lambda), y: waveData.eField,
            type: 'scatter', mode: 'lines', name: 'E_x (V/m)',
            line: { color: '#F44336', width: 2 } },
          { x: waveData.zs.map(z => z / lambda), y: waveData.hField.map(h => h * eta),
            type: 'scatter', mode: 'lines', name: 'η·H_y (V/m)',
            line: { color: '#2196F3', width: 2, dash: 'dash' } },
        ]}
        layout={{
          title: 'E and H Fields (snapshot at t = ' + time + ' ps)',
          xaxis: { title: 'z / λ' },
          yaxis: { title: 'Field amplitude' },
          margin: { t: 40, r: 20, b: 50, l: 60 }, height: 350,
          legend: { x: 0.7, y: 0.98 },
        }}
        config={{ responsive: true }} style={{ width: '100%' }}
      />

      <div className="theory">
        <h3>Theory</h3>
        <p><strong>Wave equation:</strong> ∇²E = με ∂²E/∂t²</p>
        <p><strong>Solution:</strong> E = x̂ E₀ cos(ωt − kz), H = ŷ (E₀/η) cos(ωt − kz)</p>
        <p><strong>Phase velocity:</strong> v_p = 1/√(με) = c/√(ε_r μ_r)</p>
        <p><strong>Intrinsic impedance:</strong> η = √(μ/ε) = η₀√(μ_r/ε_r)</p>
      </div>
    </div>
  );
}

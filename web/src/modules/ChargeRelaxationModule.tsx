import { useState, useMemo } from 'react';
import Plot from 'react-plotly.js';
import { wasm } from '../wasm';

export function ChargeRelaxationModule() {
  const [rho0, setRho0] = useState(1e-6);
  const [epsilonR, setEpsilonR] = useState(4.0);
  const [conductivity, setConductivity] = useState(1e-3);
  const [radius, setRadius] = useState(0.01);

  const tau = epsilonR * 8.854e-12 / conductivity;
  const tEnd = tau * 5;

  const data = useMemo(
    () => wasm.charge_relaxation(rho0, epsilonR, conductivity, radius, tEnd, 500),
    [rho0, epsilonR, conductivity, radius, tEnd],
  );

  return (
    <div className="module-panel">
      <h2>Charge Relaxation & Continuity Equation</h2>
      <div className="controls">
        <div className="control-group"><label>ρ₀ (μC/m³)</label><input type="number" value={rho0 * 1e6} onChange={e => setRho0(+e.target.value * 1e-6)} step={0.1} min={0.01} /></div>
        <div className="control-group"><label>εᵣ</label><input type="number" value={epsilonR} onChange={e => setEpsilonR(+e.target.value)} step={0.5} min={1} /></div>
        <div className="control-group"><label>σ (S/m)</label><input type="number" value={conductivity} onChange={e => setConductivity(+e.target.value)} step={1e-4} min={1e-6} /></div>
        <div className="control-group"><label>Radius (cm)</label><input type="number" value={radius * 100} onChange={e => setRadius(+e.target.value / 100)} step={0.5} min={0.1} /></div>
      </div>

      <Plot
        data={[{
          x: data.times.map((t: number) => t * 1000),
          y: data.rho.map((r: number) => r * 1e6),
          mode: 'lines', line: { color: '#e67e22', width: 2 }, name: 'ρ(t)',
        }]}
        layout={{
          width: 800, height: 400,
          xaxis: { title: { text: 'Time (ms)' } },
          yaxis: { title: { text: 'ρ (μC/m³)' } },
          margin: { t: 20, b: 50, l: 70, r: 20 },
          paper_bgcolor: 'transparent', plot_bgcolor: 'white',
        }}
        config={{ responsive: true }}
      />

      <div className="result-box">
        <div className="result-grid">
          <div className="result-item"><span className="result-label">Relaxation time τ</span><span className="result-value">{(tau * 1000).toExponential(3)} ms</span></div>
          <div className="result-item"><span className="result-label">τ = ε/σ</span><span className="result-value">{tau.toExponential(3)} s</span></div>
        </div>
        <p style={{ fontSize: '0.85rem', color: '#666', marginTop: 12 }}>
          Free charge placed inside a conductor decays exponentially: ρ(t) = ρ₀ e^(−t/τ).
          After 5τ ≈ {(tEnd * 1000).toFixed(3)} ms, the charge has migrated to the surface.
        </p>
      </div>
    </div>
  );
}

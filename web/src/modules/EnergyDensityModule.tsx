import { useState, useMemo } from 'react';
import Plot from 'react-plotly.js';

export function EnergyDensityModule() {
  const [eField, setEField] = useState(1000);
  const [bField, setBField] = useState(0.01);
  const [epsilonR, setEpsilonR] = useState(1.0);
  const [muR, setMuR] = useState(1.0);

  const eps0 = 8.854187817e-12;
  const mu0 = 4e-7 * Math.PI;

  const results = useMemo(() => {
    const uE = 0.5 * eps0 * epsilonR * eField * eField;
    const uB = 0.5 * bField * bField / (mu0 * muR);
    const uTotal = uE + uB;
    const dMag = eps0 * epsilonR * eField;
    const hMag = bField / (mu0 * muR);
    return { uE, uB, uTotal, dMag, hMag };
  }, [eField, bField, epsilonR, muR]);

  const eSweep = useMemo(() => {
    const es: number[] = [];
    const ues: number[] = [];
    for (let i = 0; i < 200; i++) {
      const e = (i + 1) * 50;
      es.push(e);
      ues.push(0.5 * eps0 * epsilonR * e * e);
    }
    return { es, ues };
  }, [epsilonR]);

  return (
    <div className="module">
      <h2>Energy Density</h2>
      <p>Electric and magnetic energy density in electromagnetic fields.</p>

      <div className="controls">
        <label>E (V/m): <input type="range" min={0} max={10000} step={10} value={eField}
          onChange={e => setEField(+e.target.value)} /> {eField}</label>
        <label>B (mT): <input type="range" min={0} max={100} step={0.1}
          value={bField * 1000} onChange={e => setBField(+e.target.value / 1000)} />
          {(bField * 1000).toFixed(1)}</label>
        <label>εᵣ: <input type="range" min={1} max={100} step={0.5} value={epsilonR}
          onChange={e => setEpsilonR(+e.target.value)} /> {epsilonR}</label>
        <label>μᵣ: <input type="range" min={1} max={1000} step={1} value={muR}
          onChange={e => setMuR(+e.target.value)} /> {muR}</label>
      </div>

      <div className="results-grid">
        <div className="result-card">
          <span className="label">u_E (electric)</span>
          <span className="value">{results.uE.toExponential(3)} J/m³</span>
        </div>
        <div className="result-card">
          <span className="label">u_B (magnetic)</span>
          <span className="value">{results.uB.toExponential(3)} J/m³</span>
        </div>
        <div className="result-card">
          <span className="label">u_total</span>
          <span className="value">{results.uTotal.toExponential(3)} J/m³</span>
        </div>
        <div className="result-card">
          <span className="label">|D|</span>
          <span className="value">{results.dMag.toExponential(3)} C/m²</span>
        </div>
        <div className="result-card">
          <span className="label">|H|</span>
          <span className="value">{results.hMag.toExponential(3)} A/m</span>
        </div>
        <div className="result-card">
          <span className="label">u_E / u_total</span>
          <span className="value">{results.uTotal > 0 ? (results.uE / results.uTotal * 100).toFixed(1) : 0}%</span>
        </div>
      </div>

      <Plot
        data={[{
          x: eSweep.es,
          y: eSweep.ues.map(u => u * 1e6),
          type: 'scatter', mode: 'lines', name: 'u_E(E)',
          line: { color: '#2196F3', width: 2 },
          fill: 'tozeroy', fillcolor: 'rgba(33,150,243,0.15)',
        }]}
        layout={{
          title: 'Electric Energy Density vs E-field',
          xaxis: { title: 'E (V/m)' },
          yaxis: { title: 'u_E (μJ/m³)' },
          margin: { t: 40, r: 20, b: 50, l: 60 }, height: 350,
        }}
        config={{ responsive: true }} style={{ width: '100%' }}
      />

      <div className="theory">
        <h3>Theory</h3>
        <p><strong>Electric:</strong> u_E = ½ ε|E|² = ½ D·E</p>
        <p><strong>Magnetic:</strong> u_B = ½ |B|²/μ = ½ B·H</p>
        <p><strong>Total:</strong> u = u_E + u_B</p>
        <p>For a plane wave in lossless media: u_E = u_B (equipartition)</p>
      </div>
    </div>
  );
}

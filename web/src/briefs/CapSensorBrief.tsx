import { useState, useMemo } from 'react';
import Plot from 'react-plotly.js';

export function CapSensorBrief() {
  const [area, setArea] = useState(1.0);
  const [gap, setGap] = useState(1.0);
  const [epsilonR, setEpsilonR] = useState(1.0);

  const eps0 = 8.854e-12;
  const capPf = eps0 * epsilonR * (area * 1e-4) / (gap * 1e-3) * 1e12;

  const gapSweep = useMemo(() => {
    const gs: number[] = [];
    const cs: number[] = [];
    for (let g = 0.1; g <= 5; g += 0.05) {
      gs.push(g);
      cs.push(eps0 * epsilonR * (area * 1e-4) / (g * 1e-3) * 1e12);
    }
    return { gs, cs };
  }, [area, epsilonR]);

  return (
    <div className="module">
      <h2>TB9: Capacitive Sensors</h2>
      <p>Capacitive sensors detect changes in capacitance caused by proximity, displacement, or material properties — the principle behind touchscreens.</p>

      <div className="controls">
        <label>Plate area (cm²): <input type="range" min={0.1} max={10} step={0.1} value={area}
          onChange={e => setArea(+e.target.value)} /> {area.toFixed(1)}</label>
        <label>Gap (mm): <input type="range" min={0.1} max={5} step={0.1} value={gap}
          onChange={e => setGap(+e.target.value)} /> {gap.toFixed(1)}</label>
        <label>εᵣ (dielectric): <input type="range" min={1} max={80} step={0.5} value={epsilonR}
          onChange={e => setEpsilonR(+e.target.value)} /> {epsilonR.toFixed(1)}</label>
      </div>

      <div className="results-grid">
        <div className="result-card"><span className="label">Capacitance</span><span className="value">{capPf.toFixed(2)} pF</span></div>
        <div className="result-card"><span className="label">Sensitivity ΔC/Δd</span><span className="value">{(capPf / gap).toFixed(2)} pF/mm</span></div>
        <div className="result-card"><span className="label">E-field (at 5V)</span><span className="value">{(5 / (gap * 1e-3)).toFixed(0)} V/m</span></div>
      </div>

      <Plot
        data={[{
          x: gapSweep.gs, y: gapSweep.cs,
          type: 'scatter', mode: 'lines', name: 'C vs gap',
          line: { color: '#2196F3', width: 2 },
        }]}
        layout={{
          title: 'Capacitance vs Gap Distance (1/d relationship)',
          xaxis: { title: 'Gap (mm)' },
          yaxis: { title: 'Capacitance (pF)' },
          margin: { t: 40, r: 20, b: 50, l: 60 }, height: 300,
        }}
        config={{ responsive: true }} style={{ width: '100%' }}
      />

      <div style={{ margin: '20px 0', padding: 16, background: '#E3F2FD', borderRadius: 8 }}>
        <h4>Touchscreen Technologies</h4>
        <p style={{ fontSize: 14 }}><strong>Mutual capacitance:</strong> Grid of TX/RX electrodes. Finger absorbs charge at intersection → detects (x,y). Supports multi-touch.</p>
        <p style={{ fontSize: 14 }}><strong>Self-capacitance:</strong> Each electrode measures its own capacitance to ground. Finger adds body capacitance (~100 pF). Simpler but no true multi-touch.</p>
        <p style={{ fontSize: 14 }}><strong>Why fingers work:</strong> Human body is conductive (εᵣ ≈ 80 for tissue) and grounded through shoes/floor. A finger near an electrode changes the local electric field distribution.</p>
      </div>

      <div className="theory">
        <h3>Sensing Modes</h3>
        <p><strong>Gap change:</strong> C = εA/d — proximity sensors, accelerometers (MEMS)</p>
        <p><strong>Area change:</strong> C ∝ A — rotary encoders, liquid level sensors</p>
        <p><strong>Dielectric change:</strong> C ∝ εᵣ — moisture sensors, material identification</p>
      </div>
    </div>
  );
}

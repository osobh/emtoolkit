import { useState, useMemo } from 'react';
import Plot from 'react-plotly.js';

export function SupercapBrief() {
  const [capacitance, setCapacitance] = useState(100);
  const [voltage, setVoltage] = useState(2.7);
  const [esr, setEsr] = useState(0.01);
  const [loadR, setLoadR] = useState(10);

  const energy = 0.5 * capacitance * voltage * voltage;
  const tau = capacitance * (esr + loadR);
  const peakCurrent = voltage / (esr + loadR);
  const peakPower = voltage * voltage / (4 * esr);

  const discharge = useMemo(() => {
    const tEnd = tau * 5;
    const n = 300;
    const ts: number[] = [];
    const vc: number[] = [];
    const ic: number[] = [];
    const pc: number[] = [];
    for (let i = 0; i < n; i++) {
      const t = tEnd * i / (n - 1);
      ts.push(t);
      const v = voltage * Math.exp(-t / tau);
      vc.push(v);
      ic.push(v / (esr + loadR));
      pc.push(v * v / (esr + loadR));
    }
    return { ts, vc, ic, pc };
  }, [capacitance, voltage, esr, loadR, tau]);

  const comparisons = [
    { name: 'Li-ion battery', energy: 250, power: 1, label: 'Wh/kg vs kW/kg' },
    { name: 'Supercapacitor', energy: 5, power: 10, label: '' },
    { name: 'Electrolytic cap', energy: 0.01, power: 100, label: '' },
  ];

  return (
    <div className="module">
      <h2>TB8: Supercapacitors as Batteries</h2>
      <p>Supercapacitors (EDLCs) store energy electrostatically in the electric double layer — bridging the gap between batteries and capacitors.</p>

      <div className="controls">
        <label>Capacitance (F): <input type="range" min={1} max={3000} step={10} value={capacitance}
          onChange={e => setCapacitance(+e.target.value)} /> {capacitance}</label>
        <label>Voltage (V): <input type="range" min={1} max={5.5} step={0.1} value={voltage}
          onChange={e => setVoltage(+e.target.value)} /> {voltage.toFixed(1)}</label>
        <label>ESR (Ω): <input type="range" min={0.001} max={0.1} step={0.001} value={esr}
          onChange={e => setEsr(+e.target.value)} /> {esr.toFixed(3)}</label>
        <label>Load R (Ω): <input type="range" min={0.1} max={100} step={0.1} value={loadR}
          onChange={e => setLoadR(+e.target.value)} /> {loadR}</label>
      </div>

      <div className="results-grid">
        <div className="result-card"><span className="label">Stored energy</span><span className="value">{energy >= 3600 ? (energy / 3600).toFixed(2) + ' Wh' : energy.toFixed(1) + ' J'}</span></div>
        <div className="result-card"><span className="label">Time constant τ</span><span className="value">{tau >= 60 ? (tau / 60).toFixed(1) + ' min' : tau.toFixed(1) + ' s'}</span></div>
        <div className="result-card"><span className="label">Peak current</span><span className="value">{peakCurrent.toFixed(2)} A</span></div>
        <div className="result-card"><span className="label">Peak power (matched)</span><span className="value">{peakPower >= 1000 ? (peakPower / 1000).toFixed(1) + ' kW' : peakPower.toFixed(1) + ' W'}</span></div>
      </div>

      <Plot
        data={[
          { x: discharge.ts, y: discharge.vc, type: 'scatter', mode: 'lines',
            name: 'Voltage', line: { color: '#2196F3', width: 2 } },
          { x: discharge.ts, y: discharge.pc, type: 'scatter', mode: 'lines',
            name: 'Power (W)', line: { color: '#F44336', width: 2, dash: 'dash' }, yaxis: 'y2' },
        ]}
        layout={{
          title: 'Discharge Through Load',
          xaxis: { title: 'Time (s)' },
          yaxis: { title: 'Voltage (V)' },
          yaxis2: { title: 'Power (W)', overlaying: 'y', side: 'right' },
          margin: { t: 40, r: 60, b: 50, l: 60 }, height: 350,
        }}
        config={{ responsive: true }} style={{ width: '100%' }}
      />

      <div className="theory">
        <h3>How Supercapacitors Work</h3>
        <p><strong>Electric Double Layer:</strong> At each electrode-electrolyte interface, ions form a ~1 nm thick layer (Helmholtz layer). This nanoscale gap + huge surface area (activated carbon: ~2000 m²/g) = enormous capacitance.</p>
        <p><strong>C = εA/d:</strong> With d ≈ 1 nm and A ≈ 2000 m²/g → Farads per gram!</p>
        <p><strong>vs Batteries:</strong> 10-100× more power density, 10-100× less energy density, 1M+ charge cycles vs ~500-1000 for Li-ion.</p>
        <p><strong>Applications:</strong> Regenerative braking, UPS, peak power assist, energy harvesting buffer.</p>
      </div>
    </div>
  );
}

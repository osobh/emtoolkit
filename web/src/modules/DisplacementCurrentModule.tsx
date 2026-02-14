import { useState, useMemo } from 'react';
import Plot from 'react-plotly.js';
import { wasm } from '../wasm';

export function DisplacementCurrentModule() {
  const [area, setArea] = useState(0.01);
  const [separation, setSeparation] = useState(0.001);
  const [epsilonR, setEpsilonR] = useState(1.0);
  const [vPeak, setVPeak] = useState(100);
  const [omega, setOmega] = useState(2 * Math.PI * 60);

  const tEnd = (2 * Math.PI * 2) / omega;
  const data = useMemo(
    () => wasm.displacement_current_sim(area, separation, epsilonR, vPeak, omega, tEnd, 500),
    [area, separation, epsilonR, vPeak, omega, tEnd],
  );

  return (
    <div className="module-panel">
      <h2>Displacement Current (Parallel-Plate Capacitor)</h2>
      <div className="controls">
        <div className="control-group"><label>Plate Area (m²)</label><input type="number" value={area} onChange={e => setArea(+e.target.value)} step={0.001} min={0.001} /></div>
        <div className="control-group"><label>Separation (mm)</label><input type="number" value={separation * 1000} onChange={e => setSeparation(+e.target.value / 1000)} step={0.1} min={0.1} /></div>
        <div className="control-group"><label>εᵣ</label><input type="number" value={epsilonR} onChange={e => setEpsilonR(+e.target.value)} step={0.5} min={1} /></div>
        <div className="control-group"><label>V_peak (V)</label><input type="number" value={vPeak} onChange={e => setVPeak(+e.target.value)} step={10} min={1} /></div>
        <div className="control-group"><label>Freq (Hz)</label><input type="number" value={omega / (2 * Math.PI)} onChange={e => setOmega(+e.target.value * 2 * Math.PI)} step={10} min={1} /></div>
      </div>
      <Plot
        data={[
          { x: data.times, y: data.voltage, mode: 'lines', line: { color: '#2196f3', width: 2 }, name: 'V(t)', yaxis: 'y' },
          { x: data.times, y: data.j_d, mode: 'lines', line: { color: '#e63946', width: 2 }, name: 'J_d(t)', yaxis: 'y2' },
        ]}
        layout={{
          width: 800, height: 400,
          xaxis: { title: { text: 'Time (s)' } },
          yaxis: { title: { text: 'Voltage (V)' }, side: 'left' },
          yaxis2: { title: { text: 'J_d (A/m²)' }, side: 'right', overlaying: 'y' },
          margin: { t: 20, b: 50, l: 60, r: 60 },
          legend: { x: 0.02, y: 0.98 },
          paper_bgcolor: 'transparent', plot_bgcolor: 'white',
        }}
        config={{ responsive: true }}
      />
    </div>
  );
}

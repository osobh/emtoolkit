import { useState, useMemo } from 'react';
import Plot from 'react-plotly.js';
import { wasm } from '../wasm';

export function DipoleModule() {
  const [frequency, setFrequency] = useState(1e9);
  const [current, setCurrent] = useState(1.0);
  const [antennaType, setAntennaType] = useState<'hertzian' | 'halfwave'>('halfwave');

  const data = useMemo(() => {
    if (antennaType === 'halfwave') {
      return wasm.half_wave_dipole(frequency, current, 361);
    }
    const lambda = 3e8 / frequency;
    return wasm.hertzian_dipole(lambda * 0.01, current, frequency, 361);
  }, [frequency, current, antennaType]);

  // Convert to polar plot data
  const polarR = data.pattern;
  const polarTheta = data.thetas_deg;

  return (
    <div className="module-panel">
      <h2>Dipole Antenna Radiation Pattern</h2>
      <div className="controls">
        <div className="control-group">
          <label>Type</label>
          <select value={antennaType} onChange={e => setAntennaType(e.target.value as 'hertzian' | 'halfwave')}>
            <option value="halfwave">Half-Wave Dipole</option>
            <option value="hertzian">Hertzian Dipole</option>
          </select>
        </div>
        <div className="control-group">
          <label>Frequency (GHz)</label>
          <input type="number" value={frequency / 1e9} onChange={e => setFrequency(+e.target.value * 1e9)} step={0.1} min={0.1} />
        </div>
        <div className="control-group">
          <label>Current (A)</label>
          <input type="number" value={current} onChange={e => setCurrent(+e.target.value)} step={0.1} min={0.01} />
        </div>
      </div>

      <div style={{ display: 'flex', gap: 20, flexWrap: 'wrap' }}>
        <Plot
          data={[{
            type: 'scatterpolar' as const,
            r: polarR,
            theta: polarTheta,
            mode: 'lines',
            line: { color: '#e63946', width: 2 },
            name: 'E-plane',
          }]}
          layout={{
            width: 500, height: 500,
            polar: {
              radialaxis: { range: [0, 1.1] },
              angularaxis: { direction: 'clockwise' },
            },
            margin: { t: 30, b: 30, l: 30, r: 30 },
            paper_bgcolor: 'transparent',
            showlegend: false,
          }}
          config={{ responsive: true }}
        />

        <div className="result-box" style={{ flex: 1, minWidth: 250 }}>
          <h3 style={{ marginTop: 0 }}>Antenna Parameters</h3>
          <div className="result-grid">
            <div className="result-item"><span className="result-label">R_rad</span><span className="result-value">{data.radiation_resistance.toFixed(2)} Ω</span></div>
            <div className="result-item"><span className="result-label">Directivity</span><span className="result-value">{data.directivity.toFixed(3)}</span></div>
            <div className="result-item"><span className="result-label">Directivity</span><span className="result-value">{data.directivity_dbi.toFixed(2)} dBi</span></div>
            <div className="result-item"><span className="result-label">P_rad</span><span className="result-value">{data.radiated_power.toExponential(3)} W</span></div>
            <div className="result-item"><span className="result-label">A_eff</span><span className="result-value">{data.effective_area.toExponential(3)} m²</span></div>
          </div>
        </div>
      </div>
    </div>
  );
}

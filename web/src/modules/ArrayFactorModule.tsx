import { useState, useMemo } from 'react';
import Plot from 'react-plotly.js';
import { wasm } from '../wasm';

export function ArrayFactorModule() {
  const [numElements, setNumElements] = useState(8);
  const [spacing, setSpacing] = useState(0.5);
  const [betaDeg, setBetaDeg] = useState(0);

  const data = useMemo(
    () => wasm.antenna_array(numElements, spacing, betaDeg, 361),
    [numElements, spacing, betaDeg],
  );

  return (
    <div className="module-panel">
      <h2>Antenna Array Factor</h2>
      <div className="controls">
        <div className="control-group"><label>Elements</label><input type="number" value={numElements} onChange={e => setNumElements(+e.target.value)} step={1} min={2} max={32} /></div>
        <div className="control-group"><label>Spacing (d/λ)</label><input type="number" value={spacing} onChange={e => setSpacing(+e.target.value)} step={0.05} min={0.1} /></div>
        <div className="control-group">
          <label>Progressive Phase β (°)</label>
          <input type="range" min={-180} max={180} step={1} value={betaDeg}
            onChange={e => setBetaDeg(+e.target.value)} />
          <span>{betaDeg}°</span>
        </div>
      </div>

      <div style={{ display: 'flex', gap: 20, flexWrap: 'wrap' }}>
        <Plot
          data={[{
            type: 'scatterpolar' as const,
            r: data.pattern, theta: data.thetas_deg, mode: 'lines',
            line: { color: '#9b59b6', width: 2 },
          }]}
          layout={{
            width: 500, height: 500,
            polar: { radialaxis: { range: [0, 1.1] }, angularaxis: { direction: 'clockwise' } },
            margin: { t: 30, b: 30, l: 30, r: 30 },
            showlegend: false,
            paper_bgcolor: 'transparent',
          }}
          config={{ responsive: true }}
        />
        <div className="result-box" style={{ flex: 1, minWidth: 250 }}>
          <h3 style={{ marginTop: 0 }}>Array Properties</h3>
          <div className="result-grid">
            <div className="result-item"><span className="result-label">Directivity</span><span className="result-value">{data.directivity_dbi.toFixed(2)} dBi</span></div>
            <div className="result-item"><span className="result-label">HPBW</span><span className="result-value">{data.hpbw_deg.toFixed(1)}°</span></div>
            <div className="result-item"><span className="result-label">Scan angle</span><span className="result-value">{data.scan_angle_deg.toFixed(1)}°</span></div>
          </div>
        </div>
      </div>
    </div>
  );
}

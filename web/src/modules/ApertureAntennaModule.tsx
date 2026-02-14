import { useState, useMemo } from 'react';
import Plot from 'react-plotly.js';

export function ApertureAntennaModule() {
  const [width, setWidth] = useState(0.3);
  const [height, setHeight] = useState(0.2);
  const [frequency, setFrequency] = useState(10e9);
  const [efficiency, setEfficiency] = useState(0.65);

  const c = 2.99792458e8;
  const lambda = c / frequency;
  const area = width * height;
  const directivity = 4 * Math.PI * area / (lambda * lambda);
  const gain = efficiency * directivity;
  const gainDb = 10 * Math.log10(gain);
  const directivityDb = 10 * Math.log10(directivity);
  const beamwidthH = (lambda / width) * 180 / Math.PI;
  const beamwidthV = (lambda / height) * 180 / Math.PI;
  const aEff = efficiency * area;

  const pattern = useMemo(() => {
    const n = 361;
    const angles: number[] = [];
    const hPlane: number[] = [];
    const ePlane: number[] = [];
    for (let i = 0; i < n; i++) {
      const theta = (i - 180) * Math.PI / 180;
      angles.push((i - 180));
      const argH = Math.PI * width * Math.sin(theta) / lambda;
      const argE = Math.PI * height * Math.sin(theta) / lambda;
      const fH = Math.abs(argH) < 1e-10 ? 1.0 : Math.sin(argH) / argH;
      const fE = Math.abs(argE) < 1e-10 ? 1.0 : Math.sin(argE) / argE;
      hPlane.push(20 * Math.log10(Math.abs(fH)));
      ePlane.push(20 * Math.log10(Math.abs(fE)));
    }
    return { angles, hPlane, ePlane };
  }, [width, height, lambda]);

  return (
    <div className="module">
      <h2>Aperture Antenna</h2>
      <p>Rectangular aperture (horn, parabolic reflector feed) — directivity, gain, and radiation pattern.</p>

      <div className="controls">
        <label>Width a (cm): <input type="range" min={1} max={100} step={0.5}
          value={width * 100} onChange={e => setWidth(+e.target.value / 100)} />
          {(width * 100).toFixed(1)}</label>
        <label>Height b (cm): <input type="range" min={1} max={100} step={0.5}
          value={height * 100} onChange={e => setHeight(+e.target.value / 100)} />
          {(height * 100).toFixed(1)}</label>
        <label>Frequency: <input type="range" min={1e9} max={100e9} step={1e8} value={frequency}
          onChange={e => setFrequency(+e.target.value)} />
          {(frequency / 1e9).toFixed(1)} GHz</label>
        <label>Aperture efficiency: <input type="range" min={0.1} max={1.0} step={0.01} value={efficiency}
          onChange={e => setEfficiency(+e.target.value)} /> {(efficiency * 100).toFixed(0)}%</label>
      </div>

      <div className="results-grid">
        <div className="result-card">
          <span className="label">Directivity</span>
          <span className="value">{directivityDb.toFixed(1)} dBi</span>
        </div>
        <div className="result-card">
          <span className="label">Gain</span>
          <span className="value">{gainDb.toFixed(1)} dBi</span>
        </div>
        <div className="result-card">
          <span className="label">Beamwidth (H-plane)</span>
          <span className="value">{beamwidthH.toFixed(1)}°</span>
        </div>
        <div className="result-card">
          <span className="label">Beamwidth (E-plane)</span>
          <span className="value">{beamwidthV.toFixed(1)}°</span>
        </div>
        <div className="result-card">
          <span className="label">Effective area</span>
          <span className="value">{(aEff * 1e4).toFixed(1)} cm²</span>
        </div>
        <div className="result-card">
          <span className="label">λ</span>
          <span className="value">{(lambda * 100).toFixed(2)} cm</span>
        </div>
      </div>

      <Plot
        data={[
          { x: pattern.angles, y: pattern.hPlane, type: 'scatter', mode: 'lines',
            name: 'H-plane (a)', line: { color: '#2196F3', width: 2 } },
          { x: pattern.angles, y: pattern.ePlane, type: 'scatter', mode: 'lines',
            name: 'E-plane (b)', line: { color: '#F44336', width: 2 } },
        ]}
        layout={{
          title: 'Normalized Radiation Pattern',
          xaxis: { title: 'θ (degrees)', range: [-90, 90] },
          yaxis: { title: 'Pattern (dB)', range: [-40, 3] },
          margin: { t: 40, r: 20, b: 50, l: 60 }, height: 380,
          legend: { x: 0.02, y: 0.98 },
        }}
        config={{ responsive: true }} style={{ width: '100%' }}
      />

      <div className="theory">
        <h3>Theory</h3>
        <p><strong>Directivity:</strong> D = 4πA/λ² (uniform illumination)</p>
        <p><strong>Gain:</strong> G = η_ap · D (η_ap = aperture efficiency, typically 0.5–0.7)</p>
        <p><strong>Pattern (sinc):</strong> f(θ) = sin(πa sin θ/λ) / (πa sin θ/λ)</p>
        <p><strong>3-dB beamwidth:</strong> θ₃dB ≈ λ/a (radians)</p>
      </div>
    </div>
  );
}

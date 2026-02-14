import { useState, useMemo } from 'react';
import Plot from 'react-plotly.js';

export function AntennaGainModule() {
  const [antennaType, setAntennaType] = useState<'isotropic' | 'hertzian' | 'halfwave' | 'patch'>('halfwave');
  const [frequency, setFrequency] = useState(2.4e9);

  const c = 2.99792458e8;
  const lambda = c / frequency;

  const patternData = useMemo(() => {
    const n = 361;
    const theta: number[] = [];
    const pattern: number[] = [];
    const patternDb: number[] = [];

    const gains: Record<string, { d: number; name: string }> = {
      isotropic: { d: 1.0, name: 'Isotropic' },
      hertzian: { d: 1.5, name: 'Hertzian Dipole' },
      halfwave: { d: 1.64, name: 'Half-Wave Dipole' },
      patch: { d: 6.0, name: 'Patch Antenna' },
    };

    const info = gains[antennaType];

    for (let i = 0; i < n; i++) {
      const th = (i - 180) * Math.PI / 180;
      theta.push(i - 180);
      let f: number;
      switch (antennaType) {
        case 'isotropic':
          f = 1.0;
          break;
        case 'hertzian':
          f = Math.abs(Math.sin(th));
          break;
        case 'halfwave': {
          const sinTh = Math.sin(th);
          if (Math.abs(sinTh) < 1e-10) {
            f = 0;
          } else {
            f = Math.abs(Math.cos(Math.PI / 2 * Math.cos(th)) / sinTh);
          }
          break;
        }
        case 'patch': {
          const sinTh = Math.sin(th);
          f = Math.pow(Math.abs(Math.cos(th)), 1) * (Math.abs(th) <= Math.PI / 2 ? 1 : 0.1);
          if (Math.abs(sinTh) > 0.01) {
            const arg = Math.PI * 0.5 * lambda * sinTh / lambda;
            f *= Math.abs(Math.sin(arg) / arg);
          }
          break;
        }
        default: f = 1;
      }
      pattern.push(f);
      patternDb.push(f > 1e-10 ? 20 * Math.log10(f) : -60);
    }

    return { theta, pattern, patternDb, info };
  }, [antennaType, lambda]);

  // Polar plot data
  const polarData = useMemo(() => {
    const r = patternData.pattern;
    const theta = patternData.theta;
    return { r, theta };
  }, [patternData]);

  const gains: Record<string, number> = {
    isotropic: 0, hertzian: 1.76, halfwave: 2.15, patch: 7.78,
  };

  return (
    <div className="module">
      <h2>Antenna Gain & Pattern</h2>
      <p>Compare radiation patterns and gains of common antenna types.</p>

      <div className="controls">
        <label>Antenna type:
          <select value={antennaType} onChange={e => setAntennaType(e.target.value as typeof antennaType)}>
            <option value="isotropic">Isotropic (reference)</option>
            <option value="hertzian">Hertzian (short) Dipole</option>
            <option value="halfwave">Half-Wave Dipole</option>
            <option value="patch">Patch Antenna</option>
          </select>
        </label>
        <label>Frequency: <input type="range" min={1e8} max={30e9} step={1e8} value={frequency}
          onChange={e => setFrequency(+e.target.value)} />
          {frequency >= 1e9 ? (frequency / 1e9).toFixed(1) + ' GHz' : (frequency / 1e6).toFixed(0) + ' MHz'}</label>
      </div>

      <div className="results-grid">
        <div className="result-card">
          <span className="label">Antenna</span>
          <span className="value">{patternData.info.name}</span>
        </div>
        <div className="result-card">
          <span className="label">Directivity</span>
          <span className="value">{patternData.info.d.toFixed(2)} ({(10 * Math.log10(patternData.info.d)).toFixed(2)} dBi)</span>
        </div>
        <div className="result-card">
          <span className="label">Gain</span>
          <span className="value">{gains[antennaType].toFixed(2)} dBi</span>
        </div>
        <div className="result-card">
          <span className="label">λ</span>
          <span className="value">{lambda < 0.01 ? (lambda * 1e3).toFixed(1) + ' mm'
            : (lambda * 100).toFixed(1) + ' cm'}</span>
        </div>
        <div className="result-card">
          <span className="label">λ/2 length</span>
          <span className="value">{(lambda * 50).toFixed(1)} cm</span>
        </div>
      </div>

      <Plot
        data={[{
          type: 'scatterpolar',
          r: polarData.r,
          theta: polarData.theta,
          mode: 'lines',
          name: patternData.info.name,
          line: { color: '#2196F3', width: 2 },
        }]}
        layout={{
          title: 'Radiation Pattern (E-plane)',
          polar: { radialaxis: { range: [0, 1.1] } },
          margin: { t: 40, r: 40, b: 40, l: 40 }, height: 450,
          showlegend: false,
        }}
        config={{ responsive: true }} style={{ width: '100%' }}
      />

      <Plot
        data={[{
          x: patternData.theta,
          y: patternData.patternDb,
          type: 'scatter', mode: 'lines',
          name: 'Pattern (dB)',
          line: { color: '#F44336', width: 2 },
        }]}
        layout={{
          title: 'Pattern (Cartesian, dB)',
          xaxis: { title: 'θ (degrees)', range: [-180, 180] },
          yaxis: { title: 'Normalized pattern (dB)', range: [-40, 3] },
          margin: { t: 40, r: 20, b: 50, l: 60 }, height: 300,
        }}
        config={{ responsive: true }} style={{ width: '100%' }}
      />

      <div className="theory">
        <h3>Antenna Parameters</h3>
        <p><strong>Isotropic:</strong> D = 1 (0 dBi), uniform in all directions</p>
        <p><strong>Hertzian dipole:</strong> D = 1.5 (1.76 dBi), pattern: sin²θ</p>
        <p><strong>Half-wave dipole:</strong> D = 1.64 (2.15 dBi), standard reference</p>
        <p><strong>Gain vs Directivity:</strong> G = η_rad · D (radiation efficiency)</p>
      </div>
    </div>
  );
}

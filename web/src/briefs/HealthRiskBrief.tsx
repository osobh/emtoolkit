import { useState } from 'react';

const SOURCES = [
  { name: 'Power lines (60 Hz)', freq: 60, eField: 5000, bField: 0.1e-6, sar: 0, category: 'ELF', ionizing: false },
  { name: 'MRI scanner (static)', freq: 0, eField: 0, bField: 3, sar: 0, category: 'Static', ionizing: false },
  { name: 'Cell phone (900 MHz)', freq: 900e6, eField: 50, bField: 0, sar: 1.0, category: 'RF', ionizing: false },
  { name: 'Cell phone (2.1 GHz)', freq: 2.1e9, eField: 40, bField: 0, sar: 0.8, category: 'RF', ionizing: false },
  { name: 'WiFi router', freq: 2.4e9, eField: 5, bField: 0, sar: 0.01, category: 'RF', ionizing: false },
  { name: '5G mmWave (28 GHz)', freq: 28e9, eField: 10, bField: 0, sar: 0.1, category: 'RF', ionizing: false },
  { name: 'Microwave oven (leak)', freq: 2.45e9, eField: 50, bField: 0, sar: 0, category: 'RF', ionizing: false },
  { name: 'Sunlight (UV)', freq: 1e15, eField: 0, bField: 0, sar: 0, category: 'UV', ionizing: false },
  { name: 'X-ray (medical)', freq: 3e18, eField: 0, bField: 0, sar: 0, category: 'Ionizing', ionizing: true },
];

export function HealthRiskBrief() {
  const [selected, setSelected] = useState(2);
  const [distance, setDistance] = useState(1.0);
  const [duration, setDuration] = useState(60);

  const source = SOURCES[selected];
  const eAtDist = source.eField / (distance * distance);

  const sarLimit = 1.6; // W/kg (FCC, 1g tissue)
  const icnirpLimit = 2.0; // W/kg (ICNIRP, 10g tissue)

  return (
    <div className="module">
      <h2>TB17: Health Risks of EM Fields</h2>
      <p>Understanding exposure limits, SAR, and the science behind electromagnetic field safety standards.</p>

      <div className="controls">
        <label>EM source:
          <select value={selected} onChange={e => setSelected(+e.target.value)}>
            {SOURCES.map((s, i) => <option key={i} value={i}>{s.name}</option>)}
          </select>
        </label>
        <label>Distance (m): <input type="range" min={0.01} max={10} step={0.01} value={distance}
          onChange={e => setDistance(+e.target.value)} /> {distance.toFixed(2)}</label>
        <label>Duration (min): <input type="range" min={1} max={480} step={1} value={duration}
          onChange={e => setDuration(+e.target.value)} /> {duration}</label>
      </div>

      <div className="results-grid">
        <div className="result-card"><span className="label">Category</span>
          <span className="value" style={{ color: source.ionizing ? '#F44336' : '#4CAF50' }}>
            {source.category} ({source.ionizing ? 'IONIZING ⚠️' : 'Non-ionizing ✓'})</span></div>
        <div className="result-card"><span className="label">Frequency</span>
          <span className="value">{source.freq >= 1e15 ? (source.freq / 1e15).toFixed(0) + ' PHz'
            : source.freq >= 1e9 ? (source.freq / 1e9).toFixed(1) + ' GHz'
            : source.freq >= 1e6 ? (source.freq / 1e6).toFixed(0) + ' MHz'
            : source.freq + ' Hz'}</span></div>
        {source.sar > 0 && <>
          <div className="result-card"><span className="label">SAR (at body)</span>
            <span className="value" style={{ color: source.sar > sarLimit ? '#F44336' : '#4CAF50' }}>
              {source.sar.toFixed(2)} W/kg {source.sar > sarLimit ? '⚠️ OVER LIMIT' : '✓ within limits'}</span></div>
          <div className="result-card"><span className="label">FCC limit (1g)</span><span className="value">{sarLimit} W/kg</span></div>
          <div className="result-card"><span className="label">ICNIRP limit (10g)</span><span className="value">{icnirpLimit} W/kg</span></div>
        </>}
        <div className="result-card"><span className="label">Photon energy</span>
          <span className="value">{source.freq > 0 ? (6.626e-34 * source.freq / 1.6e-19).toExponential(2) + ' eV' : 'Static field'}</span></div>
      </div>

      <div style={{ margin: '20px 0', padding: 16, background: source.ionizing ? '#FFEBEE' : '#E8F5E9', borderRadius: 8 }}>
        <h4>Ionizing vs Non-Ionizing Radiation</h4>
        <div style={{ display: 'flex', gap: 12, fontSize: 13, flexWrap: 'wrap' }}>
          <div style={{ flex: 1, minWidth: 200 }}>
            <strong>Non-ionizing (below ~1 PHz):</strong>
            <ul>
              <li>Radio, microwave, infrared, visible</li>
              <li>Cannot break chemical bonds</li>
              <li>Primary effect: heating (thermal)</li>
              <li>SAR limits protect against thermal damage</li>
            </ul>
          </div>
          <div style={{ flex: 1, minWidth: 200 }}>
            <strong>Ionizing (above ~1 PHz):</strong>
            <ul>
              <li>UV-C, X-rays, gamma rays</li>
              <li>Can break DNA bonds → mutations</li>
              <li>Cumulative dose matters (Sieverts)</li>
              <li>ALARA principle: as low as reasonably achievable</li>
            </ul>
          </div>
        </div>
      </div>

      <div className="theory">
        <h3>Key Concepts</h3>
        <p><strong>SAR (Specific Absorption Rate):</strong> Power absorbed per unit mass (W/kg). SAR = σ|E|²/ρ where σ = conductivity, ρ = density.</p>
        <p><strong>Thermal threshold:</strong> ~4 W/kg whole-body SAR raises core temperature by ~1°C. Safety limits include 50× safety factor.</p>
        <p><strong>Frequency matters:</strong> Low frequencies penetrate deeply but couple weakly. Microwaves couple well but are absorbed in cm of tissue. mmWave is absorbed in skin surface.</p>
        <p><strong>Scientific consensus:</strong> No confirmed non-thermal health effects from RF at levels below international exposure limits. Research continues.</p>
      </div>
    </div>
  );
}

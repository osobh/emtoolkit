import { useState } from 'react';

const BANDS = [
  { name: 'LF (125-134 kHz)', freq: 125e3, range: 0.1, coupling: 'Inductive', use: 'Animal tags, access cards', lambda: 2400 },
  { name: 'HF (13.56 MHz)', freq: 13.56e6, range: 1, coupling: 'Inductive', use: 'NFC, smart cards, libraries', lambda: 22.1 },
  { name: 'UHF (860-960 MHz)', freq: 915e6, range: 12, coupling: 'Radiative', use: 'Supply chain, retail, tolls', lambda: 0.328 },
  { name: 'Microwave (2.45 GHz)', freq: 2.45e9, range: 2, coupling: 'Radiative', use: 'Active tags, RTLS', lambda: 0.122 },
];

export function RFIDBrief() {
  const [bandIdx, setBandIdx] = useState(2);
  const [readerPower, setReaderPower] = useState(1.0);
  const [readerGain, setReaderGain] = useState(6);
  const [tagSensitivity, setTagSensitivity] = useState(-18);

  const band = BANDS[bandIdx];
  const c = 3e8;
  const lambda = c / band.freq;

  // Friis for UHF/Microwave
  const eirp = readerPower * Math.pow(10, readerGain / 10);
  const pTag = Math.pow(10, tagSensitivity / 10) * 1e-3; // mW → W
  const readRange = band.coupling === 'Radiative'
    ? lambda / (4 * Math.PI) * Math.sqrt(eirp / pTag)
    : band.range;

  return (
    <div className="module">
      <h2>TB13: RFID Systems</h2>
      <p>Radio-Frequency Identification uses electromagnetic fields to automatically identify and track tags attached to objects.</p>

      <div className="controls">
        <label>Frequency band:
          <select value={bandIdx} onChange={e => setBandIdx(+e.target.value)}>
            {BANDS.map((b, i) => <option key={i} value={i}>{b.name}</option>)}
          </select>
        </label>
        <label>Reader power (W): <input type="range" min={0.01} max={4} step={0.01} value={readerPower}
          onChange={e => setReaderPower(+e.target.value)} /> {readerPower.toFixed(2)}</label>
        <label>Reader antenna gain (dBi): <input type="range" min={0} max={12} step={0.5} value={readerGain}
          onChange={e => setReaderGain(+e.target.value)} /> {readerGain}</label>
        <label>Tag sensitivity (dBm): <input type="range" min={-25} max={-10} step={1} value={tagSensitivity}
          onChange={e => setTagSensitivity(+e.target.value)} /> {tagSensitivity}</label>
      </div>

      <div className="results-grid">
        <div className="result-card"><span className="label">Frequency</span>
          <span className="value">{band.freq >= 1e9 ? (band.freq / 1e9).toFixed(2) + ' GHz' : band.freq >= 1e6 ? (band.freq / 1e6).toFixed(2) + ' MHz' : (band.freq / 1e3).toFixed(0) + ' kHz'}</span></div>
        <div className="result-card"><span className="label">Wavelength</span>
          <span className="value">{lambda >= 1 ? lambda.toFixed(1) + ' m' : (lambda * 100).toFixed(1) + ' cm'}</span></div>
        <div className="result-card"><span className="label">Coupling</span><span className="value">{band.coupling}</span></div>
        <div className="result-card"><span className="label">Read range</span>
          <span className="value">{readRange.toFixed(1)} m</span></div>
        <div className="result-card"><span className="label">EIRP</span>
          <span className="value">{(10 * Math.log10(eirp) + 30).toFixed(1)} dBm</span></div>
        <div className="result-card"><span className="label">Applications</span><span className="value">{band.use}</span></div>
      </div>

      <h3>Frequency Band Comparison</h3>
      <table style={{ width: '100%', borderCollapse: 'collapse', fontSize: 13, marginTop: 8 }}>
        <thead><tr style={{ borderBottom: '2px solid #ddd' }}>
          <th style={{ padding: 6, textAlign: 'left' }}>Band</th>
          <th style={{ padding: 6 }}>Coupling</th>
          <th style={{ padding: 6 }}>Typical range</th>
          <th style={{ padding: 6, textAlign: 'left' }}>Applications</th>
        </tr></thead>
        <tbody>
          {BANDS.map((b, i) => (
            <tr key={i} style={{ background: i === bandIdx ? '#E3F2FD' : i % 2 ? '#f9f9f9' : 'white', fontWeight: i === bandIdx ? 'bold' : 'normal' }}>
              <td style={{ padding: 6 }}>{b.name}</td>
              <td style={{ padding: 6, textAlign: 'center' }}>{b.coupling}</td>
              <td style={{ padding: 6, textAlign: 'center' }}>{b.range < 1 ? (b.range * 100) + ' cm' : b.range + ' m'}</td>
              <td style={{ padding: 6 }}>{b.use}</td>
            </tr>
          ))}
        </tbody>
      </table>

      <div className="theory">
        <h3>RFID Physics</h3>
        <p><strong>Passive tags:</strong> No battery! Harvest energy from reader's RF field. Near-field (inductive coupling) at LF/HF, far-field (backscatter) at UHF.</p>
        <p><strong>Backscatter modulation:</strong> UHF tags modulate their antenna impedance, varying the reflected signal — like a programmable radar cross-section.</p>
        <p><strong>Read range:</strong> R_max = (λ/4π)√(P_reader·G_reader·G_tag·τ/P_threshold) for far-field. Inversely proportional to frequency (√(1/f²)).</p>
        <p><strong>NFC:</strong> Near Field Communication is RFID at 13.56 MHz, standardized for phone payments (ISO 14443, ISO 18092).</p>
      </div>
    </div>
  );
}

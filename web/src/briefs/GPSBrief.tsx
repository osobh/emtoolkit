import { useState, useMemo } from 'react';

export function GPSBrief() {
  const [numSats, setNumSats] = useState(8);
  const [altitude, setAltitude] = useState(20200);
  const [clockError, setClockError] = useState(10);

  const c = 2.99792458e8;
  const orbitPeriod = 2 * Math.PI * Math.sqrt(((6371 + altitude) * 1e3) ** 3 / (3.986e14));
  const posError = c * clockError * 1e-9;
  const gdop = numSats >= 4 ? Math.sqrt(16 / numSats) : Infinity;
  const posAccuracy = posError * gdop;
  const dopplerShift = 3874 * 4.3e3 / c; // max Doppler at L1

  const freqs = [
    { name: 'L1 C/A', freq: 1575.42, use: 'Civilian navigation' },
    { name: 'L1 P(Y)', freq: 1575.42, use: 'Military (encrypted)' },
    { name: 'L2 P(Y)', freq: 1227.60, use: 'Dual-freq ionospheric correction' },
    { name: 'L2C', freq: 1227.60, use: 'Civilian (modernized)' },
    { name: 'L5', freq: 1176.45, use: 'Safety-of-life, high accuracy' },
  ];

  return (
    <div className="module">
      <h2>TB5: Global Positioning System</h2>
      <p>GPS determines position by measuring signal propagation time from multiple satellites.</p>

      <div className="controls">
        <label>Visible satellites: <input type="range" min={4} max={14} step={1} value={numSats}
          onChange={e => setNumSats(+e.target.value)} /> {numSats}</label>
        <label>Orbit altitude (km): <input type="range" min={19000} max={22000} step={100} value={altitude}
          onChange={e => setAltitude(+e.target.value)} /> {altitude}</label>
        <label>Clock error (ns): <input type="range" min={1} max={100} step={1} value={clockError}
          onChange={e => setClockError(+e.target.value)} /> {clockError}</label>
      </div>

      <div className="results-grid">
        <div className="result-card">
          <span className="label">Orbit period</span>
          <span className="value">{(orbitPeriod / 3600).toFixed(2)} hours</span>
        </div>
        <div className="result-card">
          <span className="label">Range error (per ns)</span>
          <span className="value">{(c * 1e-9).toFixed(2)} m/ns = {posError.toFixed(1)} m</span>
        </div>
        <div className="result-card">
          <span className="label">GDOP estimate</span>
          <span className="value">{isFinite(gdop) ? gdop.toFixed(2) : '∞ (need ≥4 sats)'}</span>
        </div>
        <div className="result-card">
          <span className="label">Position accuracy</span>
          <span className="value">~{posAccuracy.toFixed(1)} m</span>
        </div>
        <div className="result-card">
          <span className="label">Signal travel time</span>
          <span className="value">~{((altitude * 1e3) / c * 1e3).toFixed(1)} ms</span>
        </div>
        <div className="result-card">
          <span className="label">Constellation</span>
          <span className="value">31 satellites, 6 orbital planes</span>
        </div>
      </div>

      <h3>GPS Signal Frequencies</h3>
      <table style={{ width: '100%', borderCollapse: 'collapse', fontSize: 13 }}>
        <thead><tr style={{ borderBottom: '2px solid #ddd' }}>
          <th style={{ padding: 6, textAlign: 'left' }}>Signal</th>
          <th style={{ padding: 6 }}>Frequency (MHz)</th>
          <th style={{ padding: 6 }}>λ (cm)</th>
          <th style={{ padding: 6, textAlign: 'left' }}>Use</th>
        </tr></thead>
        <tbody>
          {freqs.map((f, i) => (
            <tr key={i} style={{ background: i % 2 ? '#f9f9f9' : 'white' }}>
              <td style={{ padding: 6 }}>{f.name}</td>
              <td style={{ padding: 6, textAlign: 'center' }}>{f.freq}</td>
              <td style={{ padding: 6, textAlign: 'center' }}>{(c / (f.freq * 1e6) * 100).toFixed(1)}</td>
              <td style={{ padding: 6 }}>{f.use}</td>
            </tr>
          ))}
        </tbody>
      </table>

      <div style={{ margin: '20px 0', padding: 16, background: '#E3F2FD', borderRadius: 8 }}>
        <h4>Trilateration Principle</h4>
        <p style={{ fontSize: 14 }}>Each satellite measurement defines a sphere of possible positions (radius = c × Δt). With 3 satellites → intersection of 3 spheres → 2 points (one usually in space). A 4th satellite resolves receiver clock bias.</p>
        <p style={{ fontSize: 14 }}><strong>Why atomic clocks?</strong> 1 ns clock error = 30 cm position error. Satellites carry cesium/rubidium clocks (stable to ~1 ns/day). Receivers use cheap quartz clocks — the 4th satellite equation solves for the clock offset.</p>
      </div>

      <div className="theory">
        <h3>EM Concepts in GPS</h3>
        <p><strong>Propagation delay:</strong> t = d/c — fundamental measurement principle</p>
        <p><strong>Ionospheric delay:</strong> Plasma in ionosphere slows signals (dispersive). Dual-frequency receivers (L1+L2) measure and correct this.</p>
        <p><strong>Relativistic corrections:</strong> Satellite clocks run 38 μs/day fast (general + special relativity). Without correction, position would drift ~10 km/day!</p>
        <p><strong>Spread spectrum:</strong> GPS uses CDMA — each satellite has a unique PRN code. All transmit on the same frequency but can be distinguished by their codes.</p>
      </div>
    </div>
  );
}

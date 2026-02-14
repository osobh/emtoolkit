import { useState, useMemo } from 'react';

const BANDS = [
  { name: 'ELF', fMin: 3, fMax: 30, unit: 'Hz', color: '#9C27B0', uses: 'Submarine communication' },
  { name: 'SLF', fMin: 30, fMax: 300, unit: 'Hz', color: '#7B1FA2', uses: 'AC power, submarine' },
  { name: 'VLF', fMin: 3, fMax: 30, unit: 'kHz', color: '#512DA8', uses: 'Navigation, time signals' },
  { name: 'LF', fMin: 30, fMax: 300, unit: 'kHz', color: '#303F9F', uses: 'AM radio (longwave), RFID' },
  { name: 'MF', fMin: 300, fMax: 3000, unit: 'kHz', color: '#1976D2', uses: 'AM broadcast, maritime' },
  { name: 'HF', fMin: 3, fMax: 30, unit: 'MHz', color: '#0288D1', uses: 'Shortwave, amateur radio, aviation' },
  { name: 'VHF', fMin: 30, fMax: 300, unit: 'MHz', color: '#0097A7', uses: 'FM radio, TV, air traffic' },
  { name: 'UHF', fMin: 300, fMax: 3000, unit: 'MHz', color: '#00796B', uses: 'TV, cellular, WiFi, GPS, Bluetooth' },
  { name: 'SHF', fMin: 3, fMax: 30, unit: 'GHz', color: '#388E3C', uses: 'Satellite, radar, 5G, WiFi 6E' },
  { name: 'EHF', fMin: 30, fMax: 300, unit: 'GHz', color: '#689F38', uses: 'mmWave 5G, radio astronomy' },
  { name: 'THF', fMin: 300, fMax: 3000, unit: 'GHz', color: '#AFB42B', uses: 'Terahertz imaging, spectroscopy' },
];

const RADAR_BANDS = [
  { name: 'L', fMin: 1e9, fMax: 2e9, uses: 'Air traffic control, weather' },
  { name: 'S', fMin: 2e9, fMax: 4e9, uses: 'Weather radar, ATC' },
  { name: 'C', fMin: 4e9, fMax: 8e9, uses: 'Satellite comm, weather' },
  { name: 'X', fMin: 8e9, fMax: 12e9, uses: 'Marine radar, military' },
  { name: 'Ku', fMin: 12e9, fMax: 18e9, uses: 'Satellite TV, radar' },
  { name: 'K', fMin: 18e9, fMax: 27e9, uses: 'Satellite, radar' },
  { name: 'Ka', fMin: 27e9, fMax: 40e9, uses: 'Satellite, 5G backhaul' },
  { name: 'V', fMin: 40e9, fMax: 75e9, uses: 'mmWave, 5G' },
  { name: 'W', fMin: 75e9, fMax: 110e9, uses: 'Automotive radar, imaging' },
];

export function SpectrumModule() {
  const [freqInput, setFreqInput] = useState(2.4e9);

  const c = 2.99792458e8;

  const info = useMemo(() => {
    const f = freqInput;
    const lambda = c / f;
    const period = 1 / f;
    const omega = 2 * Math.PI * f;
    const k = omega / c;
    const photonEnergy = 6.626e-34 * f;

    let bandName = 'Unknown';
    for (const b of BANDS) {
      const mult = b.unit === 'Hz' ? 1 : b.unit === 'kHz' ? 1e3 : b.unit === 'MHz' ? 1e6 : b.unit === 'GHz' ? 1e9 : 1;
      if (f >= b.fMin * mult && f < b.fMax * mult) { bandName = b.name; break; }
    }

    let radarBand = '';
    for (const rb of RADAR_BANDS) {
      if (f >= rb.fMin && f < rb.fMax) { radarBand = rb.name + '-band'; break; }
    }

    return { lambda, period, omega, k, photonEnergy, bandName, radarBand };
  }, [freqInput]);

  return (
    <div className="module">
      <h2>EM Spectrum Reference</h2>
      <p>Explore the electromagnetic spectrum — frequency bands, wavelengths, and applications.</p>

      <div className="controls">
        <label>Frequency: <input type="range" min={0} max={12} step={0.1}
          value={Math.log10(freqInput)} onChange={e => setFreqInput(10 ** +e.target.value)} />
          {freqInput >= 1e12 ? (freqInput / 1e12).toFixed(2) + ' THz'
            : freqInput >= 1e9 ? (freqInput / 1e9).toFixed(3) + ' GHz'
            : freqInput >= 1e6 ? (freqInput / 1e6).toFixed(3) + ' MHz'
            : freqInput >= 1e3 ? (freqInput / 1e3).toFixed(3) + ' kHz'
            : freqInput.toFixed(1) + ' Hz'}</label>
      </div>

      <div className="results-grid">
        <div className="result-card">
          <span className="label">Band</span>
          <span className="value">{info.bandName} {info.radarBand && `(${info.radarBand})`}</span>
        </div>
        <div className="result-card">
          <span className="label">Wavelength</span>
          <span className="value">{info.lambda >= 1
            ? info.lambda.toFixed(2) + ' m'
            : info.lambda >= 0.01
            ? (info.lambda * 100).toFixed(2) + ' cm'
            : info.lambda >= 1e-3
            ? (info.lambda * 1e3).toFixed(2) + ' mm'
            : (info.lambda * 1e6).toFixed(2) + ' μm'}</span>
        </div>
        <div className="result-card">
          <span className="label">Period</span>
          <span className="value">{info.period >= 1e-3 ? (info.period * 1e3).toFixed(3) + ' ms'
            : info.period >= 1e-6 ? (info.period * 1e6).toFixed(3) + ' μs'
            : info.period >= 1e-9 ? (info.period * 1e9).toFixed(3) + ' ns'
            : (info.period * 1e12).toFixed(3) + ' ps'}</span>
        </div>
        <div className="result-card">
          <span className="label">Photon energy</span>
          <span className="value">{(info.photonEnergy / 1.6e-19 * 1e6).toFixed(4)} μeV</span>
        </div>
        <div className="result-card">
          <span className="label">ω</span>
          <span className="value">{info.omega.toExponential(3)} rad/s</span>
        </div>
        <div className="result-card">
          <span className="label">k</span>
          <span className="value">{info.k.toExponential(3)} rad/m</span>
        </div>
      </div>

      <h3 style={{ marginTop: 24 }}>Frequency Bands</h3>
      <div style={{ overflowX: 'auto' }}>
        <table style={{ width: '100%', borderCollapse: 'collapse', fontSize: 13 }}>
          <thead>
            <tr style={{ borderBottom: '2px solid #ddd' }}>
              <th style={{ padding: 6, textAlign: 'left' }}>Band</th>
              <th style={{ padding: 6 }}>Frequency Range</th>
              <th style={{ padding: 6 }}>Wavelength</th>
              <th style={{ padding: 6, textAlign: 'left' }}>Applications</th>
            </tr>
          </thead>
          <tbody>
            {BANDS.map((b, i) => {
              const mult = b.unit === 'Hz' ? 1 : b.unit === 'kHz' ? 1e3 : b.unit === 'MHz' ? 1e6 : b.unit === 'GHz' ? 1e9 : 1;
              const isActive = freqInput >= b.fMin * mult && freqInput < b.fMax * mult;
              return (
                <tr key={i} style={{ background: isActive ? '#E3F2FD' : i % 2 ? '#f9f9f9' : 'white',
                  fontWeight: isActive ? 'bold' : 'normal' }}>
                  <td style={{ padding: 6 }}>
                    <span style={{ display: 'inline-block', width: 12, height: 12, borderRadius: 2,
                      background: b.color, marginRight: 6, verticalAlign: 'middle' }}></span>
                    {b.name}
                  </td>
                  <td style={{ padding: 6, textAlign: 'center' }}>{b.fMin}–{b.fMax} {b.unit}</td>
                  <td style={{ padding: 6, textAlign: 'center' }}>
                    {(() => {
                      const lMax = c / (b.fMin * mult);
                      const lMin = c / (b.fMax * mult);
                      const fmt = (l: number) => l >= 1 ? l.toFixed(0) + 'm'
                        : l >= 0.01 ? (l * 100).toFixed(0) + 'cm'
                        : l >= 1e-3 ? (l * 1e3).toFixed(1) + 'mm'
                        : (l * 1e6).toFixed(0) + 'μm';
                      return `${fmt(lMin)}–${fmt(lMax)}`;
                    })()}
                  </td>
                  <td style={{ padding: 6 }}>{b.uses}</td>
                </tr>
              );
            })}
          </tbody>
        </table>
      </div>
    </div>
  );
}

import { useState } from 'react';

const LASERS = [
  { name: 'HeNe (red)', lambda: 632.8, medium: 'Helium-Neon gas', power: '0.5-50 mW', use: 'Alignment, barcode scanners' },
  { name: 'Nd:YAG', lambda: 1064, medium: 'Neodymium crystal', power: '1-100 W', use: 'Laser cutting, surgery, LIDAR' },
  { name: 'CO₂', lambda: 10600, medium: 'CO₂ gas', power: '10W-10kW', use: 'Industrial cutting, engraving' },
  { name: 'GaAs diode (IR)', lambda: 850, medium: 'Gallium arsenide', power: '1-500 mW', use: 'Fiber optics, CD players' },
  { name: 'InGaN diode (blue)', lambda: 405, medium: 'Indium gallium nitride', power: '5-100 mW', use: 'Blu-ray, laser projection' },
  { name: 'Excimer (ArF)', lambda: 193, medium: 'Argon fluoride gas', power: '100W pulsed', use: 'LASIK eye surgery, lithography' },
  { name: 'Fiber (Erbium)', lambda: 1550, medium: 'Er-doped fiber', power: '10 mW-10 kW', use: 'Telecom amplifiers, cutting' },
];

export function LaserBrief() {
  const [selected, setSelected] = useState(0);
  const [cavityLength, setCavityLength] = useState(30);

  const laser = LASERS[selected];
  const c = 3e8;
  const freq = c / (laser.lambda * 1e-9);
  const photonEnergy = 6.626e-34 * freq;
  const photonEnergyEv = photonEnergy / 1.6e-19;
  const fsr = c / (2 * cavityLength * 1e-2); // Free spectral range
  const modesInBandwidth = Math.max(1, Math.round(1e9 / fsr)); // ~1 GHz gain bandwidth assumed

  return (
    <div className="module">
      <h2>TB15: Lasers</h2>
      <p>Lasers produce coherent, monochromatic, highly directional light through stimulated emission of radiation.</p>

      <div className="controls">
        <label>Laser type:
          <select value={selected} onChange={e => setSelected(+e.target.value)}>
            {LASERS.map((l, i) => <option key={i} value={i}>{l.name} ({l.lambda} nm)</option>)}
          </select>
        </label>
        <label>Cavity length (cm): <input type="range" min={1} max={100} step={1} value={cavityLength}
          onChange={e => setCavityLength(+e.target.value)} /> {cavityLength}</label>
      </div>

      <div className="results-grid">
        <div className="result-card"><span className="label">Wavelength</span><span className="value">{laser.lambda} nm</span></div>
        <div className="result-card"><span className="label">Frequency</span><span className="value">{(freq / 1e12).toFixed(1)} THz</span></div>
        <div className="result-card"><span className="label">Photon energy</span><span className="value">{photonEnergyEv.toFixed(3)} eV</span></div>
        <div className="result-card"><span className="label">Gain medium</span><span className="value">{laser.medium}</span></div>
        <div className="result-card"><span className="label">Typical power</span><span className="value">{laser.power}</span></div>
        <div className="result-card"><span className="label">Free spectral range</span><span className="value">{(fsr / 1e6).toFixed(0)} MHz</span></div>
        <div className="result-card"><span className="label">Cavity modes (est.)</span><span className="value">~{modesInBandwidth}</span></div>
        <div className="result-card"><span className="label">EM spectrum region</span>
          <span className="value">{laser.lambda < 400 ? 'UV' : laser.lambda < 700 ? 'Visible' : laser.lambda < 1400 ? 'Near IR' : 'Mid/Far IR'}</span></div>
      </div>

      <div style={{ display: 'flex', gap: 16, margin: '20px 0', alignItems: 'center', flexWrap: 'wrap' }}>
        <div style={{ flex: 1, minWidth: 200, padding: 16, background: '#FFEBEE', borderRadius: 8 }}>
          <h4>1. Absorption</h4>
          <p style={{ fontSize: 13 }}>Photon is absorbed, exciting atom to higher energy level.</p>
        </div>
        <div style={{ flex: 1, minWidth: 200, padding: 16, background: '#FFF3E0', borderRadius: 8 }}>
          <h4>2. Spontaneous Emission</h4>
          <p style={{ fontSize: 13 }}>Excited atom randomly emits photon — random direction and phase.</p>
        </div>
        <div style={{ flex: 1, minWidth: 200, padding: 16, background: '#E8F5E9', borderRadius: 8 }}>
          <h4>3. Stimulated Emission ⭐</h4>
          <p style={{ fontSize: 13 }}>Incoming photon triggers emission of identical photon — same λ, direction, phase. This is amplification!</p>
        </div>
      </div>

      <div className="theory">
        <h3>LASER = Light Amplification by Stimulated Emission of Radiation</h3>
        <p><strong>Population inversion:</strong> More atoms in excited state than ground state (requires external pumping). Necessary for net gain.</p>
        <p><strong>Optical cavity:</strong> Two mirrors create a resonator. Light bounces back and forth, being amplified each pass. One mirror is partially transparent (output coupler).</p>
        <p><strong>Cavity modes:</strong> Standing wave condition: L = mλ/2. Mode spacing (FSR) = c/2L.</p>
        <p><strong>Coherence:</strong> Spatial coherence (uniform wavefront) + temporal coherence (long coherence length) make lasers unique among light sources.</p>
      </div>
    </div>
  );
}

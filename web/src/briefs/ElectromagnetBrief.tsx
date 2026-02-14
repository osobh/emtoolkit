import { useState } from 'react';

export function ElectromagnetBrief() {
  const [turns, setTurns] = useState(500);
  const [current, setCurrent] = useState(2.0);
  const [length, setLength] = useState(0.1);
  const [muR, setMuR] = useState(1000);
  const [area, setArea] = useState(1.0);

  const mu0 = 4e-7 * Math.PI;
  const bField = mu0 * muR * turns * current / length;
  const hField = turns * current / length;
  const flux = bField * area * 1e-4;
  const inductance = mu0 * muR * turns * turns * area * 1e-4 / length;
  const energy = 0.5 * inductance * current * current;
  const force = bField * bField * area * 1e-4 / (2 * mu0); // Maxwell stress

  const cores = [
    { name: 'Air', muR: 1 },
    { name: 'Ferrite', muR: 2000 },
    { name: 'Silicon steel', muR: 4000 },
    { name: 'Iron (pure)', muR: 5000 },
    { name: 'Mu-metal', muR: 50000 },
  ];

  return (
    <div className="module">
      <h2>TB10: Electromagnets</h2>
      <p>Electromagnets create controllable magnetic fields by passing current through a coil, often with a ferromagnetic core to amplify the field.</p>

      <div className="controls">
        <label>Turns: <input type="range" min={10} max={5000} step={10} value={turns}
          onChange={e => setTurns(+e.target.value)} /> {turns}</label>
        <label>Current (A): <input type="range" min={0.1} max={20} step={0.1} value={current}
          onChange={e => setCurrent(+e.target.value)} /> {current.toFixed(1)}</label>
        <label>Length (cm): <input type="range" min={1} max={50} step={1} value={length * 100}
          onChange={e => setLength(+e.target.value / 100)} /> {(length * 100).toFixed(0)}</label>
        <label>Core material:
          <select value={muR} onChange={e => setMuR(+e.target.value)}>
            {cores.map(c => <option key={c.muR} value={c.muR}>{c.name} (μᵣ={c.muR.toLocaleString()})</option>)}
          </select>
        </label>
        <label>Cross-section (cm²): <input type="range" min={0.1} max={10} step={0.1} value={area}
          onChange={e => setArea(+e.target.value)} /> {area.toFixed(1)}</label>
      </div>

      <div className="results-grid">
        <div className="result-card"><span className="label">B field</span>
          <span className="value">{bField >= 1 ? bField.toFixed(2) + ' T' : (bField * 1e3).toFixed(2) + ' mT'}</span></div>
        <div className="result-card"><span className="label">H field</span><span className="value">{hField.toFixed(0)} A/m</span></div>
        <div className="result-card"><span className="label">Magnetic flux Φ</span>
          <span className="value">{flux >= 1e-3 ? (flux * 1e3).toFixed(2) + ' mWb' : (flux * 1e6).toFixed(2) + ' μWb'}</span></div>
        <div className="result-card"><span className="label">Inductance</span>
          <span className="value">{inductance >= 1 ? inductance.toFixed(2) + ' H' : inductance >= 1e-3 ? (inductance * 1e3).toFixed(2) + ' mH' : (inductance * 1e6).toFixed(1) + ' μH'}</span></div>
        <div className="result-card"><span className="label">Stored energy</span>
          <span className="value">{energy >= 1 ? energy.toFixed(2) + ' J' : (energy * 1e3).toFixed(2) + ' mJ'}</span></div>
        <div className="result-card"><span className="label">Pull force (Maxwell)</span>
          <span className="value">{force >= 1 ? force.toFixed(1) + ' N' : (force * 1e3).toFixed(1) + ' mN'} ({(force / 9.81).toFixed(2)} kgf)</span></div>
      </div>

      <div className="theory">
        <h3>Key Equations</h3>
        <p><strong>Solenoid field:</strong> B = μ₀μᵣ NI/ℓ</p>
        <p><strong>Inductance:</strong> L = μ₀μᵣ N²A/ℓ</p>
        <p><strong>Pull force:</strong> F = B²A/(2μ₀) (Maxwell stress tensor)</p>
        <p><strong>Applications:</strong> MRI magnets (superconducting, ~3T), junkyard cranes, maglev trains, solenoid valves, relays, particle accelerators.</p>
        <p><strong>Limitations:</strong> Core saturation (B_sat ≈ 1.5-2T for iron), hysteresis losses, eddy current heating.</p>
      </div>
    </div>
  );
}

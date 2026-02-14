import { useState } from 'react';

export function EMFSensorBrief() {
  const [bField, setBField] = useState(0.5);
  const [current, setCurrent] = useState(100);
  const [thickness, setThickness] = useState(0.5);
  const [n, setN] = useState(1e25);
  const [sensorType, setSensorType] = useState<'hall' | 'current'>('hall');

  const q = 1.6e-19;
  const vHall = current * bField / (n * q * thickness * 1e-3);
  const rHall = 1 / (n * q);

  // Current sensor (Hall-effect clamp)
  const wireCurrent = current;
  const mu0 = 4e-7 * Math.PI;
  const clampRadius = 0.015;
  const bFromWire = mu0 * wireCurrent / (2 * Math.PI * clampRadius);

  return (
    <div className="module">
      <h2>TB12: EMF Sensors (Hall Effect)</h2>
      <p>Hall effect sensors detect magnetic fields by measuring the transverse voltage produced when current flows through a conductor in a magnetic field.</p>

      <div className="controls">
        <label>Mode:
          <select value={sensorType} onChange={e => setSensorType(e.target.value as 'hall' | 'current')}>
            <option value="hall">Hall voltage calculator</option>
            <option value="current">Current sensor (clamp)</option>
          </select>
        </label>
        <label>B field (T): <input type="range" min={0.001} max={2} step={0.001} value={bField}
          onChange={e => setBField(+e.target.value)} /> {bField >= 0.01 ? bField.toFixed(3) + ' T' : (bField * 1e3).toFixed(1) + ' mT'}</label>
        <label>Current (A): <input type="range" min={0.001} max={500} step={0.1} value={current}
          onChange={e => setCurrent(+e.target.value)} /> {current}</label>
        <label>Sensor thickness (mm): <input type="range" min={0.1} max={5} step={0.1} value={thickness}
          onChange={e => setThickness(+e.target.value)} /> {thickness.toFixed(1)}</label>
      </div>

      <div className="results-grid">
        {sensorType === 'hall' ? <>
          <div className="result-card"><span className="label">Hall voltage</span>
            <span className="value">{vHall >= 1e-3 ? (vHall * 1e3).toFixed(3) + ' mV' : (vHall * 1e6).toFixed(3) + ' μV'}</span></div>
          <div className="result-card"><span className="label">Hall coefficient R_H</span>
            <span className="value">{rHall.toExponential(3)} m³/C</span></div>
          <div className="result-card"><span className="label">Carrier density</span>
            <span className="value">{n.toExponential(2)} /m³</span></div>
        </> : <>
          <div className="result-card"><span className="label">Wire current</span><span className="value">{wireCurrent} A</span></div>
          <div className="result-card"><span className="label">B at sensor (clamp)</span>
            <span className="value">{(bFromWire * 1e3).toFixed(3)} mT</span></div>
          <div className="result-card"><span className="label">Clamp radius</span><span className="value">{(clampRadius * 1e3).toFixed(0)} mm</span></div>
        </>}
      </div>

      <div style={{ margin: '20px 0', padding: 16, background: '#E8F5E9', borderRadius: 8 }}>
        <h4>Hall Effect Principle</h4>
        <p style={{ fontSize: 14 }}>When current I flows through a thin conductor in magnetic field B (perpendicular), the Lorentz force F = qv × B deflects charge carriers to one side, building up a transverse voltage:</p>
        <p style={{ fontSize: 16, fontFamily: 'serif', textAlign: 'center' }}>V_H = IB / (nqd)</p>
        <p style={{ fontSize: 14 }}>where n = carrier density, q = charge, d = thickness.</p>
      </div>

      <div className="theory">
        <h3>Applications</h3>
        <p><strong>Current sensors:</strong> Hall-effect clamp meters measure B around a wire (B = μ₀I/2πr) without breaking the circuit.</p>
        <p><strong>Position sensors:</strong> Brushless DC motors use Hall sensors to detect rotor magnet position for commutation.</p>
        <p><strong>Gaussmeters:</strong> Direct B-field measurement instruments.</p>
        <p><strong>Proximity switches:</strong> Detect presence of magnets — used in door sensors, speedometers, flow meters.</p>
      </div>
    </div>
  );
}

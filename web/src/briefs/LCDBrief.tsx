import { useState } from 'react';

export function LCDBrief() {
  const [appliedVoltage, setAppliedVoltage] = useState(0);
  const [twistAngle, setTwistAngle] = useState(90);
  const [polarizerAngle, setPolarizerAngle] = useState(0);

  const threshold = 1.5;
  const transmittance = appliedVoltage < threshold
    ? Math.cos(twistAngle * Math.PI / 180) ** 2
    : appliedVoltage > 4.0 ? 0.01 : Math.cos(twistAngle * Math.PI / 180 * (1 - (appliedVoltage - threshold) / 2.5)) ** 2;

  const brightness = Math.max(0, Math.min(100, transmittance * 100));

  return (
    <div className="module">
      <h2>TB14: Liquid Crystal Display</h2>
      <p>LCDs control light transmission by electrically rotating the polarization of light passing through liquid crystal molecules.</p>

      <div className="controls">
        <label>Applied voltage (V): <input type="range" min={0} max={5} step={0.1} value={appliedVoltage}
          onChange={e => setAppliedVoltage(+e.target.value)} /> {appliedVoltage.toFixed(1)}</label>
        <label>LC twist angle (°): <input type="range" min={0} max={270} step={10} value={twistAngle}
          onChange={e => setTwistAngle(+e.target.value)} /> {twistAngle}</label>
      </div>

      <div style={{ display: 'flex', gap: 20, alignItems: 'center', margin: '20px 0' }}>
        <div style={{ width: 120, height: 120, borderRadius: 8, border: '3px solid #333',
          background: `rgba(200,200,200,${brightness / 100})`,
          display: 'flex', alignItems: 'center', justifyContent: 'center',
          fontSize: 24, color: brightness > 50 ? '#333' : '#ccc' }}>
          {brightness > 50 ? '💡' : '⬛'}
        </div>
        <div>
          <div style={{ fontSize: 14 }}>
            <strong>State:</strong> {appliedVoltage < threshold ? 'OFF (light passes)' : 'ON (light blocked)'}
          </div>
          <div style={{ fontSize: 14 }}>
            <strong>Transmittance:</strong> {brightness.toFixed(1)}%
          </div>
          <div style={{ fontSize: 14 }}>
            <strong>Threshold:</strong> {threshold.toFixed(1)} V
          </div>
        </div>
      </div>

      <div style={{ margin: '20px 0', padding: 16, background: '#FFF3E0', borderRadius: 8 }}>
        <h4>LCD Layer Structure</h4>
        <div style={{ fontFamily: 'monospace', fontSize: 13, lineHeight: 1.8, textAlign: 'center' }}>
          <div>☀️ Backlight</div>
          <div style={{ background: '#E3F2FD', padding: 4, margin: 2 }}>→ Polarizer (horizontal)</div>
          <div style={{ background: '#E8F5E9', padding: 4, margin: 2 }}>→ Glass + ITO electrode</div>
          <div style={{ background: '#FCE4EC', padding: 8, margin: 2 }}>
            → Liquid Crystal Layer<br/>
            {appliedVoltage < threshold ? '(twisted → rotates polarization 90°)' : '(aligned → no rotation)'}
          </div>
          <div style={{ background: '#E8F5E9', padding: 4, margin: 2 }}>→ Glass + ITO electrode</div>
          <div style={{ background: '#E3F2FD', padding: 4, margin: 2 }}>→ Polarizer (vertical)</div>
          <div>👁️ Viewer</div>
        </div>
      </div>

      <div className="theory">
        <h3>LCD Physics</h3>
        <p><strong>Twisted Nematic (TN):</strong> LC molecules spiral 90° between plates. Incoming polarized light follows the twist, rotating 90°, and passes through the crossed analyzer. Applying voltage aligns molecules with the field, eliminating the twist — light is blocked.</p>
        <p><strong>Polarization:</strong> Malus's law: I = I₀ cos²θ where θ is angle between polarizer and light polarization.</p>
        <p><strong>Birefringence:</strong> LC molecules are birefringent (n_e ≠ n_o). The extraordinary axis follows the director orientation, enabling waveguiding of polarization.</p>
        <p><strong>ITO electrodes:</strong> Indium tin oxide — transparent and conductive. Allows applying E-field across the thin LC layer (~5μm).</p>
        <p><strong>Response time:</strong> ~5-25 ms (TN). IPS panels are slower but have better viewing angles.</p>
      </div>
    </div>
  );
}

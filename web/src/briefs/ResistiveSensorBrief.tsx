import { useState, useMemo } from 'react';

export function ResistiveSensorBrief() {
  const [gf, setGf] = useState(2.0);
  const [strain, setStrain] = useState(500);
  const [r0, setR0] = useState(350);
  const [vex, setVex] = useState(5.0);

  const deltaR = r0 * gf * strain * 1e-6;
  const rStrained = r0 + deltaR;

  // Quarter bridge
  const vOut = vex * deltaR / (4 * r0 + 2 * deltaR);
  const vOutMv = vOut * 1000;

  // Sensitivity
  const sensitivity = vex * gf / 4; // mV/V per 1000 με

  return (
    <div className="module">
      <h2>TB7: Resistive Sensors</h2>
      <p>Strain gauges and Wheatstone bridges convert mechanical deformation into measurable electrical signals using the piezoresistive effect.</p>

      <div className="controls">
        <label>Gauge Factor (GF): <input type="range" min={1} max={200} step={0.5} value={gf}
          onChange={e => setGf(+e.target.value)} /> {gf}</label>
        <label>Strain (με): <input type="range" min={0} max={5000} step={10} value={strain}
          onChange={e => setStrain(+e.target.value)} /> {strain}</label>
        <label>Nominal R (Ω): <input type="range" min={100} max={1000} step={10} value={r0}
          onChange={e => setR0(+e.target.value)} /> {r0}</label>
        <label>Excitation (V): <input type="range" min={1} max={15} step={0.5} value={vex}
          onChange={e => setVex(+e.target.value)} /> {vex}</label>
      </div>

      <div className="results-grid">
        <div className="result-card"><span className="label">ΔR/R</span><span className="value">{(gf * strain * 1e-6 * 100).toFixed(4)}%</span></div>
        <div className="result-card"><span className="label">ΔR</span><span className="value">{deltaR.toFixed(4)} Ω</span></div>
        <div className="result-card"><span className="label">R (strained)</span><span className="value">{rStrained.toFixed(2)} Ω</span></div>
        <div className="result-card"><span className="label">Bridge output (¼ bridge)</span><span className="value">{vOutMv.toFixed(3)} mV</span></div>
        <div className="result-card"><span className="label">Sensitivity</span><span className="value">{sensitivity.toFixed(2)} mV/V/1000με</span></div>
      </div>

      <div style={{ margin: '20px 0', padding: 16, background: '#E8F5E9', borderRadius: 8 }}>
        <h4>Wheatstone Bridge</h4>
        <pre style={{ fontFamily: 'monospace', fontSize: 14, textAlign: 'center' }}>{`
    Vex (+)
     │
  ┌──┤──┐
  R1  R2(gauge)
  ├──┤──┤ → V_out
  R3  R4
  └──┤──┘
     │
    GND
        `}</pre>
        <p style={{ fontSize: 14 }}>V_out = V_ex × (R2·R3 − R1·R4) / ((R1+R2)(R3+R4))</p>
        <p style={{ fontSize: 14 }}>When balanced (all R equal): V_out = 0. Strain on R2 unbalances the bridge.</p>
      </div>

      <div className="theory">
        <h3>Gauge Factor</h3>
        <p><strong>Definition:</strong> GF = (ΔR/R) / ε where ε = strain</p>
        <p><strong>Metal foil gauges:</strong> GF ≈ 2 (due to geometric change + slight resistivity change)</p>
        <p><strong>Semiconductor gauges:</strong> GF ≈ 50-200 (piezoresistive effect dominates)</p>
        <p><strong>ΔR/R = GF × ε</strong> — incredibly small changes (0.1% typical) require precise bridge circuits and amplification.</p>
      </div>
    </div>
  );
}

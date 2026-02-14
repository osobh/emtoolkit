import { useState, useMemo } from 'react';
import * as wasm from '../lib/em_wasm';

interface PowerResult {
  poynting_avg: number;
  power_delivered: number;
  power_reflected: number;
  return_loss_db: number;
  mismatch_loss_db: number;
  gamma_mag: number;
}

interface UnitResult {
  watts: number;
  dbm: number;
  dbw: number;
  mw: number;
  uw: number;
}

export function PowerModule() {
  const [e0, setE0] = useState(10.0);
  const [eta, setEta] = useState(377.0);
  const [gammaMag, setGammaMag] = useState(0.2);
  const [powerVal, setPowerVal] = useState(0);
  const [powerUnit, setPowerUnit] = useState<'W' | 'dBm' | 'dBW'>('dBm');

  const power: PowerResult | null = useMemo(() => {
    try {
      return wasm.power_calculations(e0, eta, gammaMag) as PowerResult;
    } catch { return null; }
  }, [e0, eta, gammaMag]);

  const units: UnitResult | null = useMemo(() => {
    try {
      return wasm.unit_conversions(powerVal, powerUnit) as UnitResult;
    } catch { return null; }
  }, [powerVal, powerUnit]);

  return (
    <div className="module">
      <h2>Power & Poynting Vector</h2>
      <p>Calculate time-average power flow, mismatch losses, and convert between power units.</p>

      <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: 24 }}>
        <div>
          <h3>Poynting Vector & Mismatch</h3>
          <div className="controls">
            <label>E₀ (V/m): <input type="range" min={0.1} max={100} step={0.1} value={e0}
              onChange={e => setE0(+e.target.value)} /> {e0.toFixed(1)}</label>
            <label>η (Ω): <input type="range" min={1} max={1000} step={1} value={eta}
              onChange={e => setEta(+e.target.value)} /> {eta.toFixed(0)}</label>
            <label>|Γ|: <input type="range" min={0} max={0.99} step={0.01} value={gammaMag}
              onChange={e => setGammaMag(+e.target.value)} /> {gammaMag.toFixed(2)}</label>
          </div>

          {power && (
            <div className="results-grid">
              <div className="result-card">
                <span className="label">S_avg</span>
                <span className="value">{power.poynting_avg < 1
                  ? (power.poynting_avg * 1e3).toFixed(3) + ' mW/m²'
                  : power.poynting_avg.toFixed(3) + ' W/m²'}</span>
              </div>
              <div className="result-card">
                <span className="label">Power delivered</span>
                <span className="value">{(power.power_delivered / power.poynting_avg * 100).toFixed(1)}%</span>
              </div>
              <div className="result-card">
                <span className="label">Power reflected</span>
                <span className="value">{(power.power_reflected / power.poynting_avg * 100).toFixed(1)}%</span>
              </div>
              <div className="result-card">
                <span className="label">Return Loss</span>
                <span className="value">{isFinite(power.return_loss_db)
                  ? power.return_loss_db.toFixed(2) + ' dB' : '∞ dB'}</span>
              </div>
              <div className="result-card">
                <span className="label">Mismatch Loss</span>
                <span className="value">{power.mismatch_loss_db.toFixed(3)} dB</span>
              </div>
              <div className="result-card">
                <span className="label">VSWR</span>
                <span className="value">{((1 + gammaMag) / (1 - gammaMag)).toFixed(2)}</span>
              </div>
            </div>
          )}
        </div>

        <div>
          <h3>Power Unit Converter</h3>
          <div className="controls">
            <label>
              Value: <input type="number" value={powerVal} step={0.1}
                onChange={e => setPowerVal(+e.target.value)} style={{ width: 100 }} />
            </label>
            <label>
              Unit:
              <select value={powerUnit} onChange={e => setPowerUnit(e.target.value as 'W' | 'dBm' | 'dBW')}>
                <option value="dBm">dBm</option>
                <option value="dBW">dBW</option>
                <option value="W">Watts</option>
              </select>
            </label>
          </div>

          {units && (
            <div className="results-grid">
              <div className="result-card">
                <span className="label">Watts</span>
                <span className="value">{units.watts.toExponential(4)}</span>
              </div>
              <div className="result-card">
                <span className="label">dBm</span>
                <span className="value">{units.dbm.toFixed(2)}</span>
              </div>
              <div className="result-card">
                <span className="label">dBW</span>
                <span className="value">{units.dbw.toFixed(2)}</span>
              </div>
              <div className="result-card">
                <span className="label">milliwatts</span>
                <span className="value">{units.mw.toExponential(4)}</span>
              </div>
              <div className="result-card">
                <span className="label">microwatts</span>
                <span className="value">{units.uw.toExponential(4)}</span>
              </div>
            </div>
          )}

          <div style={{ marginTop: 16, padding: 12, background: '#f5f5f5', borderRadius: 8, fontSize: 13 }}>
            <p style={{ margin: 0 }}><strong>Quick reference:</strong></p>
            <p style={{ margin: '4px 0' }}>0 dBm = 1 mW = −30 dBW</p>
            <p style={{ margin: '4px 0' }}>30 dBm = 1 W = 0 dBW</p>
            <p style={{ margin: '4px 0' }}>−10 dBm = 100 μW</p>
            <p style={{ margin: '4px 0' }}>20 dBm = 100 mW</p>
          </div>
        </div>
      </div>

      <div className="theory">
        <h3>Theory</h3>
        <p><strong>Poynting vector:</strong> S = E × H, ⟨S⟩ = |E₀|²/(2η)</p>
        <p><strong>Power delivered:</strong> P_L = P_inc(1 − |Γ|²)</p>
        <p><strong>Return loss:</strong> RL = −20 log₁₀|Γ| (higher is better)</p>
        <p><strong>Mismatch loss:</strong> ML = −10 log₁₀(1 − |Γ|²)</p>
      </div>
    </div>
  );
}
